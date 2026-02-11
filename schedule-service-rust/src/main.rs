mod config;
mod db;
mod http;
mod model;
mod service;
mod util;

use axum::{routing::get, Router};
use config::AppConfig;
use db::Db;
use http::handlers::{get_schedule_by_date, get_schedule_week, health};
use http::state::AppState;
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() {
    config::init_tracing();

    let config = AppConfig::from_env();
    let db = Db::connect(&config.database_url).await;

    let state = AppState {
        db,
        max_date_variants: config.max_date_variants,
        max_range_days: config.max_range_days,
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/v1/bot/v2/schedule", get(get_schedule_by_date))
        .route("/api/v1/bot/v2/schedule/week", get(get_schedule_week))
        .with_state(state);

    let addr: SocketAddr = config
        .bind_addr
        .parse()
        .unwrap_or_else(|_| "0.0.0.0:9900".parse().unwrap());

    info!("schedule-service-rust listening on {}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
