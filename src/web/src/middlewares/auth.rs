use std::result;
use axum::{
    headers::{authorization::Bearer, Authorization},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    TypedHeader,
};
use jsonwebtoken::errors::ErrorKind;
use tracing::info;
use service::user::user_dto::UserDto;
use utils::core::jwt;
use crate::middlewares::dto::user::UserPayload;
use service::user::user_service;
use utils::core::jwt::jwt_create;
use utils::error::app_error::AppGenericError;
use crate::utils::user::{cache_refresh_user_data, get_cached_user_id_by_token, get_cached_user_token_by_id};


pub async fn guard<B>(
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let jwt_token = token.token().to_owned();
    let token_payload_result = jwt::jwt_decode(&jwt_token[..]);

    if let Err(err) = token_payload_result {
        match err.kind() {
            ErrorKind::ExpiredSignature => {
                match get_cached_user_id_by_token(&jwt_token[..]).await {
                    Some(data) => {
                        match user_service::user_load_by_id(data).await {
                            Ok(user_dto) => {
                                let new_token = jwt_create(user_dto.get_id().unwrap());
                                cache_refresh_user_data(user_dto.get_id().unwrap(), &new_token[..]).await;
                            }
                            Err(_) => return Err(StatusCode::UNAUTHORIZED),
                        }
                    }
                    None => return Err(StatusCode::UNAUTHORIZED),
                }
            }
            _ => return Err(StatusCode::UNAUTHORIZED)
        }
    } else {
        let token_payload = token_payload_result.unwrap();

        let user_dto = user_service::user_load_by_id(token_payload.get_user_id()).await.unwrap();
        req.extensions_mut().insert(UserPayload::new(user_dto, token_payload));
    }

    Ok(next.run(req).await)
}
