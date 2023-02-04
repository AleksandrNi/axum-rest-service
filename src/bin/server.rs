use dotenv::dotenv;
use repository::migrations;
use tracing::info;
use utils::core::logger;
use web;

#[tokio::main]
async fn main() -> Result<(), ()> {
    dotenv().ok();
    logger::run().await;
    info!("server is running up ...");
    migrations::run().await;
    web::server::run().await;
    Ok(())
}
