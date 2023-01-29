use chrono::Duration;
use std::env;
use serde::{Serialize, Deserialize};
use axum::http::StatusCode;
use jsonwebtoken::{EncodingKey, Header, decode, Validation, DecodingKey};


pub fn jwt_create() -> Result<String, StatusCode> {
    let mut now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let expires_in = Duration::seconds(30);
    now += expires_in;
    let exp = now.timestamp() as usize;
    let claim = Claims {
        iat,
        exp,
    };

    let jwt_secret = get_jwt_secret();
    let key = EncodingKey::from_secret(jwt_secret.as_ref());
    let token = jsonwebtoken::encode(&Header::default(), &claim, &key)
        .map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)
        .unwrap();

    Ok(token)
}

pub fn jwt_verify(token: &str) -> Result<(), jsonwebtoken::errors::Error> {
    let jwt_secret = get_jwt_secret();
    let key = DecodingKey::from_secret(jwt_secret.as_ref());
    let validation = &Validation::default();

    let is_valid  = decode::<Claims>(token, &key, validation)
        .map(|_v| true);

    match is_valid {
        Ok(_v) => return Ok(()),
        Err(err) => return Err(err)
    };
}


fn get_jwt_secret() -> String {
    env::var("JWT_SECRET")
        .unwrap_or_else(|_| panic!("JWT_SECRET must be set!"))
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    // aud: String,         // Optional. Audience
    iat: usize,          // Optional. Issued at (as UTC timestamp)
    exp: usize,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    // iss: String,         // Optional. Issuer
    // nbf: usize,          // Optional. Not Before (as UTC timestamp)
    // sub: String,         // Optional. Subject (whom token refers to)
}