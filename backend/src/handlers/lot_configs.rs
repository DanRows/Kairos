use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

use crate::{
    config::AppConfig,
    db::DbPool,
    error::AppError,
    models::lot_config::{LotConfig, NewLotConfig, UpdateLotConfig},
    schemas::lot_configs::{CreateLotConfigRequest, UpdateLotConfigRequest},
    schemas::common::{PaginationParams, SearchParams},
};

pub async fn create_lot_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    request: web::Json<CreateLotConfigRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let new_lot_config = NewLotConfig {
        lot_id: request.lot_id,
        config_type: request.config_type,
        key: request.key.clone(),
        value: request.value.clone(),
        metadata: request.metadata.clone(),
    };

    let lot_config = LotConfig::create(conn, new_lot_config)?;

    Ok(HttpResponse::Created().json(lot_config))
}

pub async fn list_lot_configs(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    pagination: web::Query<PaginationParams>,
    search: web::Query<SearchParams>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let lot_configs = if let Some(lot_id) = search.lot_id {
        LotConfig::find_by_lot(conn, lot_id)?
    } else {
        LotConfig::find_all(conn, offset, per_page)?
    };

    Ok(HttpResponse::Ok().json(json!({
        "lot_configs": lot_configs,
        "page": page,
        "per_page": per_page,
        "total": lot_configs.len()
    })))
}

pub async fn get_lot_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    lot_config_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let lot_config = LotConfig::find_by_id(conn, lot_config_id.into_inner())?;

    Ok(HttpResponse::Ok().json(lot_config))
}

pub async fn update_lot_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    lot_config_id: web::Path<Uuid>,
    request: web::Json<UpdateLotConfigRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let update_lot_config = UpdateLotConfig {
        config_type: request.config_type,
        key: request.key.clone(),
        value: request.value.clone(),
        metadata: request.metadata.clone(),
    };

    let lot_config = LotConfig::update(conn, lot_config_id.into_inner(), update_lot_config)?;

    Ok(HttpResponse::Ok().json(lot_config))
}

pub async fn delete_lot_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    lot_config_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    LotConfig::delete(conn, lot_config_id.into_inner())?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn delete_all_lot_configs(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    lot_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    LotConfig::delete_all_by_lot(conn, lot_id.into_inner())?;

    Ok(HttpResponse::NoContent().finish())
} 