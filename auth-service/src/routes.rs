use actix_web::{web, Scope};
use crate::handlers::auth::{login};

pub fn auth_routes() -> Scope {
    web::scope("/auth")
        .route("/login", web::post().to(login))
}
