use actix_web::{HttpResponse, Responder, web};
use crate::{
    config::database::DbPool,
    models::auth::{LoginRequest},
    services::auth_service::AuthService,
};

pub async fn login(pool: web::Data<DbPool>, login_request: web::Json<LoginRequest>) -> impl Responder {
    let login_response = AuthService::authenticate(&pool, &login_request).await;

    match login_response {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(err) => HttpResponse::Unauthorized().body(err),
    }
}
