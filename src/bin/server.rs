use dotenv::dotenv;
use repository::db;
use web;
use utils::core::logger;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), ()> {
    dotenv().ok();
    logger::run().await;
    info!("server is running up ...");
    db::health_check::run().await;
    web::server::run().await;
    Ok(())
}