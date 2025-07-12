use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use serde_json::json;
use uuid::Uuid;

use crate::{
    config::AppConfig,
    db::DbPool,
    error::AppError,
    models::payment::{Payment, NewPayment, UpdatePayment},
    schemas::payments::{CreatePaymentRequest, UpdatePaymentRequest},
    schemas::common::{PaginationParams, SearchParams},
};

pub async fn create_payment(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    request: web::Json<CreatePaymentRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let new_payment = NewPayment {
        purchase_id: request.purchase_id,
        amount: request.amount,
        payment_method: request.payment_method,
        status: request.status,
        metadata: request.metadata.clone(),
    };

    let payment = Payment::create(conn, new_payment)?;

    Ok(HttpResponse::Created().json(payment))
}

pub async fn list_payments(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    pagination: web::Query<PaginationParams>,
    search: web::Query<SearchParams>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let payments = if let Some(purchase_id) = search.purchase_id {
        Payment::find_by_purchase(conn, purchase_id)?
    } else {
        Payment::find_all(conn, offset, per_page)?
    };

    Ok(HttpResponse::Ok().json(json!({
        "payments": payments,
        "page": page,
        "per_page": per_page,
        "total": payments.len()
    })))
}

pub async fn get_payment(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    payment_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let payment = Payment::find_by_id(conn, payment_id.into_inner())?;

    Ok(HttpResponse::Ok().json(payment))
}

pub async fn update_payment(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    payment_id: web::Path<Uuid>,
    request: web::Json<UpdatePaymentRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    let update_payment = UpdatePayment {
        amount: request.amount,
        payment_method: request.payment_method,
        status: request.status,
        metadata: request.metadata.clone(),
    };

    let payment = Payment::update(conn, payment_id.into_inner(), update_payment)?;

    Ok(HttpResponse::Ok().json(payment))
}

pub async fn delete_payment(
    pool: web::Data<DbPool>,
    config: web::Data<AppConfig>,
    payment_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;

    Payment::delete(conn, payment_id.into_inner())?;

    Ok(HttpResponse::NoContent().finish())
} 