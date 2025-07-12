use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

use crate::{
    config::AppConfig,
    db::DbPool,
    error::AppError,
    models::notification::{Notification, NewNotification, UpdateNotification},
    schemas::notifications::{CreateNotificationRequest, UpdateNotificationRequest},
    schemas::common::{PaginationParams, SearchParams},
};

pub async fn create_notification(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    request: web::Json<CreateNotificationRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let new_notification = NewNotification {
        user_id: request.user_id,
        title: request.title.clone(),
        message: request.message.clone(),
        notification_type: request.notification_type,
        is_read: false,
        metadata: request.metadata.clone(),
    };

    let notification = Notification::create(conn, new_notification)?;

    Ok(HttpResponse::Created().json(notification))
}

pub async fn list_notifications(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    pagination: web::Query<PaginationParams>,
    search: web::Query<SearchParams>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let notifications = if let Some(user_id) = search.user_id {
        Notification::find_by_user(conn, user_id)?
    } else {
        Notification::find_all(conn, offset, per_page)?
    };

    Ok(HttpResponse::Ok().json(json!({
        "notifications": notifications,
        "page": page,
        "per_page": per_page,
        "total": notifications.len()
    })))
}

pub async fn get_notification(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    notification_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let notification = Notification::find_by_id(conn, notification_id.into_inner())?;

    Ok(HttpResponse::Ok().json(notification))
}

pub async fn update_notification(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    notification_id: web::Path<Uuid>,
    request: web::Json<UpdateNotificationRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let update_notification = UpdateNotification {
        title: request.title.clone(),
        message: request.message.clone(),
        notification_type: request.notification_type,
        is_read: request.is_read,
        metadata: request.metadata.clone(),
    };

    let notification = Notification::update(conn, notification_id.into_inner(), update_notification)?;

    Ok(HttpResponse::Ok().json(notification))
}

pub async fn delete_notification(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    notification_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    Notification::delete(conn, notification_id.into_inner())?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn mark_as_read(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    notification_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let update_notification = UpdateNotification {
        title: None,
        message: None,
        notification_type: None,
        is_read: Some(true),
        metadata: None,
    };

    let notification = Notification::update(conn, notification_id.into_inner(), update_notification)?;

    Ok(HttpResponse::Ok().json(notification))
}

pub async fn mark_all_as_read(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    user_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    Notification::mark_all_as_read(conn, user_id.into_inner())?;

    Ok(HttpResponse::NoContent().finish())
} 