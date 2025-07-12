use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    http::header,
    Error, HttpMessage, HttpRequest,
};
use futures_util::future::{ready, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::{
    auth::Claims,
    config::AppConfig,
    models::producer::Producer,
    errors::AppError,
};

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + Clone + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService { service }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + Clone + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Extraer el token del header Authorization
        let auth_header = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
            .map(|s| s.to_string());

        let config = req.app_data::<actix_web::web::Data<AppConfig>>()
            .cloned()
            .map(|c| c.get_ref().clone());

        let service = self.service.clone();

        Box::pin(async move {
            if let Some(token) = auth_header {
                if let Some(config) = config {
                    let validation = Validation::default();
                    let key = DecodingKey::from_secret(config.jwt_secret.as_ref());
                    
                    match decode::<Claims>(&token, &key, &validation) {
                        Ok(token_data) => {
                            // Crear un nuevo request con los claims
                            let req = req;
                            req.extensions_mut().insert(token_data.claims);
                            let res = service.call(req).await?;
                            Ok(res)
                        }
                        Err(_) => {
                            Err(ErrorUnauthorized("Invalid token"))
                        }
                    }
                } else {
                    Err(ErrorUnauthorized("Configuration not available"))
                }
            } else {
                Err(ErrorUnauthorized("Missing authorization header"))
            }
        })
    }
}

pub struct ProducerAuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for ProducerAuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + Clone + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ProducerAuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ProducerAuthMiddlewareService { service }))
    }
}

pub struct ProducerAuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ProducerAuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + Clone + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Extraer el token del header Authorization
        let auth_header = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
            .map(|s| s.to_string());

        let config = req.app_data::<actix_web::web::Data<AppConfig>>()
            .cloned()
            .map(|c| c.get_ref().clone());

        let service = self.service.clone();

        Box::pin(async move {
            if let Some(token) = auth_header {
                if let Some(config) = config {
                    let validation = Validation::default();
                    let key = DecodingKey::from_secret(config.jwt_secret.as_ref());
                    
                    match decode::<Claims>(&token, &key, &validation) {
                        Ok(token_data) => {
                            // Extraer el producer_id del token
                            let producer_id = match uuid::Uuid::parse_str(&token_data.claims.sub.to_string()) {
                                Ok(id) => id,
                                Err(_) => {
                                    return Err(ErrorUnauthorized("Invalid producer ID in token"));
                                }
                            };

                            // Buscar el productor en la base de datos
                            let pool = req.app_data::<actix_web::web::Data<crate::database::DbPool>>()
                                .ok_or_else(|| ErrorUnauthorized("Database pool not available"))?;

                            let mut conn = pool.get()
                                .map_err(|_| ErrorUnauthorized("Database connection failed"))?;

                            let producer = crate::models::producer::Producer::find_by_id(&mut conn, producer_id)
                                .map_err(|_| ErrorUnauthorized("Producer not found"))?;

                            // Verificar que el productor está activo
                            if !producer.is_active {
                                return Err(ErrorUnauthorized("Producer account is inactive"));
                            }

                            // Crear un nuevo request con el productor y los claims
                            let req = req;
                            req.extensions_mut().insert(producer);
                            req.extensions_mut().insert(token_data.claims);

                            // Continuar con el request
                            let res = service.call(req).await?;
                            Ok(res)
                        }
                        Err(_) => {
                            Err(ErrorUnauthorized("Invalid token"))
                        }
                    }
                } else {
                    Err(ErrorUnauthorized("Configuration not available"))
                }
            } else {
                Err(ErrorUnauthorized("Missing authorization header"))
            }
        })
    }
}

// Función helper para extraer el productor del request
pub fn get_producer_from_request(req: &HttpRequest) -> Result<Producer, AppError> {
    req.extensions()
        .get::<Producer>()
        .cloned()
        .ok_or_else(|| AppError::Unauthorized("Producer not found in request".into()))
}