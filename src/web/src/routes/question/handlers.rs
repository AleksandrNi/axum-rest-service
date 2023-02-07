use service::question_service;
use repository::domain::question::Question;

use utils::error::app_repository_error::AppRepositoryError;
use utils::error::app_service_error::AppServiceError;
use utils::error::app_web_error::AppWebError;
use utils::error::app_error::AppGenericError;
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use crate::utils::error::{AppResponseError, prepare_response};
use axum::extract::Path;
use tracing::field::debug;

pub async fn get_questions() -> Result<Json<Vec<Question>>, AppResponseError> {
    prepare_response(question_service::get_questions().await)
    // match  {
    //     Ok(data) => Ok(Json(data)),
    //     Err(err) => match err {
    //         AppGenericError::Repository(e) => AppError::Repository(e),
    //         AppGenericError::Service(e) => AppError::Service(e)
    //     }
    // }
}

pub async fn post_question(Json(new_question): Json<Question>) -> Result<Json<Question>, AppResponseError> {
    prepare_response(question_service::post_question(new_question).await)

    // let result = question::post_question(Question).await.unwrap();
    // Ok(Json(result))
}
pub async fn get_question_by_id(Path(id): Path<i32>) -> Result<Json<Question>, AppResponseError> {
    prepare_response(question_service::get_question_by_id(id).await)

    // println!("------------> {}",  id);
    // let result = question::get_question_by_id(id).await.unwrap();
    // Ok(Json(result))
}