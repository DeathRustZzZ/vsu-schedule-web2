// src/main.rs

// Модули приложения.
// Декларируются в корне, чтобы main выступал как точка сборки всего приложения,
// а не содержал бизнес-логику.
pub mod bot;
pub mod config;
pub mod db;
pub mod domain;

use crate::bot::run_bot;
use crate::config::AppConfig;
use crate::db::connection::init_pool;

#[tokio::main]
async fn main() {
    // Инициализация логгера.
    //
    // pretty_env_logger читает переменную окружения RUST_LOG
    // и позволяет управлять уровнем логов без перекомпиляции:
    //   RUST_LOG=info
    //   RUST_LOG=debug
    //   RUST_LOG=sqlx=warn,bot=debug
    //
    // Важно вызывать как можно раньше, до любых логов.
    pretty_env_logger::init();

    log::info!("application startup");

    // Загрузка конфигурации приложения из переменных окружения.
    //
    // На этом этапе чаще всего ловятся ошибки:
    //  - отсутствует BOT_TOKEN
    //  - неверный DATABASE_URL
    //
    // Поэтому логируем сам факт успешной загрузки.
    let config = AppConfig::load_from_environment();
    log::debug!("config loaded successfully");

    // Инициализация пула соединений с базой данных.
    //
    // Это критическая точка старта:
    //  - неверный URL
    //  - БД недоступна
    //  - превышен лимит соединений
    //
    // Если здесь произойдёт panic внутри init_pool — лог выше поможет понять,
    // что приложение упало именно на этапе подключения к БД.
    log::info!("initializing database connection pool");
    let pool = init_pool(&config.database_url).await;
    log::info!("database pool initialized");

    // Создание Telegram-бота.
    //
    // Здесь реальных сетевых операций ещё нет —
    // Bot::new только подготавливает клиент.
    // Ошибки на этом этапе возможны только при некорректном токене
    // и проявятся уже при первом запросе к Telegram API.
    log::info!("creating telegram bot instance");
    let bot = teloxide::Bot::new(config.bot_token.clone());
    log::debug!("telegram bot instance created");

    // Запуск основного цикла бота.
    //
    // Обычно это "вечная" асинхронная операция:
    //  - polling / webhook
    //  - обработка апдейтов
    //  - работа с БД
    //
    // Если выполнение дошло сюда — значит инициализация прошла успешно.
    // Если приложение завершится — причина почти наверняка внутри run_bot.
    log::info!("starting bot runtime");
    run_bot(bot, pool, config.schedule_api_base.clone()).await;

    // Эта строка будет достигнута только если run_bot завершится.
    // В нормальном режиме работы это либо:
    //  - корректный shutdown
    //  - фатальная ошибка, после которой run_bot решил завершиться
    log::warn!("bot runtime stopped, application is shutting down");
}
