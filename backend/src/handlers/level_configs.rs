use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

use crate::{
    config::AppConfig,
    db::DbPool,
    error::AppError,
    models::level_config::{LevelConfig, NewLevelConfig, UpdateLevelConfig},
    schemas::level_configs::{CreateLevelConfigRequest, UpdateLevelConfigRequest},
    schemas::common::{PaginationParams, SearchParams},
};

pub async fn create_level_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    request: web::Json<CreateLevelConfigRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let new_level_config = NewLevelConfig {
        level_id: request.level_id,
        config_type: request.config_type,
        key: request.key.clone(),
        value: request.value.clone(),
        metadata: request.metadata.clone(),
    };

    let level_config = LevelConfig::create(conn, new_level_config)?;

    Ok(HttpResponse::Created().json(level_config))
}

pub async fn list_level_configs(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    pagination: web::Query<PaginationParams>,
    search: web::Query<SearchParams>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let level_configs = if let Some(level_id) = search.level_id {
        LevelConfig::find_by_level(conn, level_id)?
    } else {
        LevelConfig::find_all(conn, offset, per_page)?
    };

    Ok(HttpResponse::Ok().json(json!({
        "level_configs": level_configs,
        "page": page,
        "per_page": per_page,
        "total": level_configs.len()
    })))
}

pub async fn get_level_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    level_config_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let level_config = LevelConfig::find_by_id(conn, level_config_id.into_inner())?;

    Ok(HttpResponse::Ok().json(level_config))
}

pub async fn update_level_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    level_config_id: web::Path<Uuid>,
    request: web::Json<UpdateLevelConfigRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let update_level_config = UpdateLevelConfig {
        config_type: request.config_type,
        key: request.key.clone(),
        value: request.value.clone(),
        metadata: request.metadata.clone(),
    };

    let level_config = LevelConfig::update(conn, level_config_id.into_inner(), update_level_config)?;

    Ok(HttpResponse::Ok().json(level_config))
}

pub async fn delete_level_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    level_config_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    LevelConfig::delete(conn, level_config_id.into_inner())?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn delete_all_level_configs(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    level_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    LevelConfig::delete_all_by_level(conn, level_id.into_inner())?;

    Ok(HttpResponse::NoContent().finish())
} 