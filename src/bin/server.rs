use dotenv::dotenv;
use web;
use utils::core::{logger};
use repository::migrations;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), ()> {
    dotenv().ok();
    logger::run().await;
    info!("server is running up ...");
    migrations::run().await;
    web::server::run().await;
    Ok(())
}