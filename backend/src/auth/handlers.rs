use actix_web::{web, HttpResponse, Responder};
use crate::{
    database::DbPool,
    errors::AppError,
    models::producer::Producer,
    auth::jwt::create_token,
};
use kairos_common::{LoginRequest, TokenResponse, RegisterProducerRequest};

pub async fn login(
    pool: web::Data<DbPool>,
    request: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get().map_err(AppError::from)?;
    
    let producer = Producer::authenticate(&mut conn, &request.email, &request.password)?;
    
    let token = create_token(&producer.id.to_string(), &producer.email, "producer", "your-secret-key")?;
    
    Ok(HttpResponse::Ok().json(TokenResponse {
        access_token: token.access_token,
        token_type: token.token_type,
        expires_in: token.expires_in,
    }))
}

pub async fn register(
    pool: web::Data<DbPool>,
    request: web::Json<RegisterProducerRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get().map_err(AppError::from)?;
    
    // Verificar si el email ya existe
    if Producer::find_by_email(&mut conn, &request.email).is_ok() {
        return Err(AppError::BadRequest("Email already exists".to_string()));
    }
    
    let producer = Producer::create(&mut conn, request.into_inner())?;
    
    let token = create_token(&producer.id.to_string(), &producer.email, "producer", "your-secret-key")?;
    
    Ok(HttpResponse::Created().json(TokenResponse {
        access_token: token.access_token,
        token_type: token.token_type,
        expires_in: token.expires_in,
    }))
}

pub async fn logout(
    _pool: web::Data<DbPool>,
) -> Result<HttpResponse, AppError> {
    // En una implementación real, aquí invalidarías el token
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Logged out successfully"
    })))
}

pub async fn me(
    _pool: web::Data<DbPool>,
    producer: web::ReqData<Producer>,
) -> Result<HttpResponse, AppError> {
    let producer = producer.into_inner();
    Ok(HttpResponse::Ok().json(producer))
} 