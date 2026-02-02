//src/main.rs
pub mod config;
pub mod db;
pub mod domain;
pub mod errors;
pub mod services;
pub mod app;

use crate::config::AppConfig;
use crate::db::connection::init_pool;
use crate::app::facade::run_app;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let config = AppConfig::load_from_environment();
    let pool = init_pool(&config.database_url).await;

    run_app(config, pool).await;
}