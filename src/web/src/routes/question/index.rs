use service::question;
use repository::domain::question::Question;

use utils::error::app_repository_error::AppRepositoryError;
use utils::error::app_service_error::AppServiceError;
use utils::error::app_web_error::AppWebError;
use utils::error::app_error::{AppErrorData, AppGenericError};
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use crate::utils::error::{AppError, prepareResponse};
use axum::extract::Path;
use tracing::field::debug;

pub async fn get_questions() -> Result<Json<Vec<Question>>, AppError> {
    prepareResponse(question::get_questions().await)
    // match  {
    //     Ok(data) => Ok(Json(data)),
    //     Err(err) => match err {
    //         AppGenericError::Repository(e) => AppError::Repository(e),
    //         AppGenericError::Service(e) => AppError::Service(e)
    //     }
    // }
}

pub async fn post_question(Json(Question): Json<Question>) -> Result<Json<Question>, AppError> {
    prepareResponse(question::post_question(Question).await)

    // let result = question::post_question(Question).await.unwrap();
    // Ok(Json(result))
}
pub async fn get_question_by_id(Path(id): Path<i32>) -> Result<Json<Question>, AppError> {
    prepareResponse(question::get_question_by_id(id).await)

    // println!("------------> {}",  id);
    // let result = question::get_question_by_id(id).await.unwrap();
    // Ok(Json(result))
}