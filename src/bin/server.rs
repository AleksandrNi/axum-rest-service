use dotenv::dotenv;
use repository::migrations;
use tracing::info;
use utils::core::logger;
use utils::core::cache;
use web;

#[tokio::main]
async fn main() -> Result<(), ()> {
    dotenv().ok();
    logger::run().await;
    info!("server is running up ...");
    migrations::run().await;
    // test cache connection
    cache::get_connection().await.expect("Failed to create cache connection");
    web::server::run().await;
    Ok(())
}
