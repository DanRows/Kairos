use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use crate::{
    models::{lot::Lot, producer::Producer},
    errors::AppError
};
use kairos_common::{CreateLotRequest, UpdateLotRequest};
use crate::database::DbPool;

pub fn configure() -> actix_web::Scope {
    web::scope("/lots")
        .route("", web::post().to(create_lot))
        .route("", web::get().to(list_lots))
        .route("/{id}", web::get().to(get_lot))
        .route("/{id}", web::put().to(update_lot))
        .route("/{id}", web::delete().to(delete_lot))
}

pub async fn create_lot(
    pool: web::Data<DbPool>,
    producer: web::ReqData<Producer>, // Obtener el productor autenticado
    request: web::Json<CreateLotRequest>,
) -> Result<HttpResponse, AppError> {
    let conn = &mut pool.get()?;
    let producer_id = producer.into_inner().id;

    let lot = Lot::create(conn, producer_id, request.into_inner())?;
    
    Ok(HttpResponse::Created().json(lot))
}

pub async fn list_lots(
    pool: web::Data<DbPool>,
    producer_id: web::Query<Uuid>,
) -> impl Responder {
    let conn = &mut pool.get().unwrap();

    match Lot::find_by_producer(conn, producer_id.into_inner(), 1, 10) {
        Ok(lots) => HttpResponse::Ok().json(lots),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn get_lot(
    pool: web::Data<DbPool>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let conn = &mut pool.get().unwrap();

    match Lot::find_by_id(conn, id.into_inner()) {
        Ok(lot) => HttpResponse::Ok().json(lot),
        Err(e) => HttpResponse::NotFound().json(e.to_string()),
    }
}

pub async fn update_lot(
    pool: web::Data<DbPool>,
    id: web::Path<Uuid>,
    request: web::Json<UpdateLotRequest>,
) -> impl Responder {
    let conn = &mut pool.get().unwrap();

    match Lot::update(conn, id.into_inner(), request.into_inner().into()) {
        Ok(lot) => HttpResponse::Ok().json(lot),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub async fn delete_lot(
    pool: web::Data<DbPool>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let conn = &mut pool.get().unwrap();

    match Lot::delete(conn, id.into_inner()) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
} 