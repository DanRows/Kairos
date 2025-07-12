use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

use crate::{
    config::AppConfig,
    db::DbPool,
    error::AppError,
    models::config::{Config, NewConfig, UpdateConfig},
    schemas::configs::{CreateConfigRequest, UpdateConfigRequest},
    schemas::common::{PaginationParams, SearchParams},
};

pub async fn create_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    request: web::Json<CreateConfigRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let new_config = NewConfig {
        user_id: request.user_id,
        config_type: request.config_type,
        key: request.key.clone(),
        value: request.value.clone(),
        metadata: request.metadata.clone(),
    };

    let config = Config::create(conn, new_config)?;

    Ok(HttpResponse::Created().json(config))
}

pub async fn list_configs(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    pagination: web::Query<PaginationParams>,
    search: web::Query<SearchParams>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let configs = if let Some(user_id) = search.user_id {
        Config::find_by_user(conn, user_id)?
    } else {
        Config::find_all(conn, offset, per_page)?
    };

    Ok(HttpResponse::Ok().json(json!({
        "configs": configs,
        "page": page,
        "per_page": per_page,
        "total": configs.len()
    })))
}

pub async fn get_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    config_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let config = Config::find_by_id(conn, config_id.into_inner())?;

    Ok(HttpResponse::Ok().json(config))
}

pub async fn update_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    config_id: web::Path<Uuid>,
    request: web::Json<UpdateConfigRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let update_config = UpdateConfig {
        config_type: request.config_type,
        key: request.key.clone(),
        value: request.value.clone(),
        metadata: request.metadata.clone(),
    };

    let config = Config::update(conn, config_id.into_inner(), update_config)?;

    Ok(HttpResponse::Ok().json(config))
}

pub async fn delete_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    config_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    Config::delete(conn, config_id.into_inner())?;

    Ok(HttpResponse::NoContent().finish())
} 