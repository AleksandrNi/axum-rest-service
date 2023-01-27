use std::net::{IpAddr, SocketAddr};
use axum::{response, Router};
use dotenvy::dotenv;
use std::env;
use axum::extract::State;
use api::state::index;
use repository::modules::persistence;
use repository::modules::repository::health_check;
use crate::SharedData;

pub async fn run () -> State<SharedData> {
    dotenv().ok();
    let db = persistence::init::init_db().await.unwrap();
    let health_check = health_check::HealthCheck::new(db.clone());
    health_check.check().await;

    State(index::SharedData{db})

}