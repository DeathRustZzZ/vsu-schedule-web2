use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use crate::domain::{faculty::Faculty, study_form::StudyForm, course::Course};
use crate::domain::groups::mit::MitGroup;

/// Клавиатура выбора факультета
pub fn faculty_keyboard() -> InlineKeyboardMarkup {
    let faculties = [Faculty::Mit, Faculty::Law, Faculty::Ped];
    InlineKeyboardMarkup::new(
        faculties.iter()
            .map(|f| vec![InlineKeyboardButton::callback(f.title(), f.callback())])
            .collect::<Vec<_>>(),
    )
}

/// Клавиатура выбора формы обучения
pub fn study_form_keyboard() -> InlineKeyboardMarkup {
    let forms = [StudyForm::FullTime, StudyForm::PartTime];
    InlineKeyboardMarkup::new(
        forms.iter()
            .map(|f| vec![InlineKeyboardButton::callback(f.title(), f.callback())])
            .collect::<Vec<_>>(),
    )
}

/// Клавиатура выбора курса
pub fn course_keyboard() -> InlineKeyboardMarkup {
    let courses = [Course::First, Course::Second, Course::Third, Course::Fourth];
    InlineKeyboardMarkup::new(
        courses.iter()
            .map(|c| vec![InlineKeyboardButton::callback(c.title(), c.callback())])
            .collect::<Vec<_>>(),
    )
}

/// Клавиатура выбора группы (МИТ)
pub fn mit_group_keyboard() -> InlineKeyboardMarkup {
    let groups = [MitGroup::ISIT, MitGroup::PI, MitGroup::PInj, MitGroup::PM, MitGroup::UIR, MitGroup::MF];
    InlineKeyboardMarkup::new(
        groups.iter()
            .map(|g| vec![InlineKeyboardButton::callback(g.title(), g.callback())])
            .collect::<Vec<_>>(),
    )
}
