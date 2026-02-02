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

    /// Проверить, заполнены ли все необходимые данные для регистрации
    pub fn is_complete(&self) -> bool {
        self.faculty.is_some() && self.study_form.is_some() && self.course.is_some()
    }
}

