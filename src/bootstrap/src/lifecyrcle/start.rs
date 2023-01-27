use web::server::index;

pub async fn run () {
    index::run().await
}