use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

use crate::{
    config::AppConfig,
    db::DbPool,
    error::AppError,
    models::category_config::{CategoryConfig, NewCategoryConfig, UpdateCategoryConfig},
    schemas::category_configs::{CreateCategoryConfigRequest, UpdateCategoryConfigRequest},
    schemas::common::{PaginationParams, SearchParams},
};

pub async fn create_category_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    request: web::Json<CreateCategoryConfigRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let new_category_config = NewCategoryConfig {
        category_id: request.category_id,
        config_type: request.config_type,
        key: request.key.clone(),
        value: request.value.clone(),
        metadata: request.metadata.clone(),
    };

    let category_config = CategoryConfig::create(conn, new_category_config)?;

    Ok(HttpResponse::Created().json(category_config))
}

pub async fn list_category_configs(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    pagination: web::Query<PaginationParams>,
    search: web::Query<SearchParams>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let category_configs = if let Some(category_id) = search.category_id {
        CategoryConfig::find_by_category(conn, category_id)?
    } else {
        CategoryConfig::find_all(conn, offset, per_page)?
    };

    Ok(HttpResponse::Ok().json(json!({
        "category_configs": category_configs,
        "page": page,
        "per_page": per_page,
        "total": category_configs.len()
    })))
}

pub async fn get_category_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    category_config_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let category_config = CategoryConfig::find_by_id(conn, category_config_id.into_inner())?;

    Ok(HttpResponse::Ok().json(category_config))
}

pub async fn update_category_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    category_config_id: web::Path<Uuid>,
    request: web::Json<UpdateCategoryConfigRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let update_category_config = UpdateCategoryConfig {
        config_type: request.config_type,
        key: request.key.clone(),
        value: request.value.clone(),
        metadata: request.metadata.clone(),
    };

    let category_config = CategoryConfig::update(conn, category_config_id.into_inner(), update_category_config)?;

    Ok(HttpResponse::Ok().json(category_config))
}

pub async fn delete_category_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    category_config_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    CategoryConfig::delete(conn, category_config_id.into_inner())?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn delete_all_category_configs(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    category_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    CategoryConfig::delete_all_by_category(conn, category_id.into_inner())?;

    Ok(HttpResponse::NoContent().finish())
} 