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
            .merge(routes::question::routes::router().await)
            .route_layer(middleware::from_fn(auth::guard))
            .route("/ping", get(ping))
            .merge(routes::user::routes::router().await);

    let adress = SocketAddr::from(get_address());

    axum::Server::bind(&adress)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch."));
}

pub async fn ping() -> Result<String, StatusCode> {
    Ok("service works well".to_owned())
}
