use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use crate::bot::callbacks::Action;
use crate::bot::schedule_api::GroupWithSubgroupsIds;
use crate::domain::{course::Course, faculty::Faculty, groups::mit::MitGroup, study_form::StudyForm};

/// Главное меню бота.
///
/// Поведение зависит от состояния пользователя:
/// - зарегистрирован → показываем профиль, расписание и т.д.
/// - не зарегистрирован → минимальный набор действий
///
/// Важно:
/// эта функция не знает, как определяется регистрация,
/// она лишь принимает флаг — чистое разделение ответственности.
pub fn main_menu(is_registered: bool) -> InlineKeyboardMarkup {
    log::debug!(
        "building main menu keyboard (is_registered = {})",
        is_registered
    );

    if is_registered {
        // Меню для зарегистрированного пользователя.
        // Двухстрочная сетка выбрана осознанно:
        // UX лучше, чем длинный столбец кнопок.
        InlineKeyboardMarkup::new(vec![
            vec![
                InlineKeyboardButton::callback(
                    Action::MyProfile.title(),
                    Action::MyProfile.callback(),
                ),
                InlineKeyboardButton::callback(
                    Action::MySchedule.title(),
                    Action::MySchedule.callback(),
                ),
            ],
            vec![
                InlineKeyboardButton::callback(
                    Action::ChooseGroup.title(),
                    Action::ChooseGroup.callback(),
                ),
                InlineKeyboardButton::callback(
                    Action::Help.title(),
                    Action::Help.callback(),
                ),
            ],
        ])
    } else {
        // Меню для незарегистрированного пользователя.
        // Намеренно ограничиваем доступные действия,
        // чтобы не обрабатывать лишние состояния в роутере.
        InlineKeyboardMarkup::new(vec![
            vec![InlineKeyboardButton::callback(
                Action::Register.title(),
                Action::Register.callback(),
            )],
            vec![InlineKeyboardButton::callback(
                Action::Help.title(),
                Action::Help.callback(),
            )],
        ])
    }
}

/// Универсальная клавиатура с кнопкой "Назад".
///
/// Используется как навигационный элемент между шагами.
/// Выделена в отдельную функцию, чтобы:
/// - не дублировать callback
/// - гарантировать единый контракт кнопки "Назад"
pub fn back_menu() -> InlineKeyboardMarkup {
    log::debug!("building back menu keyboard");

    InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
        Action::Back.title(),
        Action::Back.callback(),
    )]])
}

/// Клавиатура выбора факультета.
///
/// На данный момент содержит один факультет,
/// но структура функции уже готова к расширению.
pub fn faculty_keyboard() -> InlineKeyboardMarkup {
    log::debug!("building faculty selection keyboard");

    // Используем массив, а не Vec:
    // - compile-time размер
    // - очевидный набор значений
    let faculties = [Faculty::Mit];

    InlineKeyboardMarkup::new(
        faculties
            .iter()
            .map(|f| {
                log::debug!("adding faculty button: {}", f.callback());
                vec![InlineKeyboardButton::callback(f.title(), f.callback())]
            })
            .collect::<Vec<_>>(),
    )
}

/// Клавиатура выбора формы обучения.
pub fn study_form_keyboard() -> InlineKeyboardMarkup {
    log::debug!("building study form selection keyboard");

    let forms = [StudyForm::FullTime, StudyForm::PartTime];

    InlineKeyboardMarkup::new(
        forms
            .iter()
            .map(|f| {
                log::debug!("adding study form button: {}", f.callback());
                vec![InlineKeyboardButton::callback(f.title(), f.callback())]
            })
            .collect::<Vec<_>>(),
    )
}

/// Клавиатура выбора курса.
///
/// Порядок элементов важен:
/// курс отображается сверху вниз в логической последовательности.
pub fn course_keyboard() -> InlineKeyboardMarkup {
    log::debug!("building course selection keyboard");

    let courses = [
        Course::First,
        Course::Second,
        Course::Third,
        Course::Fourth,
    ];

    InlineKeyboardMarkup::new(
        courses
            .iter()
            .map(|c| {
                log::debug!("adding course button: {}", c.callback());
                vec![InlineKeyboardButton::callback(c.title(), c.callback())]
            })
            .collect::<Vec<_>>(),
    )
}

/// Клавиатура выбора группы МИТ.
///
/// Каждая кнопка соответствует конкретной учебной группе.
/// callback_data напрямую связан с доменной моделью MitGroup.
pub fn mit_group_keyboard() -> InlineKeyboardMarkup {
    log::debug!("building MIT group selection keyboard");

    let groups = [
        MitGroup::PI24Z1,
        MitGroup::PI24Z2,
        MitGroup::PI23Z1,
        MitGroup::PI23Z2,
    ];

    InlineKeyboardMarkup::new(
        groups
            .iter()
            .map(|g| {
                log::debug!("adding MIT group button: {}", g.callback());
                vec![InlineKeyboardButton::callback(g.title(), g.callback())]
            })
            .collect::<Vec<_>>(),
    )
}

/// Клавиатура выбора группы, загружаемой из внешнего Schedule API.
pub fn groups_keyboard(groups: &[GroupWithSubgroupsIds]) -> InlineKeyboardMarkup {
    let mut rows = Vec::new();
    for group in groups {
        rows.push(vec![InlineKeyboardButton::callback(
            group.group_id.clone(),
            format!("group:{}", group.group_id),
        )]);
    }
    InlineKeyboardMarkup::new(rows)
}

/// Клавиатура выбора подгруппы.
pub fn subgroups_keyboard(subgroups: &[String]) -> InlineKeyboardMarkup {
    let mut rows = Vec::new();
    for subgroup in subgroups {
        rows.push(vec![InlineKeyboardButton::callback(
            subgroup.clone(),
            format!("subgroup:{}", subgroup),
        )]);
    }
    InlineKeyboardMarkup::new(rows)
}
