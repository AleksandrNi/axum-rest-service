use axum::{
    headers::{authorization::Bearer, Authorization},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    TypedHeader,
};
use tracing::info;
use tracing::log::log;
use utils::core::jwt;
pub async fn guard<B>(
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let jwt_token = token.token().to_owned();
    info!("token = {}", &jwt_token);
    if let Err(err) = jwt::jwt_verify(&jwt_token[..]) {
        match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                // refresh token
                let token = jwt::jwt_create();
                // store updated token
                todo!();
            }
            _ => return Err(StatusCode::EXPECTATION_FAILED),
        }
    }
    {
        req.extensions_mut().insert("user");
    };

    Ok(next.run(req).await)
}
