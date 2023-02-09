use axum::Router;
use axum::routing::{get, post, delete, patch};
use super::handlers::{get_questions, post_question, get_question_by_id, delete_question_by_id, patch_question};
pub async fn router() -> Router {
    Router::new()
        .route("/question", get(get_questions))
        .route("/question", post(post_question))
        .route("/question/:id", get(get_question_by_id))
        .route("/question/:id", delete(delete_question_by_id))
        .route("/question", patch(patch_question))
}