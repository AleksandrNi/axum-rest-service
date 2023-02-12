use axum::{
    headers::{authorization::Bearer, Authorization},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    TypedHeader,
};
use jsonwebtoken::errors::ErrorKind;
use tracing::info;
use utils::core::jwt;
use crate::middlewares::dto::user::UserPayload;
use service::user::user_service;


pub async fn guard<B>(
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let jwt_token = token.token().to_owned();
    info!("token = {}", &jwt_token);

    let token_payload_result = jwt::jwt_decode(&jwt_token[..]);
    if let Err(err) = token_payload_result {
        match err.kind() {
            ErrorKind::ExpiredSignature => {
                info!("error: {:?}", err)
            }
            _ => {
                info!("{:?}", err.kind());
                return Err(StatusCode::EXPECTATION_FAILED)
            }
        }
    } else {
        let token_payload = token_payload_result.unwrap();
        let user_dto = user_service::user_load(token_payload.get_email().to_string()).await.unwrap();
        req.extensions_mut().insert(UserPayload::new(user_dto, token_payload));
    }

    // if let Err(err) = jwt::jwt_verify(&jwt_token[..]) {
    //     match err.kind() {
    //         jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
    //             // refresh token
    //             let token = jwt::jwt_create();
    //             // store updated token
    //             todo!();
    //         }
    //         _ => return Err(StatusCode::EXPECTATION_FAILED),
    //     }
    // }
    // {
    //     req.extensions_mut().insert("user");
    // };

    Ok(next.run(req).await)
}
