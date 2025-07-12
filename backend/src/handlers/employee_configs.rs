use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

use crate::{
    config::AppConfig,
    db::DbPool,
    error::AppError,
    models::employee_config::{EmployeeConfig, NewEmployeeConfig, UpdateEmployeeConfig},
    schemas::employee_configs::{CreateEmployeeConfigRequest, UpdateEmployeeConfigRequest},
    schemas::common::{PaginationParams, SearchParams},
};

pub async fn create_employee_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    request: web::Json<CreateEmployeeConfigRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let new_employee_config = NewEmployeeConfig {
        employee_id: request.employee_id,
        config_type: request.config_type,
        key: request.key.clone(),
        value: request.value.clone(),
        metadata: request.metadata.clone(),
    };

    let employee_config = EmployeeConfig::create(conn, new_employee_config)?;

    Ok(HttpResponse::Created().json(employee_config))
}

pub async fn list_employee_configs(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    pagination: web::Query<PaginationParams>,
    search: web::Query<SearchParams>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let employee_configs = if let Some(employee_id) = search.employee_id {
        EmployeeConfig::find_by_employee(conn, employee_id)?
    } else {
        EmployeeConfig::find_all(conn, offset, per_page)?
    };

    Ok(HttpResponse::Ok().json(json!({
        "employee_configs": employee_configs,
        "page": page,
        "per_page": per_page,
        "total": employee_configs.len()
    })))
}

pub async fn get_employee_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    employee_config_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let employee_config = EmployeeConfig::find_by_id(conn, employee_config_id.into_inner())?;

    Ok(HttpResponse::Ok().json(employee_config))
}

pub async fn update_employee_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    employee_config_id: web::Path<Uuid>,
    request: web::Json<UpdateEmployeeConfigRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let update_employee_config = UpdateEmployeeConfig {
        config_type: request.config_type,
        key: request.key.clone(),
        value: request.value.clone(),
        metadata: request.metadata.clone(),
    };

    let employee_config = EmployeeConfig::update(conn, employee_config_id.into_inner(), update_employee_config)?;

    Ok(HttpResponse::Ok().json(employee_config))
}

pub async fn delete_employee_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    employee_config_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    EmployeeConfig::delete(conn, employee_config_id.into_inner())?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn delete_all_employee_configs(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    employee_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    EmployeeConfig::delete_all_by_employee(conn, employee_id.into_inner())?;

    Ok(HttpResponse::NoContent().finish())
} 