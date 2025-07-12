use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

use crate::{
    config::AppConfig,
    db::DbPool,
    error::AppError,
    models::log::{Log, NewLog, UpdateLog},
    schemas::logs::{CreateLogRequest, UpdateLogRequest},
    schemas::common::{PaginationParams, SearchParams},
};

pub async fn create_log(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    request: web::Json<CreateLogRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let new_log = NewLog {
        user_id: request.user_id,
        log_type: request.log_type,
        level: request.level,
        message: request.message.clone(),
        metadata: request.metadata.clone(),
    };

    let log = Log::create(conn, new_log)?;

    Ok(HttpResponse::Created().json(log))
}

pub async fn list_logs(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    pagination: web::Query<PaginationParams>,
    search: web::Query<SearchParams>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let logs = if let Some(user_id) = search.user_id {
        Log::find_by_user(conn, user_id)?
    } else {
        Log::find_all(conn, offset, per_page)?
    };

    Ok(HttpResponse::Ok().json(json!({
        "logs": logs,
        "page": page,
        "per_page": per_page,
        "total": logs.len()
    })))
}

pub async fn get_log(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    log_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let log = Log::find_by_id(conn, log_id.into_inner())?;

    Ok(HttpResponse::Ok().json(log))
}

pub async fn update_log(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    log_id: web::Path<Uuid>,
    request: web::Json<UpdateLogRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let update_log = UpdateLog {
        log_type: request.log_type,
        level: request.level,
        message: request.message.clone(),
        metadata: request.metadata.clone(),
    };

    let log = Log::update(conn, log_id.into_inner(), update_log)?;

    Ok(HttpResponse::Ok().json(log))
}

pub async fn delete_log(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    log_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    Log::delete(conn, log_id.into_inner())?;

    Ok(HttpResponse::NoContent().finish())
} 