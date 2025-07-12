use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

use crate::{
    config::AppConfig,
    db::DbPool,
    error::AppError,
    models::department_config::{DepartmentConfig, NewDepartmentConfig, UpdateDepartmentConfig},
    schemas::department_configs::{CreateDepartmentConfigRequest, UpdateDepartmentConfigRequest},
    schemas::common::{PaginationParams, SearchParams},
};

pub async fn create_department_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    request: web::Json<CreateDepartmentConfigRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let new_department_config = NewDepartmentConfig {
        department_id: request.department_id,
        config_type: request.config_type,
        key: request.key.clone(),
        value: request.value.clone(),
        metadata: request.metadata.clone(),
    };

    let department_config = DepartmentConfig::create(conn, new_department_config)?;

    Ok(HttpResponse::Created().json(department_config))
}

pub async fn list_department_configs(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    pagination: web::Query<PaginationParams>,
    search: web::Query<SearchParams>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let department_configs = if let Some(department_id) = search.department_id {
        DepartmentConfig::find_by_department(conn, department_id)?
    } else {
        DepartmentConfig::find_all(conn, offset, per_page)?
    };

    Ok(HttpResponse::Ok().json(json!({
        "department_configs": department_configs,
        "page": page,
        "per_page": per_page,
        "total": department_configs.len()
    })))
}

pub async fn get_department_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    department_config_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let department_config = DepartmentConfig::find_by_id(conn, department_config_id.into_inner())?;

    Ok(HttpResponse::Ok().json(department_config))
}

pub async fn update_department_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    department_config_id: web::Path<Uuid>,
    request: web::Json<UpdateDepartmentConfigRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let update_department_config = UpdateDepartmentConfig {
        config_type: request.config_type,
        key: request.key.clone(),
        value: request.value.clone(),
        metadata: request.metadata.clone(),
    };

    let department_config = DepartmentConfig::update(conn, department_config_id.into_inner(), update_department_config)?;

    Ok(HttpResponse::Ok().json(department_config))
}

pub async fn delete_department_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    department_config_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    DepartmentConfig::delete(conn, department_config_id.into_inner())?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn delete_all_department_configs(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    department_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    DepartmentConfig::delete_all_by_department(conn, department_id.into_inner())?;

    Ok(HttpResponse::NoContent().finish())
} 