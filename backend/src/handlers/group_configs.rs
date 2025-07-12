use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

use crate::{
    config::AppConfig,
    db::DbPool,
    error::AppError,
    models::group_config::{GroupConfig, NewGroupConfig, UpdateGroupConfig},
    schemas::group_configs::{CreateGroupConfigRequest, UpdateGroupConfigRequest},
    schemas::common::{PaginationParams, SearchParams},
};

pub async fn create_group_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    request: web::Json<CreateGroupConfigRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let new_group_config = NewGroupConfig {
        group_id: request.group_id,
        config_type: request.config_type,
        key: request.key.clone(),
        value: request.value.clone(),
        metadata: request.metadata.clone(),
    };

    let group_config = GroupConfig::create(conn, new_group_config)?;

    Ok(HttpResponse::Created().json(group_config))
}

pub async fn list_group_configs(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    pagination: web::Query<PaginationParams>,
    search: web::Query<SearchParams>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let group_configs = if let Some(group_id) = search.group_id {
        GroupConfig::find_by_group(conn, group_id)?
    } else {
        GroupConfig::find_all(conn, offset, per_page)?
    };

    Ok(HttpResponse::Ok().json(json!({
        "group_configs": group_configs,
        "page": page,
        "per_page": per_page,
        "total": group_configs.len()
    })))
}

pub async fn get_group_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    group_config_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let group_config = GroupConfig::find_by_id(conn, group_config_id.into_inner())?;

    Ok(HttpResponse::Ok().json(group_config))
}

pub async fn update_group_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    group_config_id: web::Path<Uuid>,
    request: web::Json<UpdateGroupConfigRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let update_group_config = UpdateGroupConfig {
        config_type: request.config_type,
        key: request.key.clone(),
        value: request.value.clone(),
        metadata: request.metadata.clone(),
    };

    let group_config = GroupConfig::update(conn, group_config_id.into_inner(), update_group_config)?;

    Ok(HttpResponse::Ok().json(group_config))
}

pub async fn delete_group_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    group_config_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    GroupConfig::delete(conn, group_config_id.into_inner())?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn delete_all_group_configs(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    group_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    GroupConfig::delete_all_by_group(conn, group_id.into_inner())?;

    Ok(HttpResponse::NoContent().finish())
} 