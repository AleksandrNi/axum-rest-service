use std::env;
use sqlx::postgres::PgPoolOptions;
use api::db::index::Db;

pub async fn init_db () -> Result<Db, ()> {
    println!("persistence");

    let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| panic!("DATABASE_URL must be set!"));

    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&db_url)
        .await
        .unwrap_or_else(|_| panic!("Cannot connect to the database. Please check your configuration."));

    Ok(Db::new(pool).await)
}