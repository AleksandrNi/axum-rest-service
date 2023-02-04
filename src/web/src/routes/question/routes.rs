use axum::Router;
use axum::routing::get;
use super::index::get_questions;
pub async fn router() -> Router {
    Router::new()
        .route("/question", get(get_questions))
}