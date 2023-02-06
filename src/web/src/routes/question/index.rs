use service::question;
use repository::domain::question::Question;

use utils::error::app_repository_error::AppRepositoryError;
use utils::error::app_service_error::AppServiceError;
use utils::error::app_web_error::AppWebError;
use utils::error::app_error::AppErrorData;
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use crate::utils::error::AppError;

pub async fn get_questions() -> Result<Json<Vec<Question>>, AppError> {
    let result = question::get_questions().await.unwrap();
    Ok(Json(result))
}