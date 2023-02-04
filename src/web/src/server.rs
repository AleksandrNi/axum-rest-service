use crate::middlewares::auth;
use axum::{middleware, Router};
use std::net::SocketAddr;
use axum::http::StatusCode;
use utils::core::server::get_address;
use crate::routes;
use axum::routing::get;


pub async fn run() {
    let app =
        Router::new()
            .route("/", get(get_hello))
        .merge(routes::question::routes::router().await);
    // .route_layer(middleware::from_fn(auth::guard));

    let adress = SocketAddr::from(get_address());

    axum::Server::bind(&adress)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch."));
}

pub async fn get_hello () -> Result<String, StatusCode> {
    Ok("hello world".to_owned())
}
