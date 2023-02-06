use service::question;
use repository::domain::question::Question;

use utils::error::app_repository_error::AppRepositoryError;
use utils::error::app_service_error::AppServiceError;
use utils::error::app_web_error::AppWebError;
use utils::error::app_error::AppErrorData;
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use crate::utils::error::AppError;
use axum::extract::Path;
use tracing::field::debug;

pub async fn get_questions() -> Result<Json<Vec<Question>>, AppError> {
    let result = question::get_questions().await.unwrap();
    Ok(Json(result))
}

pub async fn post_question(Json(Question): Json<Question>) -> Result<Json<Question>, AppError> {
    let result = question::post_question(Question).await.unwrap();
    Ok(Json(result))
}
pub async fn get_question_by_id(
    Path(id): Path<i32>
) -> Result<Json<Question>, AppError> {
    println!("------------> {}",  id);
    let result = question::get_question_by_id(id).await.unwrap();
    Ok(Json(result))
}