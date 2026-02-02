// src/config.rs
//! Конфигурация приложения

use log::info;
use std::env;

/// Конфигурация приложения
#[derive(Clone, Debug)]
pub struct AppConfig {
    /// Токен Telegram бота
    pub bot_token: String,
    /// URL базы данных
    pub database_url: String,
    /// Логический уровень
    pub log_level: String,
}

impl AppConfig {
    /// Загрузить конфигурацию из переменных окружения
    pub fn load_from_environment() -> Self {
        dotenvy::dotenv().ok();

        let bot_token = env::var("BOT_TOKEN")
            .expect("Переменная окружения BOT_TOKEN не установлена");
        let database_url = env::var("DATABASE_URL")
            .expect("Переменная окружения DATABASE_URL не установлена");
        let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

        info!("Конфигурация загружена: log_level={}", log_level);

        Self {
            bot_token,
            database_url,
            log_level,
        }
    }
}
