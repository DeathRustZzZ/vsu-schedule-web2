use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use tracing::warn;

use crate::http::state::AppState;
use crate::model::ListLessonResponse;
use crate::service::schedule;
use crate::util::date::{build_date_variants, build_date_variants_for_range, parse_date_any};

#[derive(Debug, Deserialize)]
pub struct ScheduleQuery {
    pub faculty: String,
    pub group: String,
    pub subgroup: String,
    pub date: String,
}

#[derive(Debug, Deserialize)]
pub struct ScheduleWeekQuery {
    pub faculty: String,
    pub group: String,
    pub subgroup: String,
    pub start: String,
    pub end: Option<String>,
}

pub async fn health() -> &'static str {
    "ok"
}

pub async fn get_schedule_by_date(
    State(state): State<AppState>,
    Query(query): Query<ScheduleQuery>,
) -> Result<Json<ListLessonResponse>, ApiError> {
    let faculty = query.faculty.trim();
    let group = query.group.trim();
    let subgroup = query.subgroup.trim();
    let date = query.date.trim();

    if faculty.is_empty() || group.is_empty() || date.is_empty() {
        return Err(ApiError::BadRequest("missing required query params"));
    }

    let parsed_date = parse_date_any(date)
        .ok_or(ApiError::BadRequest("invalid date format"))?;

    let variants = build_date_variants(parsed_date, state.max_date_variants);
    if variants.is_empty() {
        return Err(ApiError::BadRequest("no date variants"));
    }

    let lessons = schedule::get_schedule_by_date(
        &state.db.pool,
        faculty,
        group,
        subgroup,
        &variants,
    )
    .await
    .map_err(ApiError::Db)?;

    Ok(Json(ListLessonResponse {
        lesson_responses: lessons,
    }))
}

pub async fn get_schedule_week(
    State(state): State<AppState>,
    Query(query): Query<ScheduleWeekQuery>,
) -> Result<Json<ListLessonResponse>, ApiError> {
    let faculty = query.faculty.trim();
    let group = query.group.trim();
    let subgroup = query.subgroup.trim();
    let start_raw = query.start.trim();

    if faculty.is_empty() || group.is_empty() || start_raw.is_empty() {
        return Err(ApiError::BadRequest("missing required query params"));
    }

    let start_date = parse_date_any(start_raw)
        .ok_or(ApiError::BadRequest("invalid start date format"))?;

    let end_date = match query.end.as_ref() {
        Some(value) if !value.trim().is_empty() => {
            parse_date_any(value).ok_or(ApiError::BadRequest("invalid end date format"))?
        }
        _ => start_date
            .checked_add_signed(chrono::Duration::days(state.max_range_days - 1))
            .ok_or(ApiError::BadRequest("invalid date range"))?,
    };

    if end_date < start_date {
        return Err(ApiError::BadRequest("end date before start date"));
    }

    let days = (end_date - start_date).num_days() + 1;
    if days > state.max_range_days {
        return Err(ApiError::BadRequest("date range exceeds limit"));
    }

    let variants = build_date_variants_for_range(
        start_date,
        end_date,
        state.max_date_variants,
        state.max_range_days,
    );
    if variants.is_empty() {
        return Err(ApiError::BadRequest("no date variants"));
    }

    let lessons = schedule::get_schedule_by_date(
        &state.db.pool,
        faculty,
        group,
        subgroup,
        &variants,
    )
    .await
    .map_err(ApiError::Db)?;

    Ok(Json(ListLessonResponse {
        lesson_responses: lessons,
    }))
}

#[derive(Debug)]
pub enum ApiError {
    BadRequest(&'static str),
    Db(sqlx::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::BadRequest(message) => (StatusCode::BAD_REQUEST, message).into_response(),
            ApiError::Db(err) => {
                warn!("database error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "database error").into_response()
            }
        }
    }
}
