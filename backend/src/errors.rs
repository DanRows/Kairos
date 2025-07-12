use actix_web::{HttpResponse, ResponseError};
use actix_web::error::BlockingError;
use bcrypt::BcryptError;
use diesel::result::Error as DieselError;
use r2d2;
use jsonwebtoken::errors::Error as JwtError;
use serde::Serialize;
use std::fmt;
use uuid::Error as UuidError; // Añadido para manejar errores de UUID

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    InternalServerError(String),
    DatabaseError(DieselError),
    PoolError(r2d2::Error), // Cambiado a la ruta canónica
    JwtError(JwtError),
    BcryptError(BcryptError),
    BlockingError(BlockingError),
    UuidError(UuidError), // Añadido
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            AppError::InternalServerError(msg) => write!(f, "Internal Server Error: {}", msg),
            AppError::DatabaseError(err) => write!(f, "Database Error: {}", err),
            AppError::PoolError(err) => write!(f, "Pool Error: {}", err),
            AppError::JwtError(err) => write!(f, "JWT Error: {}", err),
            AppError::BcryptError(err) => write!(f, "Bcrypt Error: {}", err),
            AppError::BlockingError(err) => write!(f, "Blocking Error: {}", err),
            AppError::UuidError(err) => write!(f, "UUID Error: {}", err), // Añadido
        }
    }
}

impl From<DieselError> for AppError {
    fn from(error: DieselError) -> AppError {
        match error {
            DieselError::NotFound => AppError::NotFound("Record not found".into()),
            _ => AppError::DatabaseError(error),
        }
    }
}

impl From<r2d2::Error> for AppError {
    fn from(error: r2d2::Error) -> AppError {
        AppError::PoolError(error)
    }
}

impl From<JwtError> for AppError {
    fn from(error: JwtError) -> AppError {
        AppError::JwtError(error)
    }
}

impl From<BcryptError> for AppError {
    fn from(error: BcryptError) -> AppError {
        AppError::BcryptError(error)
    }
}

impl From<BlockingError> for AppError {
    fn from(error: BlockingError) -> AppError {
        AppError::BlockingError(error)
    }
}

impl From<UuidError> for AppError {
    fn from(error: UuidError) -> AppError {
        AppError::UuidError(error)
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::BadRequest(msg) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    error: msg.to_string(),
                })
            }
            AppError::Unauthorized(msg) => {
                HttpResponse::Unauthorized().json(ErrorResponse {
                    error: msg.to_string(),
                })
            }
            AppError::Forbidden(msg) => {
                HttpResponse::Forbidden().json(ErrorResponse {
                    error: msg.to_string(),
                })
            }
            AppError::NotFound(msg) => {
                HttpResponse::NotFound().json(ErrorResponse {
                    error: msg.to_string(),
                })
            }
            AppError::InternalServerError(msg) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: msg.to_string(),
                })
            }
            AppError::DatabaseError(_) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database error".into(),
                })
            }
            AppError::PoolError(_) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Database pool error".into(),
                })
            }
            AppError::JwtError(_) => {
                HttpResponse::Unauthorized().json(ErrorResponse {
                    error: "Invalid token".into(),
                })
            }
            AppError::BcryptError(_) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Password hashing error".into(),
                })
            }
            AppError::BlockingError(_) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Blocking error".into(),
                })
            }
            AppError::UuidError(_) => HttpResponse::BadRequest().json(ErrorResponse {
                error: "Invalid ID format".into(),
            }),
        }
    }
} 