use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardMarkup, MessageId};

use crate::db::facade::DbFacade;
use log::warn;

fn fallback_chat_id(chat_id: ChatId) -> i64 {
    chat_id.0
}

pub async fn render_screen(
    bot: &Bot,
    db: &DbFacade,
    telegram_id: i64,
    chat_id: ChatId,
    preferred_message_id: Option<MessageId>,
    text: &str,
    markup: Option<InlineKeyboardMarkup>,
) -> Result<(), teloxide::RequestError> {
    if let Some(msg_id) = preferred_message_id {
        if try_edit(bot, chat_id, msg_id, text, markup.clone()).await {
            let _ = db
                .set_ui_message(telegram_id, fallback_chat_id(chat_id), msg_id.0)
                .await;
            return Ok(());
        }
    }

    if let Ok(Some((ui_chat_id, ui_message_id))) = db.get_ui_message(telegram_id).await {
        if ui_chat_id == fallback_chat_id(chat_id) {
            let existing_id = MessageId(ui_message_id);
            if try_edit(bot, chat_id, existing_id, text, markup.clone()).await {
                return Ok(());
            }
        }
    }

    let mut req = bot.send_message(chat_id, text);
    if let Some(kb) = markup {
        req = req.reply_markup(kb);
    }
    let sent = req.await?;
    let _ = db
        .set_ui_message(telegram_id, fallback_chat_id(chat_id), sent.id.0)
        .await;
    Ok(())
}

async fn try_edit(
    bot: &Bot,
    chat_id: ChatId,
    message_id: MessageId,
    text: &str,
    markup: Option<InlineKeyboardMarkup>,
) -> bool {
    let mut req = bot.edit_message_text(chat_id, message_id, text);
    if let Some(kb) = markup {
        req = req.reply_markup(kb);
    }
    match req.await {
        Ok(_) => true,
        Err(err) => {
            warn!("Не удалось отредактировать сообщение {}: {:?}", message_id.0, err);
            false
        }
    }
}
