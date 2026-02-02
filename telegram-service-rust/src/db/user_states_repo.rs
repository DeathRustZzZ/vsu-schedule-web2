use sqlx::PgPool;
use crate::domain::user_state::UserState;
use log::debug;

pub async fn find_by_telegram_id(
    pool: &PgPool,
    telegram_id: i64,
) -> anyhow::Result<Option<UserState>> {
    debug!("Поиск состояния пользователя telegram_id={}", telegram_id);

    let state = sqlx::query_as::<_, UserState>(
        "SELECT id, telegram_id, state, faculty, study_form, course, ui_chat_id, ui_message_id, reply_message_id, reply_state, created_at, updated_at
         FROM user_states WHERE telegram_id = $1",
    )
    .bind(telegram_id)
    .fetch_optional(pool)
    .await?;

    Ok(state)
}

pub async fn upsert_state(
    pool: &PgPool,
    telegram_id: i64,
    state: &str,
) -> anyhow::Result<UserState> {
    debug!("Обновление состояния пользователя telegram_id={} на {}", telegram_id, state);

    let updated = sqlx::query_as::<_, UserState>(
        "INSERT INTO user_states (telegram_id, state)
         VALUES ($1, $2)
         ON CONFLICT (telegram_id)
         DO UPDATE SET state = EXCLUDED.state, updated_at = NOW()
         RETURNING id, telegram_id, state, faculty, study_form, course, ui_chat_id, ui_message_id, reply_message_id, reply_state, created_at, updated_at",
    )
    .bind(telegram_id)
    .bind(state)
    .fetch_one(pool)
    .await?;

    Ok(updated)
}

pub async fn clear_registration(
    pool: &PgPool,
    telegram_id: i64,
    state: &str,
) -> anyhow::Result<UserState> {
    debug!("Очистка регистрации пользователя telegram_id={}", telegram_id);

    let updated = sqlx::query_as::<_, UserState>(
        "INSERT INTO user_states (telegram_id, state, faculty, study_form, course)
         VALUES ($1, $2, NULL, NULL, NULL)
         ON CONFLICT (telegram_id)
         DO UPDATE SET state = EXCLUDED.state, faculty = NULL, study_form = NULL, course = NULL, updated_at = NOW()
         RETURNING id, telegram_id, state, faculty, study_form, course, ui_chat_id, ui_message_id, reply_message_id, reply_state, created_at, updated_at",
    )
    .bind(telegram_id)
    .bind(state)
    .fetch_one(pool)
    .await?;

    Ok(updated)
}

pub async fn update_faculty(
    pool: &PgPool,
    telegram_id: i64,
    faculty: &str,
    state: &str,
) -> anyhow::Result<UserState> {
    debug!("Обновление факультета пользователя telegram_id={}", telegram_id);

    let updated = sqlx::query_as::<_, UserState>(
        "INSERT INTO user_states (telegram_id, state, faculty)
         VALUES ($1, $2, $3)
         ON CONFLICT (telegram_id)
         DO UPDATE SET state = EXCLUDED.state, faculty = EXCLUDED.faculty, updated_at = NOW()
         RETURNING id, telegram_id, state, faculty, study_form, course, ui_chat_id, ui_message_id, reply_message_id, reply_state, created_at, updated_at",
    )
    .bind(telegram_id)
    .bind(state)
    .bind(faculty)
    .fetch_one(pool)
    .await?;

    Ok(updated)
}

pub async fn update_study_form(
    pool: &PgPool,
    telegram_id: i64,
    study_form: &str,
    state: &str,
) -> anyhow::Result<UserState> {
    debug!("Обновление формы обучения пользователя telegram_id={}", telegram_id);

    let updated = sqlx::query_as::<_, UserState>(
        "INSERT INTO user_states (telegram_id, state, study_form)
         VALUES ($1, $2, $3)
         ON CONFLICT (telegram_id)
         DO UPDATE SET state = EXCLUDED.state, study_form = EXCLUDED.study_form, updated_at = NOW()
         RETURNING id, telegram_id, state, faculty, study_form, course, ui_chat_id, ui_message_id, reply_message_id, reply_state, created_at, updated_at",
    )
    .bind(telegram_id)
    .bind(state)
    .bind(study_form)
    .fetch_one(pool)
    .await?;

    Ok(updated)
}

pub async fn update_course(
    pool: &PgPool,
    telegram_id: i64,
    course: &str,
    state: &str,
) -> anyhow::Result<UserState> {
    debug!("Обновление курса пользователя telegram_id={}", telegram_id);

    let updated = sqlx::query_as::<_, UserState>(
        "INSERT INTO user_states (telegram_id, state, course)
         VALUES ($1, $2, $3)
         ON CONFLICT (telegram_id)
         DO UPDATE SET state = EXCLUDED.state, course = EXCLUDED.course, updated_at = NOW()
         RETURNING id, telegram_id, state, faculty, study_form, course, ui_chat_id, ui_message_id, reply_message_id, reply_state, created_at, updated_at",
    )
    .bind(telegram_id)
    .bind(state)
    .bind(course)
    .fetch_one(pool)
    .await?;

    Ok(updated)
}

pub async fn update_ui_message(
    pool: &PgPool,
    telegram_id: i64,
    chat_id: i64,
    message_id: i32,
) -> anyhow::Result<UserState> {
    debug!(
        "Сохранение UI message_id telegram_id={} (chat_id={}, message_id={})",
        telegram_id, chat_id, message_id
    );

    let updated = sqlx::query_as::<_, UserState>(
        "INSERT INTO user_states (telegram_id, state, ui_chat_id, ui_message_id)
         VALUES ($1, 'idle', $2, $3)
         ON CONFLICT (telegram_id)
         DO UPDATE SET ui_chat_id = EXCLUDED.ui_chat_id, ui_message_id = EXCLUDED.ui_message_id, updated_at = NOW()
         RETURNING id, telegram_id, state, faculty, study_form, course, ui_chat_id, ui_message_id, reply_message_id, reply_state, created_at, updated_at",
    )
    .bind(telegram_id)
    .bind(chat_id)
    .bind(message_id)
    .fetch_one(pool)
    .await?;

    Ok(updated)
}
