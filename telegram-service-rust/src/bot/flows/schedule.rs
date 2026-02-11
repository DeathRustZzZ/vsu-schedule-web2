use chrono::{Datelike, FixedOffset, Utc};
use log::{debug, info, warn};
use teloxide::prelude::*;
use teloxide::types::MessageId;

use crate::bot::keyboards;
use crate::bot::schedule_api::{GroupWithSubgroupsIds, LessonResponse, ScheduleApi, TeacherResponse};
use crate::bot::ui::render_screen;
use crate::db::facade::DbFacade;

/// Показ расписания через внешний Schedule API.
///
/// Здесь самые важные зоны для логов:
/// - входные параметры (факультет/группа/weekday)
/// - этапы: загрузили профиль → загрузили список групп → подобрали id → загрузили расписание
/// - ошибки API (они не фатальные, но должны быть видимыми)
pub async fn show_schedule(
    bot: &Bot,
    db: &DbFacade,
    schedule_api: &ScheduleApi,
    schedule_tz_offset_seconds: i32,
    telegram_id: i64,
    chat_id: ChatId,
    message_id: Option<MessageId>,
) -> Result<(), teloxide::RequestError> {
    info!("show_schedule requested: user={} chat_id={}", telegram_id, chat_id.0);

    let student = match db.find_student(telegram_id).await {
        Ok(Some(student)) => student,
        Ok(None) => {
            info!("user {} requested schedule but not registered", telegram_id);
            return render_screen(
                bot,
                db,
                telegram_id,
                chat_id,
                message_id,
                "Сначала зарегистрируйся, чтобы смотреть расписание.",
                Some(keyboards::back_menu()),
            )
            .await;
        }
        Err(err) => {
            warn!("failed to load student profile for user {}: {:?}", telegram_id, err);
            return render_screen(
                bot,
                db,
                telegram_id,
                chat_id,
                message_id,
                "❌ Не удалось загрузить профиль.",
                Some(keyboards::back_menu()),
            )
            .await;
        }
    };

    // Приводим факультет из профиля к ожидаемому формату внешнего API.
    // Это "тонкое место": разные источники могут писать "ФМиИТ" по-разному.
    let faculty = normalize_faculty(&student.faculty);
    debug!(
        "faculty normalization: raw='{}' normalized={:?}",
        student.faculty, faculty
    );

    // Пока поддерживаем только ФМиИТ — явно сообщаем пользователю и логируем.
    if faculty.as_deref() != Some("ФМиИТ") {
        info!(
            "schedule requested for unsupported faculty: raw='{}' user={}",
            student.faculty, telegram_id
        );
        return render_screen(
            bot,
            db,
            telegram_id,
            chat_id,
            message_id,
            "Пока поддерживается только факультет ФМиИТ.",
            Some(keyboards::back_menu()),
        )
        .await;
    }

    // Текущий день недели: используется как параметр API.
    // Логируем, чтобы понимать, что именно запрашивали у API.
    let weekday = current_weekday_ru(schedule_tz_offset_seconds);
    debug!(
        "schedule request context: user={} faculty={:?} group_name='{}' subgroup='{}' weekday='{}'",
        telegram_id,
        faculty,
        student.group_name,
        student.subgroup_name.as_deref().unwrap_or(""),
        weekday
    );

    // 1) Получаем список доступных групп.
    // Это нужно, чтобы подобрать (group_id, subgroup_id) под student.group_name.
    let available = match schedule_api
        .get_available_groups(faculty.as_deref().unwrap_or(&student.faculty))
        .await
    {
        Ok(list) => {
            debug!("available groups loaded: count={}", list.list.len());
            list.list
        }
        Err(err) => {
            warn!("Failed to load groups list: {:?}", err);
            return render_screen(
                bot,
                db,
                telegram_id,
                chat_id,
                message_id,
                "❌ Не удалось получить список групп.",
                Some(keyboards::back_menu()),
            )
            .await;
        }
    };

    // 2) Подбираем group_id / subgroup_id из списка.
    let (group_id, subgroup_id) = match pick_group_and_subgroup_with_preference(
        &available,
        &student.group_name,
        student.subgroup_name.as_deref(),
    ) {
        Some(pair) => {
            debug!(
                "picked group/subgroup: group_id='{}' subgroup_id='{}' for user={}",
                pair.0, pair.1, telegram_id
            );
            pair
        }
        None => {
            warn!(
                "failed to pick group/subgroup: group_name='{}' subgroup='{}' available_count={}",
                student.group_name,
                student.subgroup_name.as_deref().unwrap_or(""),
                available.len()
            );
            return render_screen(
                bot,
                db,
                telegram_id,
                chat_id,
                message_id,
                "❌ Не удалось подобрать группу. Обратись к администратору.",
                Some(keyboards::back_menu()),
            )
            .await;
        }
    };

    // 3) Загружаем расписание.
    let schedule = match schedule_api
        .get_schedule(
            faculty.as_deref().unwrap_or(&student.faculty),
            &group_id,
            &subgroup_id,
            weekday,
        )
        .await
    {
        Ok(schedule) => {
            debug!("schedule loaded: lessons_count={}", schedule.lessons.len());
            schedule
        }
        Err(err) => {
            warn!("Failed to load schedule: {:?}", err);
            return render_screen(
                bot,
                db,
                telegram_id,
                chat_id,
                message_id,
                "❌ Не удалось получить расписание.",
                Some(keyboards::back_menu()),
            )
            .await;
        }
    };

    // 4) Формируем сообщение пользователю.
    // На этом этапе часто возникают "визуальные" баги, поэтому полезно логировать
    // количество занятий и выбранные id.
    debug!(
        "formatting schedule message: group_id='{}' subgroup_id='{}' lessons_count={}",
        group_id,
        subgroup_id,
        schedule.lessons.len()
    );

    let text = format_schedule_message(&group_id, &subgroup_id, schedule.lessons, weekday);

    render_screen(
        bot,
        db,
        telegram_id,
        chat_id,
        message_id,
        &text,
        Some(keyboards::back_menu()),
    )
    .await
}

/// Подбор пары (group_id, subgroup_id) на основании:
/// - списка доступных групп от внешнего API
/// - `group_name`, который хранится у студента
///
/// Важный момент:
/// подбор идёт по `contains` на lower-case строках → это эвристика.
/// Она удобна, но при похожих названиях групп может дать "не ту" группу.
/// Поэтому в show_schedule выше мы логируем результат выбора.
fn pick_group_and_subgroup_with_preference(
    available: &[GroupWithSubgroupsIds],
    group_name: &str,
    subgroup_name: Option<&str>,
) -> Option<(String, String)> {
    if available.is_empty() {
        // warn не ставим — это может быть валидный ответ API (например, факультет без групп).
        return None;
    }

    let needle = normalize_group_key(group_name);
    let subgroup_needle = subgroup_name.map(normalize_group_key);

    let exact_matches: Vec<&GroupWithSubgroupsIds> = available
        .iter()
        .filter(|g| normalize_group_key(&g.group_id) == needle)
        .collect();
    if exact_matches.len() == 1 {
        let selected = exact_matches[0].clone();
        let subgroup = pick_subgroup_with_preference(&selected, &needle, subgroup_needle.as_deref())?;
        return Some((selected.group_id.clone(), subgroup));
    }
    if exact_matches.len() > 1 {
        return None;
    }

    let contains_matches: Vec<&GroupWithSubgroupsIds> = available
        .iter()
        .filter(|g| normalize_group_key(&g.group_id).contains(&needle))
        .collect();
    if contains_matches.len() == 1 {
        let selected = contains_matches[0].clone();
        let subgroup = pick_subgroup_with_preference(&selected, &needle, subgroup_needle.as_deref())?;
        return Some((selected.group_id.clone(), subgroup));
    }

    if let Some(subgroup_needle) = subgroup_needle.as_deref() {
        let mut subgroup_hit: Option<(String, String)> = None;
        for group in available {
            for subgroup in &group.subgroup_ids {
                if normalize_group_key(subgroup) == subgroup_needle {
                    if subgroup_hit.is_some() {
                        return None;
                    }
                    subgroup_hit = Some((group.group_id.clone(), subgroup.clone()));
                }
            }
        }
        if let Some(pair) = subgroup_hit {
            return Some(pair);
        }

        let mut subgroup_contains: Option<(String, String)> = None;
        for group in available {
            for subgroup in &group.subgroup_ids {
                if normalize_group_key(subgroup).contains(subgroup_needle) {
                    if subgroup_contains.is_some() {
                        return None;
                    }
                    subgroup_contains = Some((group.group_id.clone(), subgroup.clone()));
                }
            }
        }
        if let Some(pair) = subgroup_contains {
            return Some(pair);
        }
    }

    None
}

/// Форматирование расписания в текст Telegram.
///
/// Почему сортировка здесь:
/// - API может вернуть занятия неотсортированными
/// - пользователю нужен человекочитаемый порядок по времени
fn format_schedule_message(
    group_id: &str,
    subgroup_id: &str,
    mut lessons: Vec<LessonResponse>,
    weekday: &str,
) -> String {
    lessons.sort_by(|a, b| a.start_time.cmp(&b.start_time));

    let header = if subgroup_id.is_empty() {
        format!("📅 Расписание на {weekday}\nГруппа: {group_id}\n")
    } else {
        format!(
            "📅 Расписание на {weekday}\nГруппа: {group_id}\nПодгруппа: {subgroup_id}\n"
        )
    };

    if lessons.is_empty() {
        return format!("{header}\nПар нет 🎉");
    }

    let mut lines = Vec::new();
    for (index, lesson) in lessons.iter().enumerate() {
        // Сборка строки максимально "бережно":
        // - trim у названия
        // - type и auditorium добавляем только если они заданы и не пустые
        let mut line = format!(
            "{}. {}-{} — {}",
            index + 1,
            lesson.start_time,
            lesson.end_time,
            lesson.name.trim()
        );

        if let Some(lesson_type) = lesson.lesson_type.as_ref() {
            if !lesson_type.is_empty() {
                line.push_str(" (");
                line.push_str(lesson_type);
                line.push(')');
            }
        }

        if let Some(auditorium) = lesson.auditorium.as_ref() {
            if !auditorium.is_empty() {
                line.push_str(" — ");
                line.push_str(auditorium);
            }
        }

        if let Some(teacher) = lesson.teacher.as_ref() {
            let name = teacher_display_name(teacher);
            if !name.is_empty() {
                line.push_str(" — ");
                line.push_str(&name);
            }
        }

        lines.push(line);
    }

    format!("{header}\n{}", lines.join("\n"))
}

fn teacher_display_name(teacher: &TeacherResponse) -> String {
    if let Some(fullname) = teacher.fullname.as_ref() {
        let v = fullname.trim();
        if !v.is_empty() {
            return v.to_string();
        }
    }
    let mut parts = Vec::new();
    if let Some(lastname) = teacher.lastname.as_ref() {
        let v = lastname.trim();
        if !v.is_empty() {
            parts.push(v);
        }
    }
    if let Some(initials) = teacher.initials.as_ref() {
        let v = initials.trim();
        if !v.is_empty() {
            parts.push(v);
        }
    } else {
        if let Some(firstname) = teacher.firstname.as_ref() {
            let v = firstname.trim();
            if !v.is_empty() {
                parts.push(v);
            }
        }
        if let Some(surname) = teacher.surname.as_ref() {
            let v = surname.trim();
            if !v.is_empty() {
                parts.push(v);
            }
        }
    }
    parts.join(" ")
}

/// Текущий день недели на русском.
///
/// Это прикладная функция для UI/параметров API.
/// Если API ожидает другой формат (например, "MONDAY"), это место нужно будет менять.
fn current_weekday_ru(offset_seconds: i32) -> &'static str {
    let offset = FixedOffset::east_opt(offset_seconds)
        .unwrap_or_else(|| FixedOffset::east_opt(0).unwrap());
    match Utc::now().with_timezone(&offset).weekday() {
        chrono::Weekday::Mon => "Понедельник",
        chrono::Weekday::Tue => "Вторник",
        chrono::Weekday::Wed => "Среда",
        chrono::Weekday::Thu => "Четверг",
        chrono::Weekday::Fri => "Пятница",
        chrono::Weekday::Sat => "Суббота",
        chrono::Weekday::Sun => "Воскресенье",
    }
}

fn normalize_group_key(value: &str) -> String {
    value
        .trim()
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect()
}

pub fn filter_groups_by_course(
    available: &[GroupWithSubgroupsIds],
    course: crate::domain::course::Course,
) -> Vec<GroupWithSubgroupsIds> {
    let course_number = match course {
        crate::domain::course::Course::First => 1,
        crate::domain::course::Course::Second => 2,
        crate::domain::course::Course::Third => 3,
        crate::domain::course::Course::Fourth => 4,
    };

    let year = Utc::now().year() % 100;
    let expected = year - course_number;

    let mut filtered: Vec<GroupWithSubgroupsIds> = available
        .iter()
        .cloned()
        .filter(|g| group_year_prefix(&g.group_id) == Some(expected))
        .collect();

    if filtered.is_empty() {
        return available.to_vec();
    }

    filtered.sort_by(|a, b| a.group_id.cmp(&b.group_id));
    filtered
}

fn group_year_prefix(group_id: &str) -> Option<i32> {
    let digits: String = group_id.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.len() >= 2 {
        digits[..2].parse::<i32>().ok()
    } else {
        None
    }
}

fn pick_subgroup_with_preference(
    selected: &GroupWithSubgroupsIds,
    needle: &str,
    subgroup_needle: Option<&str>,
) -> Option<String> {
    if selected.subgroup_ids.is_empty() {
        return Some(String::new());
    }
    if subgroup_needle.is_none() && normalize_group_key(&selected.group_id) == *needle {
        return Some(String::new());
    }
    if let Some(subgroup_needle) = subgroup_needle {
        let exact: Vec<&String> = selected
            .subgroup_ids
            .iter()
            .filter(|id| normalize_group_key(id) == subgroup_needle)
            .collect();
        if exact.len() == 1 {
            return Some(exact[0].clone());
        }
        if exact.len() > 1 {
            return None;
        }

        let contains: Vec<&String> = selected
            .subgroup_ids
            .iter()
            .filter(|id| normalize_group_key(id).contains(subgroup_needle))
            .collect();
        if contains.len() == 1 {
            return Some(contains[0].clone());
        }
        if contains.len() > 1 {
            return None;
        }
    }
    let exact: Vec<&String> = selected
        .subgroup_ids
        .iter()
        .filter(|id| normalize_group_key(id) == *needle)
        .collect();
    if exact.len() == 1 {
        return Some(exact[0].clone());
    }
    if exact.len() > 1 {
        return None;
    }

    let contains: Vec<&String> = selected
        .subgroup_ids
        .iter()
        .filter(|id| normalize_group_key(id).contains(needle))
        .collect();
    if contains.len() == 1 {
        return Some(contains[0].clone());
    }

    if selected.subgroup_ids.len() == 1 {
        return Some(selected.subgroup_ids[0].clone());
    }

    None
}

/// Нормализация факультета под контракт внешнего API.
///
/// Почему нужна:
/// - в БД/регистрации факультет может храниться как "Факультет математики и ...",
///   а API ждёт короткое "ФМиИТ".
///
/// Возвращаем Option:
/// - Some("ФМиИТ") если уверенно распознали
/// - None если факультет неизвестный → выше есть явная обработка
pub(crate) fn normalize_faculty(value: &str) -> Option<&'static str> {
    let normalized = value.to_lowercase();

    // "распознавание по подстрокам" — эвристика для пользовательских/разноформатных данных.
    if normalized.contains("математики") && normalized.contains("информационных") {
        return Some("ФМиИТ");
    }
    if normalized == "фмиит" {
        return Some("ФМиИТ");
    }

    None
}
