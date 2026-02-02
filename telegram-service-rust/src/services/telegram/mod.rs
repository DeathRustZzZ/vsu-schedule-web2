//src/services/telegram/mod.rs
pub mod handlers;
pub mod keyboards;
pub mod menu;
pub mod menu_dispatcher;
pub mod menu_state_manager;
pub mod menu_examples;
pub mod callback_router;
pub mod registration;

use teloxide::prelude::*;
use crate::db::facade::DbFacade;
use crate::services::telegram::handlers::{handle_message, handle_callback};
use std::sync::Arc; // Arc — "умный указатель" для безопасного совместного использования объекта
use log::{info, debug};

/// Запускаем Telegram-бота.
/// - `bot`: объект бота с токеном.
/// - `db`: фасад базы данных, обёрнутый в Arc для безопасного шаринга между хендлерами.
pub async fn run_telegram_bot(bot: Bot, db: DbFacade) {
    info!("Запуск Telegram-бота...");

    // Оборачиваем DbFacade в Arc, чтобы можно было клонировать "ссылку" на него,
    // а не сам объект. Это дешёвая операция: увеличивается счётчик ссылок.
    let db = Arc::new(db);

    let handler = dptree::entry()
        // Хендлер для входящих сообщений
        .branch(Update::filter_message().endpoint({
            let db = Arc::clone(&db); // создаём копию Arc для этого замыкания
            move |bot: Bot, msg: Message| {
                let db = Arc::clone(&db); // внутри тоже клонируем Arc
                async move {
                    debug!("Получено сообщение от пользователя: {:?}", msg.from);
                    handle_message(bot, msg, db).await
                }
            }
        }))
        // Хендлер для callback-запросов (нажатия кнопок)
        .branch(Update::filter_callback_query().endpoint({
            let db = Arc::clone(&db); // отдельная копия Arc для второго замыкания
            move |bot: Bot, q: CallbackQuery| {
                let db = Arc::clone(&db);
                async move {
                    debug!("Получен callback-запрос: {:?}", q.data);
                    handle_callback(bot, q, db).await
                }
            }
        }));

    // Dispatcher — основной цикл обработки событий.
    // Enable_ctrlc_handler позволяет корректно завершить работу по Ctrl+C.
    info!("Инициализация диспетчера...");
    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

}
