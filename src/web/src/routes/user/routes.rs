use axum::Router;
use axum::routing::{post};
use crate::routes::user::handlers::{create_user, login_user};

pub async fn router() -> Router {
    Router::new()
        .route("/user/create", post(create_user))
        .route("/user/login", post(login_user))
}