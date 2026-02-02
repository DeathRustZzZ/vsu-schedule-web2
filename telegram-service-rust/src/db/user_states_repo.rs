// src/db/user_states_repo.rs
//! Репозиторий для управления состоянием пользователей

use sqlx::PgPool;
use crate::domain::user_state::UserState;
use log::debug;

/// Найти состояние пользователя по Telegram ID
pub async fn find_by_telegram_id(pool: &PgPool, telegram_id: i64) -> anyhow::Result<Option<UserState>> {
    debug!("Поиск состояния пользователя telegram_id={}", telegram_id);
    
    let state = sqlx::query_as::<_, UserState>(
        "SELECT id, telegram_id, state, faculty, study_form, course, ui_chat_id, ui_message_id, created_at, updated_at 
         FROM user_states WHERE telegram_id = $1"
    )
    .bind(telegram_id)
    .fetch_optional(pool)
    .await?;

    Ok(state)
}

/// Создать новое состояние пользователя
pub async fn insert(pool: &PgPool, telegram_id: i64) -> anyhow::Result<UserState> {
    debug!("Создание состояния для пользователя telegram_id={}", telegram_id);
    
    let state = sqlx::query_as::<_, UserState>(
        "INSERT INTO user_states (telegram_id, state)
         VALUES ($1, 'idle')
         RETURNING id, telegram_id, state, faculty, study_form, course, ui_chat_id, ui_message_id, created_at, updated_at"
    )
    .bind(telegram_id)
    .fetch_one(pool)
    .await?;

    Ok(state)
}

/// Сбросить состояние пользователя (новая регистрация)
pub async fn reset(pool: &PgPool, telegram_id: i64) -> anyhow::Result<UserState> {
    debug!("Сброс состояния пользователя telegram_id={}", telegram_id);

    let updated = sqlx::query_as::<_, UserState>(
        "UPDATE user_states
         SET state = 'idle',
             faculty = NULL,
             study_form = NULL,
             course = NULL,
             updated_at = NOW()
         WHERE telegram_id = $1
         RETURNING id, telegram_id, state, faculty, study_form, course, ui_chat_id, ui_message_id, created_at, updated_at"
    )
    .bind(telegram_id)
    .fetch_optional(pool)
    .await?;

    if let Some(state) = updated {
        return Ok(state);
    }

    let inserted = sqlx::query_as::<_, UserState>(
        "INSERT INTO user_states (telegram_id, state)
         VALUES ($1, 'idle')
         RETURNING id, telegram_id, state, faculty, study_form, course, ui_chat_id, ui_message_id, created_at, updated_at"
    )
    .bind(telegram_id)
    .fetch_one(pool)
    .await?;

    Ok(inserted)
}

/// Обновить состояние пользователя
pub async fn update_state(pool: &PgPool, telegram_id: i64, state: &str) -> anyhow::Result<UserState> {
    debug!("Обновление состояния пользователя telegram_id={} на {}", telegram_id, state);
    
    let updated = sqlx::query_as::<_, UserState>(
        "UPDATE user_states SET state = $1, updated_at = NOW()
         WHERE telegram_id = $2
         RETURNING id, telegram_id, state, faculty, study_form, course, ui_chat_id, ui_message_id, created_at, updated_at"
    )
    .bind(state)
    .bind(telegram_id)
    .fetch_one(pool)
    .await?;

    Ok(updated)
}

/// Обновить факультет пользователя
pub async fn update_faculty(pool: &PgPool, telegram_id: i64, faculty: &str) -> anyhow::Result<UserState> {
    debug!("Обновление факультета пользователя telegram_id={} на {}", telegram_id, faculty);
    
    let updated = sqlx::query_as::<_, UserState>(
        "INSERT INTO user_states (telegram_id, state, faculty)
         VALUES ($2, 'idle', $1)
         ON CONFLICT (telegram_id)
         DO UPDATE SET faculty = EXCLUDED.faculty, updated_at = NOW()
         RETURNING id, telegram_id, state, faculty, study_form, course, ui_chat_id, ui_message_id, created_at, updated_at"
    )
    .bind(faculty)
    .bind(telegram_id)
    .fetch_one(pool)
    .await?;

    Ok(updated)
}

/// Обновить форму обучения пользователя
pub async fn update_study_form(pool: &PgPool, telegram_id: i64, study_form: &str) -> anyhow::Result<UserState> {
    debug!("Обновление формы обучения пользователя telegram_id={} на {}", telegram_id, study_form);
    
    let updated = sqlx::query_as::<_, UserState>(
        "INSERT INTO user_states (telegram_id, state, study_form)
         VALUES ($2, 'idle', $1)
         ON CONFLICT (telegram_id)
         DO UPDATE SET study_form = EXCLUDED.study_form, updated_at = NOW()
         RETURNING id, telegram_id, state, faculty, study_form, course, ui_chat_id, ui_message_id, created_at, updated_at"
    )
    .bind(study_form)
    .bind(telegram_id)
    .fetch_one(pool)
    .await?;

    Ok(updated)
}

/// Обновить курс пользователя
pub async fn update_course(pool: &PgPool, telegram_id: i64, course: &str) -> anyhow::Result<UserState> {
    debug!("Обновление курса пользователя telegram_id={} на {}", telegram_id, course);
    
    let updated = sqlx::query_as::<_, UserState>(
        "INSERT INTO user_states (telegram_id, state, course)
         VALUES ($2, 'idle', $1)
         ON CONFLICT (telegram_id)
         DO UPDATE SET course = EXCLUDED.course, updated_at = NOW()
         RETURNING id, telegram_id, state, faculty, study_form, course, ui_chat_id, ui_message_id, created_at, updated_at"
    )
    .bind(course)
    .bind(telegram_id)
    .fetch_one(pool)
    .await?;

    Ok(updated)
}

/// Удалить состояние пользователя
pub async fn delete(pool: &PgPool, telegram_id: i64) -> anyhow::Result<u64> {
    debug!("Удаление состояния пользователя telegram_id={}", telegram_id);
    
    let result = sqlx::query(
        "DELETE FROM user_states WHERE telegram_id = $1"
    )
    .bind(telegram_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

/// Сохранить message_id для UI-сообщения
pub async fn update_ui_message(
    pool: &PgPool,
    telegram_id: i64,
    chat_id: i64,
    message_id: i32,
) -> anyhow::Result<UserState> {
    debug!(
        "Сохранение UI message_id для пользователя telegram_id={} (chat_id={}, message_id={})",
        telegram_id, chat_id, message_id
    );

    let updated = sqlx::query_as::<_, UserState>(
        "INSERT INTO user_states (telegram_id, state, ui_chat_id, ui_message_id)
         VALUES ($1, 'idle', $2, $3)
         ON CONFLICT (telegram_id)
         DO UPDATE SET ui_chat_id = EXCLUDED.ui_chat_id, ui_message_id = EXCLUDED.ui_message_id, updated_at = NOW()
         RETURNING id, telegram_id, state, faculty, study_form, course, ui_chat_id, ui_message_id, created_at, updated_at"
    )
    .bind(telegram_id)
    .bind(chat_id)
    .bind(message_id)
    .fetch_one(pool)
    .await?;

    Ok(updated)
}
