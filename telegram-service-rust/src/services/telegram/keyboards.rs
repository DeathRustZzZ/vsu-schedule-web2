use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use crate::domain::buttons::TechButton;

/// Клавиатура для незарегистрированного пользователя
pub fn registration_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![vec![
        InlineKeyboardButton::callback(
            TechButton::Register.title(),
            TechButton::Register.callback(),
        ),
    ]])
}

/// Клавиатура для зарегистрированного пользователя
pub fn schedule_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback(
            TechButton::MySchedule.title(),
            TechButton::MySchedule.callback(),
        )],
        vec![InlineKeyboardButton::callback(
            TechButton::ChooseGroup.title(),
            TechButton::ChooseGroup.callback(),
        )],
    ])
}
