use utils::error::app_error::AppGenericError;
use axum::Json;
use axum::extract::Path;
use service::question::question_dto::QuestionDto;
use crate::utils::error::AppResponseError;
use crate::utils::error::prepare_response;
use service::question::question_service;
use crate::routes::question::dto::{QuestionRequest, QuestionResponse};

pub async fn get_questions() -> Result<Json<Vec<QuestionResponse>>, AppResponseError> {
    let result = transform_result_vec_dto_to_vec_response(question_service::get_questions().await);
    prepare_response(result)
}

pub async fn post_question(Json(question_request): Json<QuestionRequest>) -> Result<Json<QuestionResponse>, AppResponseError> {
    let question_dto = QuestionDto::from(question_request);
    let result = transform_result_dto_to_response(question_service::post_question(question_dto).await);
    prepare_response(result)
}

pub async fn get_question_by_id(Path(id): Path<i32>) -> Result<Json<QuestionResponse>, AppResponseError> {
    let result = transform_result_dto_to_response(question_service::get_question_by_id(id).await);
    prepare_response(result)
}

pub async fn delete_question_by_id(Path(id): Path<i32>) -> Result<Json<bool>, AppResponseError> {
    prepare_response(question_service::delete_question_by_id(id).await)
}

pub async fn patch_question(Json(question_request): Json<QuestionRequest>) -> Result<Json<QuestionResponse>, AppResponseError> {
    let question_dto = QuestionDto::from(question_request);
    let result = transform_result_dto_to_response(question_service::patch_question(question_dto).await);
    prepare_response(result)
}

fn transform_result_dto_to_response(result: Result<QuestionDto, AppGenericError>) -> Result<QuestionResponse, AppGenericError> {
    match result {
        Ok(dto) => Ok(QuestionResponse::from(dto)),
        Err(err) => Err(err)
    }
}
fn transform_result_vec_dto_to_vec_response(result: Result<Vec<QuestionDto>, AppGenericError>) -> Result<Vec<QuestionResponse>, AppGenericError> {
    match result {
        Ok(list) => Ok(list.into_iter().map(|dto| QuestionResponse::from(dto)).collect()),
        Err(err) => Err(err)
    }
}