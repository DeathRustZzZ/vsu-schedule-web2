use std::sync::Arc;

use log::{info, warn};
use teloxide::prelude::*;

use crate::bot::handlers::{callback, message};
use crate::bot::schedule_api::ScheduleApi;
use crate::db::facade::DbFacade;

/// Запуск Telegram-бота + подключение внешнего Schedule API.
///
/// Архитектурная идея:
/// - `DbFacade` и `ScheduleApi` заворачиваем в `Arc`, потому что обработчики dptree будут клонироваться.
/// - сам `Bot` остаётся "лёгким" хэндлом, teloxide его копирует без проблем.
///
/// Диагностически важно логировать:
/// - старт бота
/// - факт инициализации внешнего API (base_url логируется внутри ScheduleApi::new)
pub async fn run(
    bot: Bot,
    db: DbFacade,
    schedule_api_base: String,
    schedule_tz_offset_seconds: i32,
) {
    info!("Запуск Telegram-бота...");

    let db = Arc::new(db);
    let schedule_api = Arc::new(ScheduleApi::new(schedule_api_base));

    let handler = dptree::entry()
        // Обработка обычных сообщений
        .branch(Update::filter_message().endpoint({
            let db = Arc::clone(&db);
            let schedule_api = Arc::clone(&schedule_api);
            move |bot: Bot, msg: Message| {
                let db = Arc::clone(&db);
                let schedule_api = Arc::clone(&schedule_api);
                async move {
                    crate::bot::error_handler::handle_message_safe(
                        bot.clone(),
                        msg.clone(),
                        |b, m| async move { message::handle_message(b, m, db, schedule_api).await },
                    )
                    .await
                }
            }
        }))
        // Обработка callback (кнопок)
        .branch(Update::filter_callback_query().endpoint({
            let db = Arc::clone(&db);
            let schedule_api = Arc::clone(&schedule_api);
            move |bot: Bot, q: CallbackQuery| {
                let db = Arc::clone(&db);
                let schedule_api = Arc::clone(&schedule_api);
                async move {
                    crate::bot::error_handler::handle_callback_safe(
                        bot.clone(),
                        q.clone(),
                        |b, query| async move {
                            callback::handle_callback(
                                b,
                                query,
                                db,
                                schedule_api,
                                schedule_tz_offset_seconds,
                            )
                            .await
                        },
                    )
                    .await
                }
            }
        }));

    info!("Dispatcher initialized. Waiting for updates…");
    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    warn!("Dispatcher stopped. Bot runtime finished.");
}
