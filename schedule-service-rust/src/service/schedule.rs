use crate::model::{LessonResponse, TeacherResponse};
use sqlx::{FromRow, PgPool};

#[derive(Debug, FromRow)]
struct LessonRow {
    lesson_id: uuid::Uuid,
    start_time: Option<String>,
    end_time: Option<String>,
    auditorium: Option<String>,
    lesson_date: Option<String>,
    week_day: Option<String>,
    group_id: Option<String>,
    teacher_id: Option<i32>,
    subgroup_id: Option<String>,
    lesson_name: Option<String>,
    lesson_type: Option<String>,
    t_id: Option<i32>,
    t_firstname: Option<String>,
    t_lastname: Option<String>,
    t_surname: Option<String>,
    t_initials: Option<String>,
    t_img_link: Option<String>,
    t_description: Option<String>,
    t_fullname: Option<String>,
    t_qualification: Option<String>,
}

pub async fn get_schedule_by_date(
    pool: &PgPool,
    faculty: &str,
    group_id: &str,
    subgroup_id: &str,
    date_variants: &[String],
) -> Result<Vec<LessonResponse>, sqlx::Error> {
    let rows = sqlx::query_as::<_, LessonRow>(
        r#"
        SELECT
            l.lesson_id,
            l.start_time,
            l.end_time,
            l.auditorium,
            l.date AS lesson_date,
            l.weekday AS week_day,
            l.group_id,
            l.teacher_id,
            l.subgroup_id,
            l.lesson_name,
            l.type AS lesson_type,
            t.teacher_id AS t_id,
            t.firstname AS t_firstname,
            t.lastname AS t_lastname,
            t.surname AS t_surname,
            t.initials AS t_initials,
            t.img_link AS t_img_link,
            t.description AS t_description,
            t.fullname AS t_fullname,
            t.qualification AS t_qualification
        FROM lessons l
        LEFT JOIN teachers t ON l.teacher_id = t.teacher_id
        WHERE l.faculty = $1
          AND (l.group_id = $2 OR l.subgroup_id = $3)
          AND l.date = ANY($4)
        ORDER BY l.date ASC, l.start_time ASC
        "#,
    )
    .bind(faculty)
    .bind(group_id)
    .bind(subgroup_id)
    .bind(date_variants)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(map_row).collect())
}

fn map_row(row: LessonRow) -> LessonResponse {
    LessonResponse {
        id: row.lesson_id,
        start_time: row.start_time,
        end_time: row.end_time,
        auditorium: row.auditorium,
        date: row.lesson_date,
        week_day: row.week_day,
        group_id: row.group_id,
        teacher_id: row.teacher_id,
        teacher: row.t_id.map(|id| TeacherResponse {
            id: Some(id),
            firstname: row.t_firstname,
            lastname: row.t_lastname,
            surname: row.t_surname,
            initials: row.t_initials,
            img_link: row.t_img_link,
            description: row.t_description,
            fullname: row.t_fullname,
            qualification: row.t_qualification,
        }),
        subgroup_id: row.subgroup_id,
        name: row.lesson_name,
        lesson_type: row.lesson_type,
    }
}
