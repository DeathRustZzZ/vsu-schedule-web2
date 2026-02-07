// src/bot/error_handler.rs

use log::{error, warn};
use teloxide::prelude::*;

/// Обертка для безопасной обработки сообщений
pub async fn handle_message_safe<F, Fut>(
    bot: Bot,
    msg: Message,
    handler: F,
) -> ResponseResult<()>
where
    F: FnOnce(Bot, Message) -> Fut,
    Fut: std::future::Future<Output = ResponseResult<()>>,
{
    // Запускаем настоящий обработчик
    match handler(bot.clone(), msg.clone()).await {
        Ok(_) => Ok(()),
        Err(e) => {
            // Произошла ошибка!
            error!(
                "Error handling message from user {}: {:?}",
                msg.from.as_ref().map(|u| u.id.0).unwrap_or(0),
                e
            );

            // Пытаемся сообщить пользователю
            if let Err(send_err) = bot
                .send_message(
                    msg.chat.id,
                    "⚠️ Произошла ошибка. Попробуйте позже или напишите /start",
                )
                .await
            {
                warn!("Failed to send error message: {:?}", send_err);
            }

            Err(e)
        }
    }
}

/// Обертка для безопасной обработки callback-кнопок
pub async fn handle_callback_safe<F, Fut>(
    bot: Bot,
    q: CallbackQuery,
    handler: F,
) -> ResponseResult<()>
where
    F: FnOnce(Bot, CallbackQuery) -> Fut,
    Fut: std::future::Future<Output = ResponseResult<()>>,
{
    match handler(bot.clone(), q.clone()).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!(
                "Error handling callback from user {}: {:?}",
                q.from.id.0,
                e
            );

            // Убираем "часики" на кнопке
            // ИСПРАВЛЕНО: убрали & перед q.id
            let _ = bot.answer_callback_query(q.id).await;

            // Отправляем сообщение об ошибке
            if let Some(msg) = q.message {
                // ИСПРАВЛЕНО: используем chat() вместо chat
                let chat_id = match msg {
                    teloxide::types::MaybeInaccessibleMessage::Regular(m) => m.chat.id,
                    teloxide::types::MaybeInaccessibleMessage::Inaccessible(m) => m.chat.id,
                };

                let _ = bot
                    .send_message(
                        chat_id,
                        "⚠️ Произошла ошибка. Попробуйте позже или напишите /start",
                    )
                    .await;
            }

            Err(e)
        }
    }
}