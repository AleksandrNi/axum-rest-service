use core::panic;
use utils::core::db::get_connection;
use tracing::info;
pub async fn run () {
        let pool = get_connection().await;
        let result = pool.try_acquire()
            .map(|_| ())
            .ok_or_else(|| panic!("Failed to connect database `postgres`."))
            .unwrap_or_else(|_| panic!("Failed to connect database `postgres`."));

        let row: (i64,) = sqlx::query_as("SELECT $1")
            .bind(150_i64)
            .fetch_one(&*pool)
            .await
            .unwrap_or_else(|_| panic!("Failed to to execute query. Database `postgres`."));

        assert_eq!(row.0, 150);
        info!("executed: test db connection");
}