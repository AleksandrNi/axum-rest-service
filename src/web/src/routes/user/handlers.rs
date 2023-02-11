use axum::http::StatusCode;
use axum::Json;
use service::user::user_dto::UserDto;
use utils::error::app_error::AppGenericError;
use crate::routes::user::dto::{UserCreateRequest, UserCreateResponse, UserLoginRequest};
use crate::utils::error::{AppResponseError, prepare_response};
use service::user::user_service;


pub async fn user_create(Json(user_create_request): Json<UserCreateRequest>) -> Result<Json<UserCreateResponse>, AppResponseError> {
    let result = transform_result_dto_to_response(
        user_service::user_create(UserDto::from(user_create_request)).await);
    prepare_response(result)
}

pub async fn user_login(Json(user_login_request): Json<UserLoginRequest>) -> Result<Json<UserCreateResponse>, AppResponseError> {
    let result = transform_result_dto_to_response(
        user_service::user_login(UserDto::from(user_login_request)).await);
    prepare_response(result)
}

fn transform_result_dto_to_response(result: Result<UserDto, AppGenericError>) -> Result<UserCreateResponse, AppGenericError> {
    match result {
        Ok(dto) => Ok(UserCreateResponse::from(dto)),
        Err(err) => Err(err)
    }
}