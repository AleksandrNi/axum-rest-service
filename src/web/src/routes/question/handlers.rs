use service::question_service;
use repository::domain::question::QuestionModel;
use utils::error::app_error::AppGenericError;
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use axum::extract::Path;
use tracing::field::debug;
use service::question_service::QuestionDto;
use crate::utils::error::AppResponseError;
use crate::utils::error::prepare_response;

pub async fn get_questions() -> Result<Json<Vec<QuestionDto>>, AppResponseError> {
    prepare_response(question_service::get_questions().await)
}

pub async fn post_question(Json(new_question): Json<QuestionDto>) -> Result<Json<QuestionDto>, AppResponseError> {
    prepare_response(question_service::post_question(new_question).await)
}

pub async fn get_question_by_id(Path(id): Path<i32>) -> Result<Json<QuestionDto>, AppResponseError> {
    prepare_response(question_service::get_question_by_id(id).await)
}

pub async fn delete_question_by_id(Path(id): Path<i32>) -> Result<Json<bool>, AppResponseError> {
    prepare_response(question_service::delete_question_by_id(id).await)
}

pub async fn patch_question(Json(new_question): Json<QuestionDto>) -> Result<Json<QuestionDto>, AppResponseError> {
    prepare_response(question_service::patch_question(new_question).await)
}