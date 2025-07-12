use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

use crate::{
    config::AppConfig,
    db::DbPool,
    error::AppError,
    models::brand_config::{BrandConfig, NewBrandConfig, UpdateBrandConfig},
    schemas::brand_configs::{CreateBrandConfigRequest, UpdateBrandConfigRequest},
    schemas::common::{PaginationParams, SearchParams},
};

pub async fn create_brand_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    request: web::Json<CreateBrandConfigRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let new_brand_config = NewBrandConfig {
        brand_id: request.brand_id,
        config_type: request.config_type,
        key: request.key.clone(),
        value: request.value.clone(),
        metadata: request.metadata.clone(),
    };

    let brand_config = BrandConfig::create(conn, new_brand_config)?;

    Ok(HttpResponse::Created().json(brand_config))
}

pub async fn list_brand_configs(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    pagination: web::Query<PaginationParams>,
    search: web::Query<SearchParams>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let brand_configs = if let Some(brand_id) = search.brand_id {
        BrandConfig::find_by_brand(conn, brand_id)?
    } else {
        BrandConfig::find_all(conn, offset, per_page)?
    };

    Ok(HttpResponse::Ok().json(json!({
        "brand_configs": brand_configs,
        "page": page,
        "per_page": per_page,
        "total": brand_configs.len()
    })))
}

pub async fn get_brand_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    brand_config_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let brand_config = BrandConfig::find_by_id(conn, brand_config_id.into_inner())?;

    Ok(HttpResponse::Ok().json(brand_config))
}

pub async fn update_brand_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    brand_config_id: web::Path<Uuid>,
    request: web::Json<UpdateBrandConfigRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let update_brand_config = UpdateBrandConfig {
        config_type: request.config_type,
        key: request.key.clone(),
        value: request.value.clone(),
        metadata: request.metadata.clone(),
    };

    let brand_config = BrandConfig::update(conn, brand_config_id.into_inner(), update_brand_config)?;

    Ok(HttpResponse::Ok().json(brand_config))
}

pub async fn delete_brand_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    brand_config_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    BrandConfig::delete(conn, brand_config_id.into_inner())?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn delete_all_brand_configs(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    brand_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    BrandConfig::delete_all_by_brand(conn, brand_id.into_inner())?;

    Ok(HttpResponse::NoContent().finish())
} 