//src/services/telegram/menu.rs
//! Система меню с поддержкой клавиатур для удобной навигации

use teloxide::types::{
    InlineKeyboardButton, InlineKeyboardMarkup,
};
use crate::domain::menu::MenuCommand;

/// Создаёт Inline клавиатуру главного меню для незарегистрированного пользователя
pub fn main_menu_inline_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback(
                MenuCommand::Register.text(),
                MenuCommand::Register.callback(),
            ),
        ],
        vec![
            InlineKeyboardButton::callback(
                MenuCommand::Help.text(),
                MenuCommand::Help.callback(),
            ),
        ],
    ])
}

/// Создаёт Inline клавиатуру меню зарегистрированного пользователя
pub fn user_menu_inline_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![
        vec![
            InlineKeyboardButton::callback(
                MenuCommand::MyProfile.text(),
                MenuCommand::MyProfile.callback(),
            ),
            InlineKeyboardButton::callback(
                MenuCommand::MySchedule.text(),
                MenuCommand::MySchedule.callback(),
            ),
        ],
        vec![
            InlineKeyboardButton::callback(
                MenuCommand::ChooseGroup.text(),
                MenuCommand::ChooseGroup.callback(),
            ),
            InlineKeyboardButton::callback(
                MenuCommand::Help.text(),
                MenuCommand::Help.callback(),
            ),
        ],
    ])
}

/// Inline клавиатура с кнопкой регистрации
pub fn registration_inline_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![vec![
        InlineKeyboardButton::callback(
            MenuCommand::Register.text(),
            MenuCommand::Register.callback(),
        ),
    ]])
}

/// Inline клавиатура с опциями меню
pub fn menu_inline_keyboard(commands: &[MenuCommand]) -> InlineKeyboardMarkup {
    let buttons: Vec<InlineKeyboardButton> = commands
        .iter()
        .map(|cmd| {
            InlineKeyboardButton::callback(cmd.text(), cmd.callback())
        })
        .collect();

    InlineKeyboardMarkup::new(vec![buttons])
}

/// Создаёт двухколоночное меню
pub fn menu_two_column_keyboard(commands: &[MenuCommand]) -> InlineKeyboardMarkup {
    let mut rows = Vec::new();
    for chunk in commands.chunks(2) {
        let row: Vec<InlineKeyboardButton> = chunk
            .iter()
            .map(|cmd| {
                InlineKeyboardButton::callback(cmd.text(), cmd.callback())
            })
            .collect();
        rows.push(row);
    }
    InlineKeyboardMarkup::new(rows)
}

/// Меню с кнопкой "Назад"
pub fn menu_with_back_keyboard(commands: &[MenuCommand]) -> InlineKeyboardMarkup {
    let mut rows = Vec::new();

    // Добавляем основные кнопки двумя в строку
    for chunk in commands.chunks(2) {
        let row: Vec<InlineKeyboardButton> = chunk
            .iter()
            .map(|cmd| {
                InlineKeyboardButton::callback(cmd.text(), cmd.callback())
            })
            .collect();
        rows.push(row);
    }

    // Добавляем кнопку "Назад" в отдельной строке
    rows.push(vec![InlineKeyboardButton::callback(
        MenuCommand::Back.text(),
        MenuCommand::Back.callback(),
    )]);

    InlineKeyboardMarkup::new(rows)
}

/// Простая Inline клавиатура с перечисленными элементами (2 в строке)
pub fn simple_inline_keyboard(items: &[(String, String)]) -> InlineKeyboardMarkup {
    let mut rows = Vec::new();
    for chunk in items.chunks(2) {
        let row: Vec<InlineKeyboardButton> = chunk
            .iter()
            .map(|(text, callback)| {
                InlineKeyboardButton::callback(text.clone(), callback.clone())
            })
            .collect();
        rows.push(row);
    }

    InlineKeyboardMarkup::new(rows)
}


