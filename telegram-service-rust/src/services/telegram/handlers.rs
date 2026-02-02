use teloxide::prelude::*;
use crate::db::facade::DbFacade;
use crate::services::telegram::menu;
use crate::services::telegram::callback_router;
use std::sync::Arc;
use log::{info, debug, error};

/// Обработчик сообщений от пользователей
pub async fn handle_message(
    bot: Bot,
    msg: Message,
    db: Arc<DbFacade>,
) -> Result<(), teloxide::RequestError> {
    let telegram_id = match msg.from {
        Some(user) => user.id.0 as i64,
        None => {
            debug!("handle_message: сообщение без отправителя, пропускаем");
            return Ok(());
        }
    };
    debug!("handle_message: получен запрос от пользователя {}", telegram_id);

    // Проверяем, зарегистрирован ли пользователь
    match db.find_student(telegram_id).await {
        Ok(Some(student)) => {
            info!("Пользователь {} найден: {}", telegram_id, student.group_name);

            // Приветствие с меню зарегистрированного пользователя
            bot.send_message(
                msg.chat.id,
                format!("👋 Привет, {}!\n\n📚 Выберите действие из меню ниже:", student.group_name)
            )
            .reply_markup(menu::user_menu_inline_keyboard())
            .await?;
        }
        Ok(None) => {
            debug!("Пользователь {} не найден, предлагаем регистрацию", telegram_id);

            // Приветствие с предложением регистрации
            bot.send_message(
                msg.chat.id,
                "👋 Привет! Ты ещё не зарегистрирован.\n\n📝 Пройди регистрацию, чтобы получить доступ к расписанию!"
            )
            .reply_markup(menu::main_menu_inline_keyboard())
            .await?;
        }
        Err(err) => {
            error!("Ошибка поиска студента {}: {:?}", telegram_id, err);
            bot.send_message(msg.chat.id, "❌ Ошибка при обращении к базе. Попробуйте позже.").await?;
        }
    }
    Ok(())
}

/// Обработчик callback-запросов (нажатия кнопок)
/// Делегирует обработку callback_router'у для централизованной маршрутизации
pub async fn handle_callback(
    bot: Bot,
    q: CallbackQuery,
    db: Arc<DbFacade>,
) -> Result<(), teloxide::RequestError> {
    callback_router::route_callback(bot, q, db).await
}
