use crate::db::Db;

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub max_date_variants: usize,
    pub max_range_days: i64,
}
