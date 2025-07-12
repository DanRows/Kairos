use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use crate::errors::AppError;
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};
use kairos_common::TokenResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(user_id: Uuid, expiration: i64) -> Self {
        let now = Utc::now();
        Self {
            sub: user_id,
            exp: (now + Duration::seconds(expiration)).timestamp(),
            iat: now.timestamp(),
        }
    }

    pub fn encode(&self, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
        encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
    }

    pub fn decode(token: &str, secret: &str) -> Result<Self, jsonwebtoken::errors::Error> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
    }
}

pub fn create_token(
    user_id: &str, 
    _email: &str, 
    _role: &str, 
    jwt_secret: &str
) -> Result<TokenResponse, AppError> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize + (24 * 60 * 60); // 24 horas

    let claims = Claims {
        sub: Uuid::parse_str(user_id)?,
        exp: expiration as i64,
        iat: Utc::now().timestamp(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .map_err(|e| AppError::JwtError(e))?;

    Ok(TokenResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
        expires_in: 24 * 60 * 60, // 24 horas en segundos
    })
}

pub fn validate_token(token: &str, jwt_secret: &str) -> Result<Claims, AppError> {
    let validation = Validation::default();
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &validation,
    )?;
    Ok(token_data.claims)
} 