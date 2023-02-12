use axum::http::StatusCode;
use chrono::Duration;
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use tracing::info;
use crate::dto::token_payload::TokenPayload;

pub fn jwt_create(user_id: u32) -> String {
    let mut now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let expires_in = Duration::seconds(30);
    now += expires_in;
    let exp = now.timestamp() as usize;
    let claim = Claims { user_id, iat, exp };

    let jwt_secret = get_jwt_secret();
    let key = EncodingKey::from_secret(jwt_secret.as_ref());
    jsonwebtoken::encode(&Header::default(), &claim, &key).unwrap()
}

pub fn jwt_decode(token: &str) -> Result<TokenPayload, jsonwebtoken::errors::Error> {
    let jwt_secret = get_jwt_secret();
    let token_data_result = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default());

    match token_data_result {
        Ok(token_data) => {
            let claims = token_data.claims;
            Ok(TokenPayload::new(claims.user_id))
        }
        Err(err) => Err(err)
    }
}

fn get_jwt_secret() -> String {
    env::var("JWT_SECRET").unwrap_or_else(|_| panic!("JWT_SECRET must be set!"))
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user_id: u32,
    // aud: String,         // Optional. Audience
    iat: usize,
    // Optional. Issued at (as UTC timestamp)
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    // iss: String,         // Optional. Issuer
    // nbf: usize,          // Optional. Not Before (as UTC timestamp)
    // sub: String,         // Optional. Subject (whom token refers to)
}
