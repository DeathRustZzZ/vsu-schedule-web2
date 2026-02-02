use sqlx::PgPool;
use log::{info, debug, error};
use crate::domain::student::Student;

/// Найти студента по Telegram ID
pub async fn find_by_telegram_id(pool: &PgPool, telegram_id: i64) -> anyhow::Result<Option<Student>> {
    debug!("Запрос на поиск студента с telegram_id={}", telegram_id);

    let row = sqlx::query!(
        r#"
        SELECT id, telegram_id, faculty, group_name, study_form, created_at
        FROM students
        WHERE telegram_id = $1
        "#,
        telegram_id
    )
        .fetch_optional(pool)
        .await;

    match row {
        Ok(Some(r)) => {
            info!("Студент найден: id={}, telegram_id={}", r.id, r.telegram_id);
            Ok(Some(Student {
                id: r.id,
                telegram_id: r.telegram_id,
                faculty: r.faculty,
                group_name: r.group_name,
                study_form: r.study_form,
                created_at: r.created_at,
            }))
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

    let res = sqlx::query!(
        r#"
        INSERT INTO students (telegram_id, faculty, group_name, study_form)
        VALUES ($1, $2, $3, $4)
        RETURNING id, telegram_id, faculty, group_name, study_form, created_at
        "#,
        telegram_id,
        faculty,
        group_name,
        study_form
    )
        .fetch_one(pool)
        .await;

    match res {
        Ok(r) => {
            info!("Студент успешно добавлен: id={}, telegram_id={}", r.id, r.telegram_id);
            Ok(Student {
                id: r.id,
                telegram_id: r.telegram_id,
                faculty: r.faculty,
                group_name: r.group_name,
                study_form: r.study_form,
                created_at: r.created_at,
            })
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

    let res = sqlx::query!(
        r#"
        UPDATE students
        SET faculty = $2, group_name = $3, study_form = $4
        WHERE telegram_id = $1
        RETURNING id, telegram_id, faculty, group_name, study_form, created_at
        "#,
        telegram_id,
        faculty,
        group_name,
        study_form
    )
        .fetch_optional(pool)
        .await;

    match res {
        Ok(Some(r)) => {
            info!("Студент обновлён: id={}, telegram_id={}", r.id, r.telegram_id);
            Ok(Some(Student {
                id: r.id,
                telegram_id: r.telegram_id,
                faculty: r.faculty,
                group_name: r.group_name,
                study_form: r.study_form,
                created_at: r.created_at,
            }))
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

    let res = sqlx::query!(
        r#"
        DELETE FROM students
        WHERE telegram_id = $1
        "#,
        telegram_id
    )
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
