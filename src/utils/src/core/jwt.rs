use axum::http::StatusCode;
use chrono::Duration;
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

pub fn jwt_create() -> String {
    let mut now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let expires_in = Duration::seconds(30);
    now += expires_in;
    let exp = now.timestamp() as usize;
    let claim = Claims { iat, exp };

    let jwt_secret = get_jwt_secret();
    let key = EncodingKey::from_secret(jwt_secret.as_ref());
    jsonwebtoken::encode(&Header::default(), &claim, &key).unwrap()
}

pub fn jwt_verify(token: &str) -> Result<(), jsonwebtoken::errors::Error> {
    let jwt_secret = get_jwt_secret();
    let key = DecodingKey::from_secret(jwt_secret.as_ref());
    let validation = &Validation::default();

    let is_valid = decode::<Claims>(token, &key, validation).map(|_v| true);

    match is_valid {
        Ok(_v) => return Ok(()),
        Err(err) => return Err(err),
    };
}

fn get_jwt_secret() -> String {
    env::var("JWT_SECRET").unwrap_or_else(|_| panic!("JWT_SECRET must be set!"))
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    // aud: String,         // Optional. Audience
    iat: usize, // Optional. Issued at (as UTC timestamp)
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
                // iss: String,         // Optional. Issuer
                // nbf: usize,          // Optional. Not Before (as UTC timestamp)
                // sub: String,         // Optional. Subject (whom token refers to)
}
