use jsonwebtoken::{encode, EncodingKey, Header, DecodingKey, Validation, Algorithm, decode};
use serde::{Deserialize, Serialize};
use crate::config::Config;
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: uuid::Uuid,
    pub email: String,
    pub exp: i64,
}

impl Claims {
    pub fn new(user_id: uuid::Uuid, email: String, expiration_seconds: i64) -> Self {
        Self {
            user_id,
            email,
            exp: Utc::now().timestamp() + expiration_seconds,
        }
    }
}

pub fn create_token(user_id: uuid::Uuid, email: String, config: &Config) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims::new(user_id, email, config.jwt_expiration);
    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    )
}

pub fn create_refresh_token(user_id: uuid::Uuid, email: String) -> Result<String, jsonwebtoken::errors::Error> {
    let config = crate::config::Config::from_env();
    let claims = Claims::new(user_id, email, 7 * 24 * 60 * 60); // 7 days
    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    )
}

pub fn verify_token(token: &str, config: &Config) -> Result<Claims, jsonwebtoken::errors::Error> {
    let decoding_key = DecodingKey::from_secret(config.jwt_secret.as_ref());
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
    Ok(token_data.claims)
}

