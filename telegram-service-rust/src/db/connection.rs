//src/db/connection.rs
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use log::{ info, error};

pub async fn init_pool(database_url: &str) -> PgPool {
    info!("Инициализация пула соединений к базе данных...");

    match PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
    {
        Ok(pool) => {
            info!("Успешное подключение к базе данных.");
            pool
        }
        Err(e) => {
            error!("Ошибка подключения к базе данных: {:?}", e);
            panic!("Failed to connect to the database");
        }
    }
}
