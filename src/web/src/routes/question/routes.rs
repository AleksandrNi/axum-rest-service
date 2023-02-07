use axum::Router;
use axum::routing::{get, post};
use super::handlers::{get_questions, post_question, get_question_by_id};
pub async fn router() -> Router {
    Router::new()
        .route("/question", get(get_questions))
        .route("/question", post(post_question))
        .route("/question/:id", get(get_question_by_id))
}