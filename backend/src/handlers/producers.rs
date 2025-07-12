use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

use crate::{
    config::AppConfig,
    db::DbPool,
    error::AppError,
    models::producer::Producer,
    // Apuntar a los DTOs de kairos_common
    kairos_common::{PaginationParams, SearchParams, UpdateProducerRequest},
};

pub async fn list_producers(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    pagination: web::Query<PaginationParams>,
    search: web::Query<SearchParams>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let producers = Producer::find_all(conn, offset, per_page)?;

    Ok(HttpResponse::Ok().json(json!({
        "producers": producers,
        "page": page,
        "per_page": per_page,
        "total": producers.len()
    })))
}

pub async fn get_producer(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    producer_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let producer = Producer::find_by_id(conn, producer_id.into_inner())?;

    Ok(HttpResponse::Ok().json(producer))
}

pub async fn update_producer(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    producer_id: web::Path<Uuid>,
    request: web::Json<UpdateProducerRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    // La conversión se hace automáticamente con .into() gracias al trait `From`
    let producer = Producer::update(conn, producer_id.into_inner(), request.into_inner().into())?;

    Ok(HttpResponse::Ok().json(producer))
}

pub async fn delete_producer(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    producer_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    Producer::delete(conn, producer_id.into_inner())?;

    Ok(HttpResponse::NoContent().finish())
} 