// src/domain/user_state.rs
//! Модель состояния пользователя при регистрации

use sqlx::FromRow;

/// Состояние пользователя в процессе регистрации
/// Хранит на каком этапе регистрации находится пользователь и его выбранные данные
#[derive(Debug, Clone, FromRow)]
pub struct UserState {
    pub id: i32,
    pub telegram_id: i64,
    pub state: String,
    pub faculty: Option<String>,
    pub study_form: Option<String>,
    pub course: Option<String>,
    pub ui_chat_id: Option<i64>,
    pub ui_message_id: Option<i32>,
    pub reply_message_id: Option<i32>,
    pub reply_state: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl UserState {
    /// Получить факультет пользователя
    pub fn faculty(&self) -> Option<&str> {
        self.faculty.as_deref()
    }

    /// Получить форму обучения пользователя
    pub fn study_form(&self) -> Option<&str> {
        self.study_form.as_deref()
    }

    /// Получить курс пользователя
    pub fn course(&self) -> Option<&str> {
        self.course.as_deref()
    }
}
