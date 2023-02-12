use axum::Json;
use service::user::user_dto::UserDto;
use utils::error::app_error::AppGenericError;
use utils::core::jwt;
use crate::routes::user::dto::{UserCreateRequest, UserCreateResponse, UserLoginRequest};
use crate::utils::error::{AppResponseError, prepare_response};
use service::user::user_service;
use crate::utils::user::cache_user_data;


pub async fn user_create(Json(user_create_request): Json<UserCreateRequest>) -> Result<Json<UserCreateResponse>, AppResponseError> {
    let result = transform_result_dto_to_response(
        user_service::user_create(UserDto::from(user_create_request)).await).await;
    prepare_response(result)
}

pub async fn user_login(Json(user_login_request): Json<UserLoginRequest>) -> Result<Json<UserCreateResponse>, AppResponseError> {
    let result = transform_result_dto_to_response(
        user_service::user_login(UserDto::from(user_login_request)).await).await;
    prepare_response(result)
}

async fn transform_result_dto_to_response(result: Result<UserDto, AppGenericError>) -> Result<UserCreateResponse, AppGenericError> {
    match result {
        // TODO: add token to dto
        Ok(dto) => {
            let mut response = UserCreateResponse::from(dto);
            let token = jwt::jwt_create(response.get_id());
            cache_user_data(response.get_id(), &token[..]).await;
            response.set_token(token);
            Ok(response)
        },
        Err(err) => Err(err)
    }
}