use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

use crate::{
    config::AppConfig,
    db::DbPool,
    error::AppError,
    models::permission::{Permission, NewPermission, UpdatePermission},
    schemas::permissions::{CreatePermissionRequest, UpdatePermissionRequest},
    schemas::common::{PaginationParams, SearchParams},
};

pub async fn create_permission(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    request: web::Json<CreatePermissionRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let new_permission = NewPermission {
        name: request.name.clone(),
        description: request.description.clone(),
        resource: request.resource.clone(),
        action: request.action.clone(),
        metadata: request.metadata.clone(),
    };

    let permission = Permission::create(conn, new_permission)?;

    Ok(HttpResponse::Created().json(permission))
}

pub async fn list_permissions(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    pagination: web::Query<PaginationParams>,
    search: web::Query<SearchParams>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let permissions = Permission::find_all(conn, offset, per_page)?;

    Ok(HttpResponse::Ok().json(json!({
        "permissions": permissions,
        "page": page,
        "per_page": per_page,
        "total": permissions.len()
    })))
}

pub async fn get_permission(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    permission_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let permission = Permission::find_by_id(conn, permission_id.into_inner())?;

    Ok(HttpResponse::Ok().json(permission))
}

pub async fn update_permission(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    permission_id: web::Path<Uuid>,
    request: web::Json<UpdatePermissionRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let update_permission = UpdatePermission {
        name: request.name.clone(),
        description: request.description.clone(),
        resource: request.resource.clone(),
        action: request.action.clone(),
        metadata: request.metadata.clone(),
    };

    let permission = Permission::update(conn, permission_id.into_inner(), update_permission)?;

    Ok(HttpResponse::Ok().json(permission))
}

pub async fn delete_permission(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    permission_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    Permission::delete(conn, permission_id.into_inner())?;

    Ok(HttpResponse::NoContent().finish())
} 