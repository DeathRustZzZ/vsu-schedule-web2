// Подмодули слоя Telegram-бота.
//
// callbacks — обработчики inline / callback query
// keyboards — фабрики клавиатур (reply / inline)
// router    — маршрутизация апдейтов (commands, messages, callbacks)
// ui        — текстовые ответы и форматирование сообщений
pub mod callbacks;
pub mod keyboards;
pub mod router;
pub mod ui;

use crate::db::facade::DbFacade;
use sqlx::PgPool;
use teloxide::prelude::*;

/// Точка входа Telegram-бота.
///
/// Функция выполняет:
/// 1. Инициализацию фасада работы с БД
/// 2. Передачу управления роутеру апдейтов
///
/// Важно:
/// - здесь **нет** бизнес-логики
/// - функция выступает как glue-код между инфраструктурой (БД, Telegram)
///   и логикой обработки апдейтов
pub async fn run_bot(bot: Bot, pool: PgPool) {
    log::info!("bot initialization started");

    // Создаём фасад доступа к базе данных.
    //
    // DbFacade инкапсулирует sqlx и скрывает детали работы с БД
    // от слоя Telegram-бота.
    // Это упрощает тестирование и снижает связность модулей.
    log::debug!("initializing database facade");
    let db = DbFacade::new(pool);
    log::debug!("database facade initialized");

    // Передаём управление роутеру.
    //
    // router::run обычно содержит:
    //  - регистрацию обработчиков команд
    //  - обработку callback query
    //  - запуск polling / webhook
    //
    // Эта функция, как правило, блокирует поток выполнения
    // до завершения работы бота.
    log::info!("starting update router");
    router::run(bot, db).await;

    // Если выполнение дошло сюда — значит роутер завершил работу.
    // В нормальном режиме это происходит только при shutdown
    // или фатальной ошибке внутри router::run.
    log::warn!("update router stopped, bot is shutting down");
}