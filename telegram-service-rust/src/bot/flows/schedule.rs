use chrono::{Datelike, Duration, FixedOffset, NaiveDate, Utc};
use log::{debug, info, warn};
use teloxide::prelude::*;
use teloxide::types::MessageId;

use crate::bot::keyboards;
use crate::bot::schedule_api::{
    GroupWithSubgroupsIds, LessonResponse, ScheduleApi, TeacherResponse,
};
use crate::bot::ui::render_screen;
use crate::db::facade::DbFacade;

pub struct ScheduleFlowContext<'a> {
    pub bot: &'a Bot,
    pub db: &'a DbFacade,
    pub schedule_api: &'a ScheduleApi,
    pub schedule_tz_offset_seconds: i32,
    pub telegram_id: i64,
    pub chat_id: ChatId,
    pub message_id: Option<MessageId>,
}

impl<'a> ScheduleFlowContext<'a> {
    pub fn new(
        bot: &'a Bot,
        db: &'a DbFacade,
        schedule_api: &'a ScheduleApi,
        schedule_tz_offset_seconds: i32,
        telegram_id: i64,
        chat_id: ChatId,
        message_id: Option<MessageId>,
    ) -> Self {
        Self {
            bot,
            db,
            schedule_api,
            schedule_tz_offset_seconds,
            telegram_id,
            chat_id,
            message_id,
        }
    }
}

/// Показ меню выбора даты/недели для расписания.
pub async fn show_schedule_menu(
    bot: &Bot,
    db: &DbFacade,
    schedule_tz_offset_seconds: i32,
    telegram_id: i64,
    chat_id: ChatId,
    message_id: Option<MessageId>,
) -> Result<(), teloxide::RequestError> {
    debug!(
        "show_schedule_menu requested: user={} chat_id={}",
        telegram_id, chat_id.0
    );

    let is_registered = db.find_student(telegram_id).await.ok().flatten().is_some();
    if !is_registered {
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

    let menu = schedule_menu_keyboard_for_now(schedule_tz_offset_seconds);
    render_screen(
        bot,
        db,
        telegram_id,
        chat_id,
        message_id,
        "Выбери дату или неделю:",
        Some(menu),
    )
    .await
}

/// Показ выбора даты (список ближайших дней).
pub async fn show_date_picker(
    bot: &Bot,
    db: &DbFacade,
    schedule_tz_offset_seconds: i32,
    telegram_id: i64,
    chat_id: ChatId,
    message_id: Option<MessageId>,
    start_date: Option<NaiveDate>,
) -> Result<(), teloxide::RequestError> {
    debug!(
        "show_date_picker requested: user={} chat_id={} start_date={:?}",
        telegram_id, chat_id.0, start_date
    );

    let is_registered = db.find_student(telegram_id).await.ok().flatten().is_some();
    if !is_registered {
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

    let base = start_date.unwrap_or_else(|| current_date(schedule_tz_offset_seconds));
    let mut dates = Vec::new();
    for offset in 0..14 {
        let date = base + Duration::days(offset);
        let label = format!(
            "{} {}",
            weekday_ru_short(date.weekday()),
            format_display_date_short(date)
        );
        let value = format_api_date(date);
        dates.push((label, value));
    }

    let keyboard = keyboards::schedule_date_picker_keyboard(&dates);
    render_screen(
        bot,
        db,
        telegram_id,
        chat_id,
        message_id,
        "Выбери дату:",
        Some(keyboard),
    )
    .await
}

/// Показ расписания на конкретную дату.
pub async fn show_schedule_for_date(
    ctx: ScheduleFlowContext<'_>,
    date: NaiveDate,
) -> Result<(), teloxide::RequestError> {
    let ScheduleFlowContext {
        bot,
        db,
        schedule_api,
        schedule_tz_offset_seconds,
        telegram_id,
        chat_id,
        message_id,
    } = ctx;

    info!(
        "show_schedule_for_date requested: user={} chat_id={} date={}",
        telegram_id, chat_id.0, date
    );

    let context = match resolve_schedule_context(&ctx).await? {
        Some(context) => context,
        None => return Ok(()),
    };

    let date_str = format_api_date(date);
    let schedule = match schedule_api
        .get_schedule_by_date(
            &context.faculty,
            &context.group_id,
            &context.subgroup_id,
            &date_str,
        )
        .await
    {
        Ok(schedule) => {
            debug!("schedule loaded: lessons_count={}", schedule.lessons.len());
            schedule
        }
        Err(err) => {
            warn!("Failed to load schedule by date: {:?}", err);
            return render_screen(
                bot,
                db,
                telegram_id,
                chat_id,
                message_id,
                "❌ Не удалось получить расписание.",
                Some(keyboards::schedule_back_menu()),
            )
            .await;
        }
    };

    let text = format_schedule_message_for_date(
        &context.group_id,
        &context.subgroup_id,
        schedule.lessons,
        date,
    );

    render_screen(
        bot,
        db,
        telegram_id,
        chat_id,
        message_id,
        &text,
        Some(schedule_menu_keyboard_for_now(schedule_tz_offset_seconds)),
    )
    .await
}

/// Показ расписания за неделю (диапазон дат).
pub async fn show_schedule_for_week(
    ctx: ScheduleFlowContext<'_>,
    start: NaiveDate,
    end: NaiveDate,
) -> Result<(), teloxide::RequestError> {
    let ScheduleFlowContext {
        bot,
        db,
        schedule_api,
        schedule_tz_offset_seconds,
        telegram_id,
        chat_id,
        message_id,
    } = ctx;

    info!(
        "show_schedule_for_week requested: user={} chat_id={} start={} end={}",
        telegram_id, chat_id.0, start, end
    );

    let context = match resolve_schedule_context(&ctx).await? {
        Some(context) => context,
        None => return Ok(()),
    };

    let start_str = format_api_date(start);
    let end_str = format_api_date(end);
    let schedule = match schedule_api
        .get_schedule_week(
            &context.faculty,
            &context.group_id,
            &context.subgroup_id,
            &start_str,
            Some(&end_str),
        )
        .await
    {
        Ok(schedule) => {
            debug!(
                "week schedule loaded: lessons_count={}",
                schedule.lessons.len()
            );
            schedule
        }
        Err(err) => {
            warn!("Failed to load week schedule: {:?}", err);
            return render_screen(
                bot,
                db,
                telegram_id,
                chat_id,
                message_id,
                "❌ Не удалось получить расписание недели.",
                Some(keyboards::schedule_back_menu()),
            )
            .await;
        }
    };

    let text = format_schedule_message_for_week(
        &context.group_id,
        &context.subgroup_id,
        schedule.lessons,
        start,
        end,
    );

    render_screen(
        bot,
        db,
        telegram_id,
        chat_id,
        message_id,
        &text,
        Some(schedule_menu_keyboard_for_now(schedule_tz_offset_seconds)),
    )
    .await
}

struct ScheduleContext {
    faculty: String,
    group_id: String,
    subgroup_id: String,
}

async fn resolve_schedule_context(
    ctx: &ScheduleFlowContext<'_>,
) -> Result<Option<ScheduleContext>, teloxide::RequestError> {
    let ScheduleFlowContext {
        bot,
        db,
        schedule_api,
        telegram_id,
        chat_id,
        message_id,
        ..
    } = *ctx;

    let student = match db.find_student(telegram_id).await {
        Ok(Some(student)) => student,
        Ok(None) => {
            info!("user {} requested schedule but not registered", telegram_id);
            render_screen(
                bot,
                db,
                telegram_id,
                chat_id,
                message_id,
                "Сначала зарегистрируйся, чтобы смотреть расписание.",
                Some(keyboards::back_menu()),
            )
            .await?;
            return Ok(None);
        }
        Err(err) => {
            warn!(
                "failed to load student profile for user {}: {:?}",
                telegram_id, err
            );
            render_screen(
                bot,
                db,
                telegram_id,
                chat_id,
                message_id,
                "❌ Не удалось загрузить профиль.",
                Some(keyboards::back_menu()),
            )
            .await?;
            return Ok(None);
        }
    };

    let faculty = normalize_faculty(&student.faculty);
    debug!(
        "faculty normalization: raw='{}' normalized={:?}",
        student.faculty, faculty
    );

    if faculty != Some("ФМиИТ") {
        info!(
            "schedule requested for unsupported faculty: raw='{}' user={}",
            student.faculty, telegram_id
        );
        render_screen(
            bot,
            db,
            telegram_id,
            chat_id,
            message_id,
            "Пока поддерживается только факультет ФМиИТ.",
            Some(keyboards::back_menu()),
        )
        .await?;
        return Ok(None);
    }

    let available = match schedule_api
        .get_available_groups(faculty.unwrap_or(&student.faculty))
        .await
    {
        Ok(list) => {
            debug!("available groups loaded: count={}", list.list.len());
            list.list
        }
        Err(err) => {
            warn!("Failed to load groups list: {:?}", err);
            render_screen(
                bot,
                db,
                telegram_id,
                chat_id,
                message_id,
                "❌ Не удалось получить список групп.",
                Some(keyboards::back_menu()),
            )
            .await?;
            return Ok(None);
        }
    };

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
            render_screen(
                bot,
                db,
                telegram_id,
                chat_id,
                message_id,
                "❌ Не удалось подобрать группу. Обратись к администратору.",
                Some(keyboards::back_menu()),
            )
            .await?;
            return Ok(None);
        }
    };

    Ok(Some(ScheduleContext {
        faculty: faculty
            .map(|v| v.to_string())
            .unwrap_or_else(|| student.faculty.clone()),
        group_id,
        subgroup_id,
    }))
}

fn schedule_menu_keyboard_for_now(offset_seconds: i32) -> teloxide::types::InlineKeyboardMarkup {
    let today = current_date(offset_seconds);
    let tomorrow = today + Duration::days(1);
    let week_start = week_start_date(today);
    let week_end = week_start + Duration::days(6);
    let next_week_start = week_start + Duration::days(7);
    let next_week_end = next_week_start + Duration::days(6);

    keyboards::schedule_menu_keyboard(
        &format_api_date(today),
        &format_api_date(tomorrow),
        &format_api_date(week_start),
        &format_api_date(week_end),
        &format_api_date(next_week_start),
        &format_api_date(next_week_end),
    )
}

/// Форматирование расписания на дату.
fn format_schedule_message_for_date(
    group_id: &str,
    subgroup_id: &str,
    mut lessons: Vec<LessonResponse>,
    date: NaiveDate,
) -> String {
    lessons.sort_by_key(lesson_time_key);

    let header = if subgroup_id.is_empty() {
        format!(
            "📅 Расписание на {} ({})\nГруппа: {}\n",
            format_display_date(date),
            weekday_ru_full(date.weekday()),
            group_id
        )
    } else {
        format!(
            "📅 Расписание на {} ({})\nГруппа: {}\nПодгруппа: {}\n",
            format_display_date(date),
            weekday_ru_full(date.weekday()),
            group_id,
            subgroup_id
        )
    };

    if lessons.is_empty() {
        return format!("{header}\nПар нет 🎉");
    }

    let mut lines = Vec::new();
    for (index, lesson) in lessons.iter().enumerate() {
        let prefix = format!("{}.", index + 1);
        lines.extend(format_lesson_block(lesson, Some(&prefix)));
    }

    format!("{header}\n{}", lines.join("\n"))
}

/// Форматирование расписания за неделю.
fn format_schedule_message_for_week(
    group_id: &str,
    subgroup_id: &str,
    lessons: Vec<LessonResponse>,
    start: NaiveDate,
    end: NaiveDate,
) -> String {
    let header = if subgroup_id.is_empty() {
        format!(
            "📅 Расписание на неделю\n{} - {}\nГруппа: {}\n",
            format_display_date(start),
            format_display_date(end),
            group_id
        )
    } else {
        format!(
            "📅 Расписание на неделю\n{} - {}\nГруппа: {}\nПодгруппа: {}\n",
            format_display_date(start),
            format_display_date(end),
            group_id,
            subgroup_id
        )
    };

    if lessons.is_empty() {
        return format!("{header}\nПар нет 🎉");
    }

    let mut items: Vec<LessonWithDate> = lessons
        .into_iter()
        .map(|lesson| {
            let (date_value, label) = lesson_date_label(&lesson);
            LessonWithDate {
                date: date_value,
                label,
                lesson,
            }
        })
        .collect();

    items.sort_by(|a, b| {
        let date_cmp = match (&a.date, &b.date) {
            (Some(a_date), Some(b_date)) => a_date.cmp(b_date),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a.label.cmp(&b.label),
        };

        if date_cmp != std::cmp::Ordering::Equal {
            return date_cmp;
        }

        lesson_time_key(&a.lesson).cmp(&lesson_time_key(&b.lesson))
    });

    let mut lines = Vec::new();
    let mut current_label: Option<String> = None;

    for (index, item) in items.iter().enumerate() {
        if current_label.as_deref() != Some(&item.label) {
            if index > 0 {
                lines.push(String::new());
            }
            lines.push(item.label.clone());
            current_label = Some(item.label.clone());
        }

        lines.extend(format_lesson_block(&item.lesson, None));
    }

    format!("{header}\n{}", lines.join("\n"))
}

fn format_lesson_block(lesson: &LessonResponse, prefix: Option<&str>) -> Vec<String> {
    let time = format_lesson_time(lesson);
    let name = lesson
        .name
        .as_deref()
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .unwrap_or("Без названия");

    let time_cell = if time.chars().count() <= 11 {
        format!("{:<11}", time)
    } else {
        time
    };

    let mut lines = Vec::new();

    let mut time_line = String::new();
    if let Some(prefix) = prefix {
        time_line.push_str(prefix);
        time_line.push(' ');
    }
    time_line.push_str("🕒 ");
    time_line.push_str(&time_cell);
    lines.push(time_line);

    let mut name_line = String::from("📘 ");
    name_line.push_str(name);

    if let Some(lesson_type) = lesson.lesson_type.as_ref().map(|v| v.trim())
        && !lesson_type.is_empty()
    {
        name_line.push_str(" (");
        name_line.push_str(lesson_type);
        name_line.push(')');
    }
    lines.push(name_line);

    if let Some(auditorium) = lesson.auditorium.as_ref().map(|v| v.trim())
        && !auditorium.is_empty()
    {
        let normalized = normalize_auditorium(auditorium);
        lines.push(format!("🏫 ауд. {}", normalized));
    }

    if let Some(teacher) = lesson.teacher.as_ref() {
        let name = teacher_display_name(teacher);
        if !name.is_empty() {
            lines.push(format!("👤 {}", name));
        }
    }

    lines.push(String::new());
    lines
}

fn normalize_auditorium(raw: &str) -> String {
    let trimmed = raw.trim();
    let lower = trimmed.to_lowercase();
    if lower.starts_with("ауд.") {
        return trimmed[4..].trim_start().to_string();
    }
    if lower.starts_with("ауд ") {
        return trimmed[4..].trim_start().to_string();
    }
    if lower.starts_with("ауд:") {
        return trimmed[4..].trim_start().to_string();
    }
    trimmed.to_string()
}

struct LessonWithDate {
    date: Option<NaiveDate>,
    label: String,
    lesson: LessonResponse,
}

fn lesson_date_label(lesson: &LessonResponse) -> (Option<NaiveDate>, String) {
    if let Some(raw) = lesson.date.as_deref() {
        let raw = raw.trim();
        if !raw.is_empty() {
            if let Some(date) = parse_lesson_date(raw) {
                let label = format!(
                    "{} ({})",
                    format_display_date(date),
                    weekday_ru_short(date.weekday())
                );
                return (Some(date), label);
            }
            return (None, raw.to_string());
        }
    }

    (None, "Без даты".to_string())
}

fn parse_lesson_date(raw: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(raw, "%Y-%m-%d")
        .or_else(|_| NaiveDate::parse_from_str(raw, "%d.%m.%Y"))
        .or_else(|_| NaiveDate::parse_from_str(raw, "%d/%m/%Y"))
        .or_else(|_| NaiveDate::parse_from_str(raw, "%d.%m.%y"))
        .or_else(|_| NaiveDate::parse_from_str(raw, "%d/%m/%y"))
        .ok()
        .or_else(|| parse_russian_date(raw))
}

fn parse_russian_date(raw: &str) -> Option<NaiveDate> {
    let parts: Vec<&str> = raw.split_whitespace().collect();
    if parts.len() < 3 {
        return None;
    }

    let day_str = parts[0].trim_matches(|c: char| !c.is_ascii_digit());
    let mut month_str = parts[1].to_lowercase();
    month_str.retain(|c| c.is_alphabetic() || c == 'ё');
    let year_str = parts[2].trim_matches(|c: char| !c.is_ascii_digit());

    let day: u32 = day_str.parse().ok()?;
    let year: i32 = year_str.parse().ok()?;
    let month = match month_str.as_str() {
        "января" => 1,
        "февраля" => 2,
        "марта" => 3,
        "апреля" => 4,
        "мая" => 5,
        "июня" => 6,
        "июля" => 7,
        "августа" => 8,
        "сентября" => 9,
        "октября" => 10,
        "ноября" => 11,
        "декабря" => 12,
        _ => return None,
    };

    NaiveDate::from_ymd_opt(year, month, day)
}

fn format_lesson_time(lesson: &LessonResponse) -> String {
    let start = lesson.start_time.as_deref().map(str::trim).unwrap_or("");
    let end = lesson.end_time.as_deref().map(str::trim).unwrap_or("");

    if !start.is_empty() && !end.is_empty() {
        return format!("{}-{}", start, end);
    }

    if !start.is_empty() {
        return start.to_string();
    }

    if !end.is_empty() {
        return end.to_string();
    }

    "Время не указано".to_string()
}

fn lesson_time_key(lesson: &LessonResponse) -> String {
    lesson.start_time.as_deref().unwrap_or("99:99").to_string()
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

fn current_date(offset_seconds: i32) -> NaiveDate {
    let offset =
        FixedOffset::east_opt(offset_seconds).unwrap_or_else(|| FixedOffset::east_opt(0).unwrap());
    Utc::now().with_timezone(&offset).date_naive()
}

fn week_start_date(date: NaiveDate) -> NaiveDate {
    let weekday = date.weekday().num_days_from_monday() as i64;
    date - Duration::days(weekday)
}

fn format_display_date(date: NaiveDate) -> String {
    date.format("%d.%m.%Y").to_string()
}

fn format_display_date_short(date: NaiveDate) -> String {
    date.format("%d.%m").to_string()
}

fn format_api_date(date: NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

fn weekday_ru_full(weekday: chrono::Weekday) -> &'static str {
    match weekday {
        chrono::Weekday::Mon => "Понедельник",
        chrono::Weekday::Tue => "Вторник",
        chrono::Weekday::Wed => "Среда",
        chrono::Weekday::Thu => "Четверг",
        chrono::Weekday::Fri => "Пятница",
        chrono::Weekday::Sat => "Суббота",
        chrono::Weekday::Sun => "Воскресенье",
    }
}

fn weekday_ru_short(weekday: chrono::Weekday) -> &'static str {
    match weekday {
        chrono::Weekday::Mon => "Пн",
        chrono::Weekday::Tue => "Вт",
        chrono::Weekday::Wed => "Ср",
        chrono::Weekday::Thu => "Чт",
        chrono::Weekday::Fri => "Пт",
        chrono::Weekday::Sat => "Сб",
        chrono::Weekday::Sun => "Вс",
    }
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
        let subgroup =
            pick_subgroup_with_preference(&selected, &needle, subgroup_needle.as_deref())?;
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
        let subgroup =
            pick_subgroup_with_preference(&selected, &needle, subgroup_needle.as_deref())?;
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
        .filter(|g| group_year_prefix(&g.group_id) == Some(expected))
        .cloned()
        .collect();

    if filtered.is_empty() {
        return available.to_vec();
    }

    filtered.sort_by(|a, b| a.group_id.cmp(&b.group_id));
    filtered
}

fn group_year_prefix(group_id: &str) -> Option<i32> {
    let digits: String = group_id
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
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
/// - внешнее API ждёт короткие значения.
pub fn normalize_faculty(value: &str) -> Option<&'static str> {
    let v = value.trim().to_lowercase();
    if v.contains("фмиит") || v.contains("матем") {
        return Some("ФМиИТ");
    }
    None
}
