pub mod handlers;
pub mod jwt;
pub mod middleware;

use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}

pub fn verify_token(token: &str, jwt_secret: &str) -> Result<Claims, crate::errors::AppError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|e| crate::errors::AppError::Unauthorized(format!("Token verification failed: {}", e)))?;

    Ok(token_data.claims)
} 