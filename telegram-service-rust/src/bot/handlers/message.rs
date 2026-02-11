use std::sync::Arc;

use log::{debug, info};
use teloxide::prelude::*;

use crate::bot::flows::menu::{show_help, show_main_menu};
use crate::bot::flows::registration::start_registration;
use crate::bot::schedule_api::ScheduleApi;
use crate::db::facade::DbFacade;

/// Обработка обычных сообщений.
/// Сейчас schedule_api не используется (параметр оставлен для будущей функциональности).
///
/// Важно логировать:
/// - кто пишет
/// - что за команда
/// - какой fallback сработал
pub async fn handle_message(
    bot: Bot,
    msg: Message,
    db: Arc<DbFacade>,
    _schedule_api: Arc<ScheduleApi>,
) -> Result<(), teloxide::RequestError> {
    let telegram_id = match msg.from {
        Some(ref user) => user.id.0 as i64,
        None => {
            debug!("message without sender (msg_id={:?}) ignored", msg.id);
            return Ok(());
        }
    };

    debug!(
        "message received: user={} chat_id={} msg_id={:?}",
        telegram_id, msg.chat.id.0, msg.id
    );

    if let Some(text) = msg.text() {
        let cmd = text.trim();
        debug!("text message from user {}: '{}'", telegram_id, cmd);

        match cmd {
            "/start" | "/menu" => {
                info!("command /menu triggered by user {}", telegram_id);
                show_main_menu(&bot, db.as_ref(), telegram_id, msg.chat.id, None).await?;
                return Ok(());
            }
            "/register" => {
                info!("command /register triggered by user {}", telegram_id);
                start_registration(&bot, db.as_ref(), telegram_id, msg.chat.id, None).await?;
                return Ok(());
            }
            "/help" => {
                info!("command /help triggered by user {}", telegram_id);
                show_help(&bot, db.as_ref(), telegram_id, msg.chat.id, None).await?;
                return Ok(());
            }
            _ => {
                // Это не ошибка — пользователь мог написать что угодно.
                // Мы просто возвращаем в меню, чтобы UX был предсказуемым.
                debug!("unknown command/text '{}' from user {}", cmd, telegram_id);
            }
        }
    }

    // Fallback: любое нераспознанное сообщение возвращает главное меню.
    show_main_menu(&bot, db.as_ref(), telegram_id, msg.chat.id, None).await?;
    Ok(())
}
