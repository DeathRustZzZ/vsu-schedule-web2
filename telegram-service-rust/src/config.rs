// src/config.rs
//! Конфигурация приложения.
//!
//! Модуль отвечает **исключительно** за загрузку конфигурации
//! из переменных окружения. Здесь нет бизнес-логики — только
//! сбор и валидация входных параметров приложения.

use std::env;

/// Конфигурация приложения.
///
/// Структура используется на этапе старта приложения
/// и далее передаётся в инициализацию инфраструктурных компонентов
/// (БД, Telegram-бот и т.д.).
#[derive(Clone, Debug)]
pub struct AppConfig {
    /// Токен Telegram-бота.
    ///
    /// Критически важный параметр:
    /// без него приложение не имеет смысла продолжать работу.
    pub bot_token: String,

    /// URL подключения к базе данных.
    ///
    /// Используется при инициализации пула соединений.
    /// Неверное значение приведёт к падению приложения на старте,
    /// что является корректным поведением для backend-сервиса.
    pub database_url: String,

    /// Базовый URL API расписания.
    ///
    /// Используется ботом для запроса расписания через gateway.
    pub schedule_api_base: String,

    /// Смещение часового пояса для расписания (в секундах).
    ///
    /// Нужно, чтобы "сегодня" считалось по времени вуза, а не по времени сервера.
    pub schedule_tz_offset_seconds: i32,
}

impl AppConfig {
    /// Загрузить конфигурацию из переменных окружения.
    ///
    /// Алгоритм работы:
    /// 1. Пытаемся загрузить `.env` файл (если он существует)
    /// 2. Читаем обязательные переменные окружения
    /// 3. Падаем немедленно, если конфигурация некорректна
    ///
    /// Такое поведение намеренное:
    /// приложение не должно стартовать в полурабочем состоянии.
    pub fn load_from_environment() -> Self {
        // Загружаем переменные окружения из .env файла.
        //
        // В продакшене .env обычно отсутствует,
        // поэтому ошибку здесь игнорируем сознательно.
        dotenvy::dotenv().ok();
        log::debug!("environment variables loaded from .env (if present)");

        // Загрузка токена Telegram-бота.
        //
        // Используем expect, потому что:
        //  - без токена приложение бесполезно
        //  - ошибка конфигурации должна быть обнаружена сразу
        let bot_token = env::var("BOT_TOKEN").expect("environment variable BOT_TOKEN is not set");
        log::debug!("BOT_TOKEN successfully loaded");

        // Загрузка строки подключения к базе данных.
        //
        // Аналогично BOT_TOKEN — это обязательный параметр.
        let database_url =
            env::var("DATABASE_URL").expect("environment variable DATABASE_URL is not set");
        log::debug!("DATABASE_URL successfully loaded");

        let schedule_api_base =
            env::var("SCHEDULE_API_BASE").unwrap_or_else(|_| "http://api-gateway:8765".to_string());
        log::debug!("SCHEDULE_API_BASE successfully loaded");

        let schedule_tz_offset_seconds = parse_timezone_offset(
            &env::var("SCHEDULE_TZ_OFFSET").unwrap_or_else(|_| "+03:00".to_string()),
        );
        log::debug!("SCHEDULE_TZ_OFFSET successfully loaded");

        log::info!("application configuration loaded successfully");

        Self {
            bot_token,
            database_url,
            schedule_api_base,
            schedule_tz_offset_seconds,
        }
    }
}

fn parse_timezone_offset(value: &str) -> i32 {
    let trimmed = value.trim();
    let sign = if trimmed.starts_with('-') { -1 } else { 1 };
    let clean = trimmed.trim_start_matches(['+', '-']);
    let mut parts = clean.split(':');
    let hours = parts
        .next()
        .and_then(|v| v.parse::<i32>().ok())
        .unwrap_or(0);
    let minutes = parts
        .next()
        .and_then(|v| v.parse::<i32>().ok())
        .unwrap_or(0);
    sign * (hours * 3600 + minutes * 60)
}
