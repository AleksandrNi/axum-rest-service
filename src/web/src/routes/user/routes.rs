use axum::Router;
use axum::routing::{post};
use crate::routes::user::handlers::{user_create, user_login};

pub async fn router() -> Router {
    Router::new()
        .route("/user/create", post(user_create))
        .route("/user/login", post(user_login))
}