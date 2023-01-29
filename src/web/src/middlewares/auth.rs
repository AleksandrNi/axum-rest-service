use axum::{http::{Request, StatusCode}, middleware::Next, response::Response, headers::{ Authorization, authorization::Bearer}, TypedHeader};
use utils::core::jwt;
pub async fn guard<B>(
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    mut req: Request<B>,
    next: Next<B>
) -> Result<Response, StatusCode> {
    let _token = token.token().to_owned();
    if let Err(err)  = jwt::jwt_verify(&_token[..]) {
        match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                // refresh token
                let token = jwt::jwt_create();
                // store updated token
                todo!();
            },
            _ => return Err(StatusCode::EXPECTATION_FAILED),
        }
    } {
        req.extensions_mut().insert("user");
    };

    Ok(next.run(req).await)
}