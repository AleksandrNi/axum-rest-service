use std::net::SocketAddr;
use axum::{middleware, Router};
use utils::core::server::get_address;
use crate::middlewares::auth;

pub async fn run () {
    let app = Router::new();
        // .route_layer(middleware::from_fn(auth::guard));

    let adress = SocketAddr::from(get_address());

    axum::Server::bind(&adress)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch."));
}