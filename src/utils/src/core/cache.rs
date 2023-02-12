// https://github.com/djc/bb8/blob/main/redis/src/lib.rs
use std::borrow::BorrowMut;
use std::env;
// use redis::{Client, Commands, Connection, ConnectionLike, RedisResult};
// use r2d2::{Pool, PooledConnection};
use tokio::sync::OnceCell;
use tracing::info;
use crate::error::app_error::AppGenericError;
use crate::error::app_web_error::AppWebError;

use async_trait::async_trait;
use bb8::{Pool, PooledConnection};
use bb8_redis::RedisConnectionManager;
use redis::{aio::Connection, AsyncCommands, ConnectionLike, ErrorKind};
use redis::{Client, IntoConnectionInfo, RedisError};

async fn init_connection() -> Pool<RedisConnectionManager> {
    info!("execute : initializing cache connection ...");
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| panic!("REDIS_URL must be set!"));

    let manager = bb8_redis::RedisConnectionManager::new(redis_url)
        .expect("Failed to create redis connection manager");

    let pool = bb8::Pool::builder()
        .max_size(15)
        .build(manager)
        .await
        .expect("Failed to create redis pool");

    // assert_eq!(true, pool.get().await.unwrap(), "Redis connection check failed");

    health_check(pool.clone()).await;
    pool.clone()
}

async fn health_check(pool: Pool<RedisConnectionManager>) {
    let mut connection = pool.get().await.unwrap();
    info!("execute : test cache connection ...");
    // throw away the result, just make sure it does not fail
    let health_check_key = "health_check_key";
    let health_check_value = "health_check_value";
    let _ : () = connection.set(health_check_key, health_check_value).await.unwrap();
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    let result: String = connection.get(health_check_key).await.unwrap();
    connection.del::<&str, i32>(health_check_key).await.unwrap();
    assert_eq!(result, health_check_value);
    info!("executed: test cache connection");
}

static CONN: OnceCell<Pool<RedisConnectionManager>> = OnceCell::const_new();

pub async fn get_connection() -> Result<PooledConnection<'static,RedisConnectionManager>, AppGenericError> {
    (CONN.get_or_init(init_connection).await).get().await
        .map_err(|err| AppWebError::failed_getting_db_connection(err.to_string()))
}

pub async fn set_key_value(key: &str, value: &str) {
    let mut connection = get_connection().await.unwrap();
    let result: String = connection.set(key, value).await.unwrap();
    info!("stored data = {}", result);
}

pub async fn get_key(key: &str) -> Option<String> {
    let mut connection = get_connection().await.unwrap();
    let result: Option<String> = match connection.get(key).await {
        Ok(data) => Some(data),
        Err(err) => None
    };
    result
}