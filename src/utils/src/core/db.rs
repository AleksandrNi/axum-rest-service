use async_trait::async_trait;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres, Transaction};
use std::env;
use tokio::sync::OnceCell;
use tracing::info;

async fn init_connection() -> Pool<Postgres> {
    info!("execute : initializing db connection ...");
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| panic!("DATABASE_URL must be set!"));

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .unwrap_or_else(|_| {
            panic!("Cannot connect to the database. Please check your configuration.")
        });
    info!("executed: initializing db connection");
    health_check(&pool).await;
    pool
}

async fn health_check(pool: &Pool<Postgres>) {
    info!("execute : test db connection ...");
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&*pool)
        .await
        .unwrap_or_else(|_| panic!("Failed to to execute query. Database `postgres`."));

    assert_eq!(row.0, 150);
    info!("executed: test db connection");
}

static CONN: OnceCell<Pool<Postgres>> = OnceCell::const_new();

pub async fn get_connection() -> &'static Pool<Postgres> {
    CONN.get_or_init(init_connection).await
}

#[async_trait]
pub trait TxAsync {
    async fn begin() -> Transaction<'static, Postgres>;
    async fn commit(tx: Transaction<'static, Postgres>);
}

pub struct Tx;

#[async_trait]
impl TxAsync for Tx {
    async fn begin() -> Transaction<'static, Postgres> {
        get_connection()
            .await
            .begin()
            .await
            .expect("Unable to begin transaction")
    }

    async fn commit(tx: Transaction<'static, Postgres>) {
        tx.commit().await.expect("Unable to commit the transaction");
    }
}
