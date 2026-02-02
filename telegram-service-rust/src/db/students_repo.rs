use sqlx::PgPool;
use log::{info, debug, error};
use crate::domain::student::Student;

pub async fn find_by_telegram_id(
    pool: &PgPool,
    telegram_id: i64,
) -> anyhow::Result<Option<Student>> {
    debug!("Запрос на поиск студента telegram_id={}", telegram_id);

    let row = sqlx::query_as::<_, Student>(
        r#"
        SELECT id, telegram_id, faculty, group_name, study_form, course, username, created_at
        FROM students
        WHERE telegram_id = $1
        "#,
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
            info!("Студент telegram_id={} не найден.", telegram_id);
            Ok(None)
        }
        Err(e) => {
            error!("Ошибка при поиске студента: {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn insert(
    pool: &PgPool,
    telegram_id: i64,
    faculty: &str,
    group_name: &str,
    study_form: &str,
    course: Option<&str>,
    username: Option<&str>,
) -> anyhow::Result<Student> {
    info!(
        "Регистрация студента: telegram_id={}, faculty={}, group={}, study_form={}, course={:?}, username={:?}",
        telegram_id, faculty, group_name, study_form, course, username
    );

    let res = sqlx::query_as::<_, Student>(
        r#"
        INSERT INTO students (telegram_id, faculty, group_name, study_form, course, username)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (telegram_id)
        DO UPDATE SET
            faculty = EXCLUDED.faculty,
            group_name = EXCLUDED.group_name,
            study_form = EXCLUDED.study_form,
            course = EXCLUDED.course,
            username = EXCLUDED.username
        RETURNING id, telegram_id, faculty, group_name, study_form, course, username, created_at
        "#,
    )
    .bind(telegram_id)
    .bind(faculty)
    .bind(group_name)
    .bind(study_form)
    .bind(course)
    .bind(username)
    .fetch_one(pool)
    .await;

    match res {
        Ok(student) => {
            info!("Студент успешно сохранён: id={}, telegram_id={}", student.id, student.telegram_id);
            Ok(student)
        }
        Err(e) => {
            error!("Ошибка при сохранении студента: {:?}", e);
            Err(e.into())
        }
    }
}
