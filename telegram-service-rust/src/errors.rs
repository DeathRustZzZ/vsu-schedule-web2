// src/errors.rs
//! Типы ошибок приложения

use std::fmt;

/// Ошибки приложения
#[derive(Debug)]
pub enum AppError {
    /// Ошибка базы данных
    DatabaseError(String),
    /// Ошибка при работе с Telegram API
    TelegramError(String),
    /// Ошибка парсинга
    ParseError(String),
    /// Ошибка валидации
    ValidationError(String),
    /// Внутренняя ошибка сервера
    InternalError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(msg) => write!(f, "Ошибка БД: {}", msg),
            AppError::TelegramError(msg) => write!(f, "Ошибка Telegram: {}", msg),
            AppError::ParseError(msg) => write!(f, "Ошибка парсинга: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Ошибка валидации: {}", msg),
            AppError::InternalError(msg) => write!(f, "Внутренняя ошибка: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::InternalError(err.to_string())
    }
}

