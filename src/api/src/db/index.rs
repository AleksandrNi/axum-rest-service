use std::sync::Arc;
use sqlx::Postgres;
use sqlx::Pool;

#[derive(Clone)]
pub struct Db(pub Arc<Pool<Postgres>>);
impl Db {
    pub async fn new (pool: Pool<Postgres>) -> Self {
        Db(Arc::new(pool))
    }
}