use core::panic;
use api::db::index::Db;


pub struct HealthCheck {
    db: Db
}

impl HealthCheck {
    pub fn new(db: Db) -> Self {
        Self{db}
    }
}

impl HealthCheck {
    pub async fn check (&self) -> Result<(),()> {
        println!("health check");

        let pool = self.db.0.clone();

        let result = pool.try_acquire()
            .map(|_| ())
            .ok_or_else(|| panic!("Failed to connect database `postgres`."))
            .unwrap_or_else(|_| panic!("Failed to connect database `postgres`."));

        let row: (i64,) = sqlx::query_as("SELECT $1")
            .bind(150_i64)
            .fetch_one(&*pool)
            .await
            .unwrap_or_else(|_| panic!("Failed to to execute query. Database `postgres`."));

        println!("!!! >>>>>>>> {:?}", result);
        println!("row = {:?}", row);
        assert_eq!(row.0, 150);

        Ok(())
    }
}