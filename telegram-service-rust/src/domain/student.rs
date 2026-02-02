//src/domain/student.rs
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Student {
    pub id: Uuid,
    pub telegram_id: i64,
    pub faculty: String,
    pub group_name: String,
    pub study_form: String,
    pub created_at: DateTime<Utc>,
}
