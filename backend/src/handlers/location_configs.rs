use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

use crate::{
    config::AppConfig,
    db::DbPool,
    error::AppError,
    models::location_config::{LocationConfig, NewLocationConfig, UpdateLocationConfig},
    schemas::location_configs::{CreateLocationConfigRequest, UpdateLocationConfigRequest},
    schemas::common::{PaginationParams, SearchParams},
};

pub async fn create_location_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    request: web::Json<CreateLocationConfigRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let new_location_config = NewLocationConfig {
        location_id: request.location_id,
        config_type: request.config_type,
        key: request.key.clone(),
        value: request.value.clone(),
        metadata: request.metadata.clone(),
    };

    let location_config = LocationConfig::create(conn, new_location_config)?;

    Ok(HttpResponse::Created().json(location_config))
}

pub async fn list_location_configs(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    pagination: web::Query<PaginationParams>,
    search: web::Query<SearchParams>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let location_configs = if let Some(location_id) = search.location_id {
        LocationConfig::find_by_location(conn, location_id)?
    } else {
        LocationConfig::find_all(conn, offset, per_page)?
    };

    Ok(HttpResponse::Ok().json(json!({
        "location_configs": location_configs,
        "page": page,
        "per_page": per_page,
        "total": location_configs.len()
    })))
}

pub async fn get_location_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    location_config_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let location_config = LocationConfig::find_by_id(conn, location_config_id.into_inner())?;

    Ok(HttpResponse::Ok().json(location_config))
}

pub async fn update_location_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    location_config_id: web::Path<Uuid>,
    request: web::Json<UpdateLocationConfigRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let update_location_config = UpdateLocationConfig {
        config_type: request.config_type,
        key: request.key.clone(),
        value: request.value.clone(),
        metadata: request.metadata.clone(),
    };

    let location_config = LocationConfig::update(conn, location_config_id.into_inner(), update_location_config)?;

    Ok(HttpResponse::Ok().json(location_config))
}

pub async fn delete_location_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    location_config_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    LocationConfig::delete(conn, location_config_id.into_inner())?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn delete_all_location_configs(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    location_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    LocationConfig::delete_all_by_location(conn, location_id.into_inner())?;

    Ok(HttpResponse::NoContent().finish())
} 