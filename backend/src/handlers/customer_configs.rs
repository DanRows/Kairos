use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

use crate::{
    config::AppConfig,
    db::DbPool,
    error::AppError,
    models::customer_config::{CustomerConfig, NewCustomerConfig, UpdateCustomerConfig},
    schemas::customer_configs::{CreateCustomerConfigRequest, UpdateCustomerConfigRequest},
    schemas::common::{PaginationParams, SearchParams},
};

pub async fn create_customer_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    request: web::Json<CreateCustomerConfigRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let new_customer_config = NewCustomerConfig {
        customer_id: request.customer_id,
        config_type: request.config_type,
        key: request.key.clone(),
        value: request.value.clone(),
        metadata: request.metadata.clone(),
    };

    let customer_config = CustomerConfig::create(conn, new_customer_config)?;

    Ok(HttpResponse::Created().json(customer_config))
}

pub async fn list_customer_configs(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    pagination: web::Query<PaginationParams>,
    search: web::Query<SearchParams>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let customer_configs = if let Some(customer_id) = search.customer_id {
        CustomerConfig::find_by_customer(conn, customer_id)?
    } else {
        CustomerConfig::find_all(conn, offset, per_page)?
    };

    Ok(HttpResponse::Ok().json(json!({
        "customer_configs": customer_configs,
        "page": page,
        "per_page": per_page,
        "total": customer_configs.len()
    })))
}

pub async fn get_customer_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    customer_config_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let customer_config = CustomerConfig::find_by_id(conn, customer_config_id.into_inner())?;

    Ok(HttpResponse::Ok().json(customer_config))
}

pub async fn update_customer_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    customer_config_id: web::Path<Uuid>,
    request: web::Json<UpdateCustomerConfigRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let update_customer_config = UpdateCustomerConfig {
        config_type: request.config_type,
        key: request.key.clone(),
        value: request.value.clone(),
        metadata: request.metadata.clone(),
    };

    let customer_config = CustomerConfig::update(conn, customer_config_id.into_inner(), update_customer_config)?;

    Ok(HttpResponse::Ok().json(customer_config))
}

pub async fn delete_customer_config(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    customer_config_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    CustomerConfig::delete(conn, customer_config_id.into_inner())?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn delete_all_customer_configs(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    customer_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    CustomerConfig::delete_all_by_customer(conn, customer_id.into_inner())?;

    Ok(HttpResponse::NoContent().finish())
} 