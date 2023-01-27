#[tokio::main]
async fn main() -> Result<(), ()> {
    println!("Hello world!");
    bootstrap::run().await
}
