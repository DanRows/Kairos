use actix_web::{web, HttpResponse};

use crate::{
    database::DbPool,
    errors::AppError,
    models::producer::Producer,
    auth::jwt::create_token,
};
// Apuntar a los DTOs de kairos_common, corrigiendo la ruta
use kairos_common::{LoginRequest, RegisterProducerRequest, TokenResponse};

pub async fn register(
    pool: web::Data<DbPool>,
    create_request: web::Json<RegisterProducerRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;
    
    // Verificar si el email ya existe
    let existing_producer = Producer::find_by_email(&mut conn, &create_request.email);
    if existing_producer.is_ok() {
        return Err(AppError::BadRequest("Email already exists".to_string()));
    }
    
    // Crear el productor
    let producer = Producer::create(&mut conn, create_request.into_inner())?;
    
    // Generar token JWT
    let token = create_token(
        &producer.id.to_string(),
        &producer.email,
        "producer",
        "your-secret-key", // ¡Esto debería venir de la configuración!
    )?;
    
    Ok(HttpResponse::Created().json(token))
}

pub async fn login(
    pool: web::Data<DbPool>,
    request: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = pool.get()?;
    
    // Buscar el productor por email
    let producer = Producer::authenticate(&mut conn, &request.email, &request.password)
        .map_err(|_| AppError::Unauthorized("Invalid credentials".to_string()))?;
    
    // Generar token JWT
    let token = create_token(
        &producer.id.to_string(),
        &producer.email,
        "producer",
        "your-secret-key", // ¡Esto debería venir de la configuración!
    )?;
    
    Ok(HttpResponse::Ok().json(token))
}

pub async fn get_profile(
    _pool: web::Data<DbPool>,
    producer: web::ReqData<Producer>,
) -> Result<HttpResponse, AppError> {
    let producer = producer.into_inner();
    Ok(HttpResponse::Ok().json(producer))
} 