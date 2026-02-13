use std::str::FromStr;

use crate::domain::{
    course::Course, faculty::Faculty, groups::mit::MitGroup, study_form::StudyForm,
};

/// UI-действия главного меню / навигации.
///
/// Здесь важно держать **две проекции** одного и того же действия:
/// 1) `title()` — человекочитаемый текст (то, что видит пользователь)
/// 2) `callback()` — машиночитаемый идентификатор (то, что прилетает обратно в обработчик)
///
/// Это разделение критично:
/// - title может меняться из-за UX/локализации/эмодзи
/// - callback должен быть максимально стабильным, иначе сломаются старые сообщения/кнопки

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    MainMenu,
    Register,
    MyProfile,
    MySchedule,
    ChooseGroup,
    Help,
    Back,
}

impl Action {
    /// Текст кнопки для пользователя.
    ///
    /// Возвращаем `&'static str`, потому что все значения — compile-time константы,
    /// нет аллокаций, нет форматирования, нет лишних `String`.
    pub fn title(&self) -> &'static str {
        match self {
            Action::MainMenu => "🏠 Главное меню",
            Action::Register => "📝 Зарегистрироваться",
            Action::MyProfile => "👤 Мой профиль",
            Action::MySchedule => "📅 Моё расписание",
            Action::ChooseGroup => "🏫 Сменить факультет",
            Action::Help => "❓ Справка",
            Action::Back => "◀️ Назад",
        }
    }

    /// Значение, которое будет уложено в `callback_data` кнопки.
    ///
    /// ⚠️ Важно: этот контракт должен быть стабильным.
    /// Если менять строки здесь — "сломаются" уже отправленные сообщения с кнопками,
    /// потому что Telegram будет присылать старые callback_data.
    pub fn callback(&self) -> &'static str {
        match self {
            Action::MainMenu => "menu_main",
            Action::Register => "menu_register",
            Action::MyProfile => "menu_profile",
            Action::MySchedule => "menu_schedule",
            Action::ChooseGroup => "menu_choose_group",
            Action::Help => "menu_help",
            Action::Back => "menu_back",
        }
    }
}

impl FromStr for Action {
    /// Здесь ошибка не несёт данных: либо распарсили, либо нет.
    ///
    /// Мы сохраняем тип `()` как в исходнике, чтобы не ломать код вокруг.
    /// Диагностику (что именно пришло) делаем логами в местах, где ошибка становится важной.
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Небольшой debug лог полезен при "плавающих" проблемах с callback_data,
        // но может быть шумным, если парсинг вызывается часто.
        // Если станет слишком много — понижаем до trace или логируем только ошибки.
        log::debug!("parsing Action from callback string: {}", s);

        match s {
            // Поддерживаем несколько алиасов для обратной совместимости.
            // Это позволяет менять формат callback_data постепенно,
            // не ломая старые кнопки и сообщения.
            "menu_main" | "action:main" | "main" => Ok(Action::MainMenu),
            "menu_register" | "register" | "action:register" => Ok(Action::Register),
            "menu_profile" | "profile" | "action:profile" => Ok(Action::MyProfile),
            "menu_schedule" | "my_schedule" | "schedule" | "action:schedule" => {
                Ok(Action::MySchedule)
            }
            "menu_choose_group" | "choose_group" | "action:choose_group" => Ok(Action::ChooseGroup),
            "menu_help" | "help" | "action:help" => Ok(Action::Help),
            "menu_back" | "back" | "action:back" => Ok(Action::Back),

            // Ошибка парсинга Action — нормальная ситуация, потому что строка может
            // принадлежать другому доменному типу (Faculty, Course, Group...).
            // Поэтому здесь не error, а просто "не распознали".
            _ => {
                log::debug!("unknown Action callback string: {}", s);
                Err(())
            }
        }
    }
}

/// Единое представление callback_data для обработчиков.
///
/// Идея:
/// callback_data в Telegram — это просто `String`, но мы хотим работать типобезопасно:
/// - Action: навигация по меню
/// - Faculty/StudyForm/Course: выбор параметров регистрации
/// - MitGroup: выбор конкретной группы
///
/// Это снимает кучу ошибок вида "опечатались в строке и всё молча сломалось".
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Callback {
    Action(Action),
    Faculty(Faculty),
    StudyForm(StudyForm),
    Course(Course),
    MitGroup(MitGroup),
    GroupId(String),
    SubgroupChoice {
        group_id: String,
        subgroup_id: String,
    },
    ScheduleMenu,
    ScheduleDate(String),
    ScheduleWeek {
        start: String,
        end: String,
    },
    SchedulePicker {
        start: String,
    },
}

impl FromStr for Callback {
    /// Здесь возвращаем `String`, чтобы ошибка была человекочитаема,
    /// и её можно было:
    /// - залогировать
    /// - показать пользователю (в мягком виде)
    /// - отправить в мониторинг
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Это ключевой лог для отладки "почему кнопки не работают".
        // Когда прилетает странный callback_data — первым делом смотри сюда.
        log::debug!("parsing Callback from callback string: {}", s);

        // Важно: порядок имеет значение.
        // Сначала пробуем Action, затем доменные значения.
        // Если форматы пересекутся (например, одинаковые строки в разных enum),
        // победит первый матч. Это осознанная точка контроля.
        if let Ok(action) = Action::from_str(s) {
            return Ok(Callback::Action(action));
        }
        if let Ok(faculty) = Faculty::from_str(s) {
            return Ok(Callback::Faculty(faculty));
        }
        if let Ok(form) = StudyForm::from_str(s) {
            return Ok(Callback::StudyForm(form));
        }
        if let Ok(course) = Course::from_str(s) {
            return Ok(Callback::Course(course));
        }
        if let Some(rest) = s.strip_prefix("group:") {
            return Ok(Callback::GroupId(rest.to_string()));
        }
        if let Some(rest) = s.strip_prefix("subgroup:") {
            if let Some((group_id, subgroup_id)) = rest.split_once('|') {
                return Ok(Callback::SubgroupChoice {
                    group_id: group_id.to_string(),
                    subgroup_id: subgroup_id.to_string(),
                });
            }
            return Ok(Callback::SubgroupChoice {
                group_id: String::new(),
                subgroup_id: rest.to_string(),
            });
        }
        if s == "schedule:menu" {
            return Ok(Callback::ScheduleMenu);
        }
        if let Some(rest) = s.strip_prefix("schedule:date:") {
            return Ok(Callback::ScheduleDate(rest.to_string()));
        }
        if let Some(rest) = s.strip_prefix("schedule:week:") {
            let (start, end) = rest
                .split_once('|')
                .map(|(a, b)| (a.to_string(), b.to_string()))
                .unwrap_or_else(|| (rest.to_string(), String::new()));
            return Ok(Callback::ScheduleWeek { start, end });
        }
        if let Some(rest) = s.strip_prefix("schedule:picker:") {
            return Ok(Callback::SchedulePicker {
                start: rest.to_string(),
            });
        }
        if let Ok(group) = MitGroup::from_str(s) {
            return Ok(Callback::MitGroup(group));
        }

        // Если дошли сюда — данные не распознаны ни одним доменным парсером.
        // Это может быть:
        // - старая кнопка (устаревший формат callback_data)
        // - ручная подмена callback_data
        // - баг генерации клавиатуры
        //
        // warn оправдан: это не "падение сервиса", но это явная проблема UX,
        // которую нужно видеть в логах.
        log::warn!("unknown callback received: {}", s);

        Err(format!("Unknown callback: {}", s))
    }
}
