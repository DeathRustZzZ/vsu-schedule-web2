use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardMarkup, MessageId};

use crate::db::facade::DbFacade;
use log::{debug, info, warn};

/// Приводим `ChatId` к примитивному типу для хранения в БД.
///
/// Почему так:
/// - `ChatId` — обёртка вокруг i64
/// - в таблицах проще хранить i64
/// - здесь мы фиксируем контракт “UI-message привязан к chat_id + message_id”
fn fallback_chat_id(chat_id: ChatId) -> i64 {
    chat_id.0
}

/// Рендер одного "экрана" UI (сообщение + опциональная inline-клавиатура).
///
/// Алгоритм (важно понимать порядок):
/// 1) Если `preferred_message_id` задан — пытаемся редактировать именно его.
///    Это быстрый путь: обычно callback пришёл с конкретного сообщения.
/// 2) Иначе пытаемся найти в БД "последнее UI-сообщение" и отредактировать его
///    (но только если оно относится к этому же chat_id).
/// 3) Если редактирование не удалось — отправляем новое сообщение и сохраняем его id в БД.
///
/// Почему так сделано:
/// - Telegram не всегда позволяет edit (старые сообщения, отсутствие прав, message not found, etc.)
/// - при ошибках edit мы не должны "ломать" UX — лучше отправить новое сообщение.
pub async fn render_screen(
    bot: &Bot,
    db: &DbFacade,
    telegram_id: i64,
    chat_id: ChatId,
    preferred_message_id: Option<MessageId>,
    text: &str,
    markup: Option<InlineKeyboardMarkup>,
) -> Result<(), teloxide::RequestError> {
    debug!(
        "render_screen: user={} chat_id={} preferred_message_id={:?} text_len={} has_markup={}",
        telegram_id,
        chat_id.0,
        preferred_message_id.map(|m| m.0),
        text.len(),
        markup.is_some()
    );

    // 1) Сначала пробуем редактировать конкретное сообщение, если оно передано.
    // Это лучший сценарий: минимум обращений к БД и максимально предсказуемый UX.
    if let Some(msg_id) = preferred_message_id {
        debug!(
            "render_screen: trying preferred edit (chat_id={} msg_id={})",
            chat_id.0,
            msg_id.0
        );

        if try_edit(bot, chat_id, msg_id, text, markup.clone()).await {
            debug!("render_screen: preferred edit succeeded (msg_id={})", msg_id.0);

            // Сохраняем UI-message в БД, чтобы в будущем можно было редактировать его без preferred id.
            //
            // Ошибку намеренно игнорируем (как и в исходнике), чтобы UI не падал из-за проблем БД.
            // Но при расследованиях можно поднять уровень логирования в DbFacade.
            let _ = db
                .set_ui_message(telegram_id, fallback_chat_id(chat_id), msg_id.0)
                .await;

            return Ok(());
        } else {
            debug!(
                "render_screen: preferred edit failed (msg_id={}), fallback to db lookup/send",
                msg_id.0
            );
        }
    }

    // 2) Если preferred не сработал или не был задан — смотрим в БД,
    // есть ли сохранённое UI-сообщение для этого пользователя.
    //
    // Важно:
    // - мы редактируем UI-сообщение только если оно в том же чате
    // - иначе можно случайно пытаться редактировать сообщение из другого диалога (группа/личка)
    if let Ok(Some((ui_chat_id, ui_message_id))) = db.get_ui_message(telegram_id).await {
        debug!(
            "render_screen: found ui_message in db (user={} ui_chat_id={} ui_message_id={})",
            telegram_id, ui_chat_id, ui_message_id
        );

        if ui_chat_id == fallback_chat_id(chat_id) {
            let existing_id = MessageId(ui_message_id);

            debug!(
                "render_screen: trying db-stored edit (chat_id={} msg_id={})",
                chat_id.0, existing_id.0
            );

            if try_edit(bot, chat_id, existing_id, text, markup.clone()).await {
                debug!(
                    "render_screen: db-stored edit succeeded (msg_id={})",
                    existing_id.0
                );
                return Ok(());
            } else {
                debug!(
                    "render_screen: db-stored edit failed (msg_id={}), will send new message",
                    existing_id.0
                );
            }
        } else {
            // Это не ошибка: пользователь мог начать диалог в другом чате (личка/группа).
            debug!(
                "render_screen: ui_message chat mismatch (db_chat_id={}, current_chat_id={})",
                ui_chat_id,
                chat_id.0
            );
        }
    } else {
        debug!("render_screen: no ui_message in db (user={})", telegram_id);
    }

    // 3) Последний fallback — отправляем новое сообщение.
    //
    // Это гарантирует, что пользователь увидит ответ даже если:
    // - edit запрещён
    // - сообщение удалено
    // - устарело
    // - БД временно недоступна
    info!(
        "render_screen: sending new message (user={} chat_id={})",
        telegram_id, chat_id.0
    );

    let mut req = bot.send_message(chat_id, text);
    if let Some(kb) = markup {
        req = req.reply_markup(kb);
    }

    let sent = req.await?;

    debug!(
        "render_screen: message sent (chat_id={} msg_id={})",
        chat_id.0,
        sent.id.0
    );

    let _ = db
        .set_ui_message(telegram_id, fallback_chat_id(chat_id), sent.id.0)
        .await;

    Ok(())
}

/// Попытка отредактировать существующее сообщение.
///
/// Возвращает `true`, если edit удался.
///
/// Важно:
/// Мы намеренно "гасим" ошибку и возвращаем bool, потому что для UI
/// редактирование — оптимизация UX, а не обязательный шаг.
/// Если edit не вышел — всегда есть fallback на отправку нового сообщения.
async fn try_edit(
    bot: &Bot,
    chat_id: ChatId,
    message_id: MessageId,
    text: &str,
    markup: Option<InlineKeyboardMarkup>,
) -> bool {
    debug!(
        "try_edit: chat_id={} msg_id={} text_len={} has_markup={}",
        chat_id.0,
        message_id.0,
        text.len(),
        markup.is_some()
    );

    let mut req = bot.edit_message_text(chat_id, message_id, text);
    if let Some(kb) = markup {
        req = req.reply_markup(kb);
    }

    match req.await {
        Ok(_) => {
            debug!("try_edit: success (msg_id={})", message_id.0);
            true
        }
        Err(err) => {
            // warn — потому что это деградация UX (редактировать не смогли),
            // но не фатальная ошибка сервиса.
            //
            // Типовые причины:
            // - message to edit not found (удалили/другое сообщение)
            // - message can't be edited (слишком старое, другое содержимое)
            // - bot was blocked by user
            warn!(
                "try_edit: failed to edit message {} in chat {}: {:?}",
                message_id.0,
                chat_id.0,
                err
            );
            false
        }
    }
}
