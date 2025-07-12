use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use crate::{
    models::event::Event,
    errors::AppError,
    database::DbPool
};
// Apuntar a los DTOs de kairos_common en lugar de los schemas internos
use kairos_common::{CreateEventRequest, UpdateEventRequest};


pub fn configure() -> actix_web::Scope {
    // Los eventos ahora se anidan bajo los lotes para tener el contexto del lot_id
    web::scope("/lots/{lot_id}/events")
        .route("", web::post().to(create_event))
        // Estas rutas probablemente deberían estar en un scope /events/{id} global
        // pero por ahora las mantenemos simples para la compilación.
        // .route("/{id}", web::get().to(get_event))
        // .route("/{id}", web::put().to(update_event))
        // .route("/{id}", web::delete().to(delete_event))
}

pub async fn create_event(
    pool: web::Data<DbPool>,
    path: web::Path<Uuid>, // Extraer el lot_id de la ruta
    request: web::Json<CreateEventRequest>,
) -> Result<HttpResponse, AppError> {
    let lot_id = path.into_inner();
    let mut conn = pool.get()?;

    let event = Event::create(&mut conn, lot_id, request.into_inner())?;
    Ok(HttpResponse::Created().json(event))
}

/*
// Estas funciones necesitan ser re-evaluadas en el contexto de las nuevas rutas
pub async fn get_event(
    pool: web::Data<DbPool>,
    id: web::Path<Uuid>,
) -> impl Responder {
// ...
}

pub async fn update_event(
    pool: web::Data<DbPool>,
    id: web::Path<Uuid>,
    request: web::Json<UpdateEventRequest>,
) -> impl Responder {
// ...
}

pub async fn delete_event(
    pool: web::Data<DbPool>,
    id: web::Path<Uuid>,
) -> impl Responder {
// ...
    }
*/ 