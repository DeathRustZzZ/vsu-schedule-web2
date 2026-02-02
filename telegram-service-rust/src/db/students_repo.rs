use sqlx::PgPool;
use log::{info, debug, error};
use crate::domain::student::Student;

/// Найти студента по Telegram ID
pub async fn find_by_telegram_id(pool: &PgPool, telegram_id: i64) -> anyhow::Result<Option<Student>> {
    debug!("Запрос на поиск студента с telegram_id={}", telegram_id);

    let row = sqlx::query_as::<_, Student>(
        r#"
        SELECT id, telegram_id, faculty, group_name, study_form, created_at
        FROM students
        WHERE telegram_id = $1
        "#
    )
    .bind(telegram_id)
    .fetch_optional(pool)
    .await;

    match row {
        Ok(Some(student)) => {
            info!("Студент найден: id={}, telegram_id={}", student.id, student.telegram_id);
            Ok(Some(student))
        }
        Ok(None) => {
            info!("Студент с telegram_id={} не найден.", telegram_id);
            Ok(None)
        }
        Err(e) => {
            error!("Ошибка при поиске студента: {:?}", e);
            Err(e.into())
        }
    }
}

/// Добавить нового студента
pub async fn insert(
    pool: &PgPool,
    telegram_id: i64,
    faculty: &str,
    group_name: &str,
    study_form: &str,
) -> anyhow::Result<Student> {
    info!(
        "Регистрация нового студента: telegram_id={}, faculty={}, group={}, study_form={}",
        telegram_id, faculty, group_name, study_form
    );

    let res = sqlx::query_as::<_, Student>(
        r#"
        INSERT INTO students (telegram_id, faculty, group_name, study_form)
        VALUES ($1, $2, $3, $4)
        RETURNING id, telegram_id, faculty, group_name, study_form, created_at
        "#
    )
    .bind(telegram_id)
    .bind(faculty)
    .bind(group_name)
    .bind(study_form)
    .fetch_one(pool)
    .await;

    match res {
        Ok(student) => {
            info!("Студент успешно добавлен: id={}, telegram_id={}", student.id, student.telegram_id);
            Ok(student)
        }
        Err(e) => {
            error!("Ошибка при добавлении студента: {:?}", e);
            Err(e.into())
        }
    }
}

/// Обновить данные студента (факультет, группа, форма обучения)
pub async fn update(
    pool: &PgPool,
    telegram_id: i64,
    faculty: &str,
    group_name: &str,
    study_form: &str,
) -> anyhow::Result<Option<Student>> {
    info!("Обновление студента telegram_id={}", telegram_id);

    let res = sqlx::query_as::<_, Student>(
        r#"
        UPDATE students
        SET faculty = $2, group_name = $3, study_form = $4
        WHERE telegram_id = $1
        RETURNING id, telegram_id, faculty, group_name, study_form, created_at
        "#
    )
    .bind(telegram_id)
    .bind(faculty)
    .bind(group_name)
    .bind(study_form)
    .fetch_optional(pool)
    .await;

    match res {
        Ok(Some(student)) => {
            info!("Студент обновлён: id={}, telegram_id={}", student.id, student.telegram_id);
            Ok(Some(student))
        }
        Ok(None) => {
            info!("Студент с telegram_id={} не найден для обновления.", telegram_id);
            Ok(None)
        }
        Err(e) => {
            error!("Ошибка при обновлении студента: {:?}", e);
            Err(e.into())
        }
    }
}

/// Удалить студента по Telegram ID
pub async fn delete(pool: &PgPool, telegram_id: i64) -> anyhow::Result<u64> {
    info!("Удаление студента telegram_id={}", telegram_id);

    let res = sqlx::query(
        r#"
        DELETE FROM students
        WHERE telegram_id = $1
        "#
    )
    .bind(telegram_id)
    .execute(pool)
    .await;

    match res {
        Ok(result) => {
            info!("Удалено {} записей.", result.rows_affected());
            Ok(result.rows_affected())
        }
        Err(e) => {
            error!("Ошибка при удалении студента: {:?}", e);
            Err(e.into())
        }
    }
}
