use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use tokio::sync::OnceCell;
use std::env;
use tracing::info;

async fn init_connection() -> Pool<Postgres> {
    let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| panic!("DATABASE_URL must be set!"));

    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&db_url)
        .await
        .unwrap_or_else(|_| panic!("Cannot connect to the database. Please check your configuration."));
    info!("executed: initializing db connection");
    pool
}

static CONN: OnceCell<Pool<Postgres>> = OnceCell::const_new();

pub async fn get_connection() -> &'static Pool<Postgres> {
    CONN.get_or_init(init_connection).await
}