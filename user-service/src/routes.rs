use actix_web::{web, Scope};
use crate::handlers::user_handler::{create_user};

pub fn user_routes() -> Scope {
    web::scope("/users")
        .route("/create", web::post().to(create_user))
        // .route("/{id}", web::get().to(get_user))
}
