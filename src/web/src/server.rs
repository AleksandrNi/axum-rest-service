use std::net::SocketAddr;
use axum::Router;
use utils::core::server::get_address;

pub async fn run () {
    let app = Router::new();

    let adress = SocketAddr::from(get_address());

    axum::Server::bind(&adress)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch."));
}