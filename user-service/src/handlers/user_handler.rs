use actix_web::{web, HttpResponse, Responder};
use validator::Validate;

use crate::config::database::DbPool;
use crate::models::user::{NewUser};

use crate::services::user_service::UserService;

pub async fn create_user(
	pool: web::Data<DbPool>,
	new_user: web::Json<NewUser>
) -> impl Responder {
    let new_user = new_user.into_inner();
    if let Err(e) = new_user.validate() {
        return HttpResponse::BadRequest().json(format!("Invalid input: {}", e));
    }

    match UserService::create_user(&pool, new_user).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}


// pub async fn create_user(
//     pool: web::Data<DbPool>,
//     new_user: web::Json<NewUser>
// ) -> impl Responder {
//     let conn = &mut pool.get().expect("F banco de dados");
//     let hashed_password = hash_password(&new_user.password).unwrap().to_string();
//     let new_user = NewUser {
//         password: hashed_password,
//         ..new_user.into_inner()
//     };
//
//     let user_created = diesel::insert_into(users)
//         .values(new_user)
//         .execute(conn);
//
//     match user_created {
//         Ok(_) => HttpResponse::Created().body("User created successful"),
//         Err(_) => HttpResponse::InternalServerError().body("Error to trying create user!")
//     }
// }
