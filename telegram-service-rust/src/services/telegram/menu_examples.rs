// examples/menu_usage_example.rs
//! Практические примеры использования системы меню

use teloxide::prelude::*;
use crate::domain::menu::MenuCommand;
use crate::services::telegram::menu;

/// Пример 1: Отправка главного меню
pub async fn example_send_main_menu(
    bot: &Bot,
    chat_id: ChatId,
) -> Result<(), teloxide::RequestError> {
    bot.send_message(
        chat_id,
        "🏠 *Главное меню*\n\nВыберите действие из предложенных вариантов:",
    )
    .reply_markup(menu::main_menu_inline_keyboard())
    .await?;

    Ok(())
}

/// Пример 2: Отправка меню для зарегистрированного пользователя
pub async fn example_send_user_menu(
    bot: &Bot,
    chat_id: ChatId,
    username: &str,
) -> Result<(), teloxide::RequestError> {
    bot.send_message(
        chat_id,
        format!("👋 *Добро пожаловать, {}!*\n\n📚 Выберите действие:", username),
    )
    .reply_markup(menu::user_menu_inline_keyboard())
    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
    .await?;

    Ok(())
}

/// Пример 3: Создание кастомного меню
pub async fn example_custom_menu(
    bot: &Bot,
    chat_id: ChatId,
) -> Result<(), teloxide::RequestError> {
    let commands = vec![
        MenuCommand::MyProfile,
        MenuCommand::MySchedule,
        MenuCommand::ChooseGroup,
    ];

    bot.send_message(
        chat_id,
        "📋 *Выберите опцию:*",
    )
    .reply_markup(menu::menu_two_column_keyboard(&commands))
    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
    .await?;

    Ok(())
}

/// Пример 4: Меню с кнопкой "Назад"
pub async fn example_menu_with_back(
    bot: &Bot,
    chat_id: ChatId,
) -> Result<(), teloxide::RequestError> {
    let commands = vec![
        MenuCommand::Register,
        MenuCommand::Help,
    ];

    bot.send_message(
        chat_id,
        "📋 *Дополнительные опции:*",
    )
    .reply_markup(menu::menu_with_back_keyboard(&commands))
    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
    .await?;

    Ok(())
}

/// Пример 5: Редактирование сообщения с новым меню
pub async fn example_edit_menu(
    bot: &Bot,
    chat_id: ChatId,
    message_id: i32,
) -> Result<(), teloxide::RequestError> {
    bot.edit_message_text(
        chat_id,
        teloxide::types::MessageId(message_id),
        "🔄 *Меню обновлено*\n\nВыберите новое действие:",
    )
    .reply_markup(menu::user_menu_inline_keyboard())
    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
    .await?;

    Ok(())
}

/// Пример 6: Меню с динамическими опциями
pub async fn example_dynamic_menu(
    bot: &Bot,
    chat_id: ChatId,
    options: Vec<(String, String)>,
) -> Result<(), teloxide::RequestError> {
    bot.send_message(
        chat_id,
        "📋 *Выберите из доступных опций:*",
    )
    .reply_markup(menu::simple_inline_keyboard(&options))
    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
    .await?;

    Ok(())
}

/// Пример 7: Последовательное меню (шаг за шагом)
pub async fn example_sequential_menu(
    bot: &Bot,
    chat_id: ChatId,
    step: u32,
) -> Result<(), teloxide::RequestError> {
    match step {
        1 => {
            // Первый шаг - выбор факультета
            bot.send_message(
                chat_id,
                "1️⃣ *Шаг 1: Выберите факультет*",
            )
            .reply_markup(menu::menu_inline_keyboard(&[MenuCommand::Back]))
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .await?;
        }
        2 => {
            // Второй шаг - выбор формы обучения
            bot.send_message(
                chat_id,
                "2️⃣ *Шаг 2: Выберите форму обучения*",
            )
            .reply_markup(menu::menu_with_back_keyboard(&[MenuCommand::ChooseGroup]))
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .await?;
        }
        _ => {
            // Завершение
            bot.send_message(
                chat_id,
                "✅ *Регистрация завершена!*",
            )
            .reply_markup(menu::main_menu_inline_keyboard())
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .await?;
        }
    }

    Ok(())
}

/// Пример 8: Меню с отключением (disabled-like)
pub async fn example_menu_with_info(
    bot: &Bot,
    chat_id: ChatId,
    info_message: &str,
) -> Result<(), teloxide::RequestError> {
    bot.send_message(
        chat_id,
        format!(
            "ℹ️ *Информация:*\n{}\n\n📋 *Выберите действие:*",
            info_message
        ),
    )
    .reply_markup(menu::menu_two_column_keyboard(&[
        MenuCommand::MyProfile,
        MenuCommand::Help,
        MenuCommand::Back,
    ]))
    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_command_text() {
        assert_eq!(MenuCommand::MainMenu.text(), "🏠 Главное меню");
        assert_eq!(MenuCommand::Register.text(), "📝 Зарегистрироваться");
        assert_eq!(MenuCommand::MyProfile.text(), "👤 Мой профиль");
        assert_eq!(MenuCommand::Back.text(), "◀️ Назад");
    }

    #[test]
    fn test_menu_command_callback() {
        assert_eq!(MenuCommand::MainMenu.callback(), "menu_main");
        assert_eq!(MenuCommand::Register.callback(), "menu_register");
        assert_eq!(MenuCommand::MyProfile.callback(), "menu_profile");
        assert_eq!(MenuCommand::Back.callback(), "menu_back");
    }

    #[test]
    fn test_menu_command_from_str() {
        use std::str::FromStr;

        assert!(MenuCommand::from_str("menu_main").is_ok());
        assert!(MenuCommand::from_str("menu_register").is_ok());
        assert!(MenuCommand::from_str("menu_back").is_ok());
        assert!(MenuCommand::from_str("invalid").is_err());
    }
}

