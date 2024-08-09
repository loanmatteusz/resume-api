use actix_web::{App, test, web};
use diesel::prelude::*;
use log::info;
use lazy_static::lazy_static;
use std::sync::Once;
use serial_test::serial;
use uuid::Uuid;

use crate::config::database::establish_connection;
use crate::models::user::{NewUser, UpdateUser};
use crate::routes::user_routes;

lazy_static! {
    static ref INIT: Once = Once::new();
}

fn init_logger() {
  INIT.call_once(|| {
    env_logger::init();
  });
}

fn clear_users_table() {
  let pool = establish_connection();
  let conn = &mut pool.get().expect("Failed to get DB connection from pool");

  // Log antes de limpar a tabela
  let user_count: i64 = crate::models::schema::users::dsl::users
    .count()
    .get_result(conn)
    .expect("Failed to count users before clearing");
  info!("Number of users before clearing: {}", user_count);

  diesel::delete(crate::models::schema::users::dsl::users)
    .execute(conn)
    .expect("Error clearing users table");

  // Log depois de limpar a tabela
  let user_count_after: i64 = crate::models::schema::users::dsl::users
    .count()
    .get_result(conn)
    .expect("Failed to count users after clearing");
  info!("Number of users after clearing: {}", user_count_after);
}

#[actix_rt::test]
#[serial]
async fn test_create_user() {
  init_logger();
  info!("Teste de criação de usuário");
  info!("Limpando a tabela de usuários");
  clear_users_table();
  info!("Tabela de usuários limpa");

  // Given: A new user
  let pool = establish_connection();
  let new_user = NewUser {
    username: "testuser".to_string(),
    email: "testuser@example.com".to_string(),
    password: "Password123!".to_string(),
  };

  info!("Vai consumir o serviço");
  // When: The user is created
  let mut app = test::init_service(App::new()
    .app_data(web::Data::new(pool.clone()))
    .service(user_routes())).await;
  info!("Vai consumir, agora");

  let req = test::TestRequest::post()
    .uri("/users")
    .set_json(&new_user)
    .to_request();
  info!("Serviço consumido");

  let resp = test::call_service(&mut app, req).await;

  // Log response status and body for debugging
  let status = resp.status();
  let body = test::read_body(resp).await;
  info!("Create User Response Status: {:?}", status);
  info!("Create User Response Body: {:?}", body);

  // Then: The response status should be 200 OK
  assert_eq!(status, actix_web::http::StatusCode::OK);

  // Additional check to see the user is actually inserted
  let conn = &mut pool.get().expect("Failed to get DB connection from pool");
  let inserted_user: Option<(String, String)> = crate::models::schema::users::table
    .select((crate::models::schema::users::dsl::username, crate::models::schema::users::dsl::email))
    .first(conn)
    .optional()
    .expect("Failed to fetch inserted user");

  assert!(inserted_user.is_some());
  assert_eq!(inserted_user.unwrap(), ("testuser".to_string(), "testuser@example.com".to_string()));
}

#[actix_rt::test]
#[serial]
async fn test_create_user_with_invalid_email() {
  init_logger();
  clear_users_table();

  // Given: A new user with an invalid email
  let pool = establish_connection();
  let new_user = NewUser {
    username: "testuser".to_string(),
    email: "invalid_email".to_string(),
    password: "Password123!".to_string(),
  };

  // When: The user is created
  let mut app = test::init_service(App::new()
    .app_data(web::Data::new(pool.clone()))
    .service(user_routes())).await;

  let req = test::TestRequest::post()
    .uri("/users")
    .set_json(&new_user)
    .to_request();

  let resp = test::call_service(&mut app, req).await;

  // Log response status and body for debugging
  let status = resp.status();
  let body = test::read_body(resp).await;

  info!("Create User with Invalid Email Response Status: {:?}", status);
  info!("Create User with Invalid Email Response Body: {:?}", body);

  // Then: The response status should be 204 No Content
  assert_eq!(status, actix_web::http::StatusCode::BAD_REQUEST);
}

#[actix_rt::test]
#[serial]
async fn test_create_user_with_weak_password() {
  init_logger();
  clear_users_table();

  // Given: A new user with a weak password
  let pool = establish_connection();
  let new_user = NewUser {
    username: "testuser".to_string(),
    email: "testuser@example.com".to_string(),
    password: "weakpass".to_string(),
  };

  // When: The user is created
  let mut app = test::init_service(App::new()
    .app_data(web::Data::new(pool.clone()))
    .service(user_routes())).await;

  let req = test::TestRequest::post()
    .uri("/users")
    .set_json(&new_user)
    .to_request();

  let resp = test::call_service(&mut app, req).await;

  // Log response status and body for debugging
  let status = resp.status();
  let body = test::read_body(resp).await;

  info!("Create User with Weak Password Response Status: {:?}", status);
  info!("Create User with Weak Password Response Body: {:?}", body);

  // Then: The response status should be 204 No Content
  assert_eq!(status, actix_web::http::StatusCode::BAD_REQUEST);
}

#[actix_rt::test]
#[serial]
async fn test_create_user_with_short_username() {
  init_logger();
  clear_users_table();

  // Given: A new user with a short username
  let pool = establish_connection();
  let new_user = NewUser {
    username: "tu".to_string(),
    email: "testuser@example.com".to_string(),
    password: "Password123!".to_string(),
  };

  // When: The user is created
  let mut app = test::init_service(App::new()
    .app_data(web::Data::new(pool.clone()))
    .service(user_routes())).await;

  let req = test::TestRequest::post()
    .uri("/users")
    .set_json(&new_user)
    .to_request();

  let resp = test::call_service(&mut app, req).await;

  // Log response status and body for debugging
  let status = resp.status();
  let body = test::read_body(resp).await;

  info!("Create User with Short Username Response Status: {:?}", status);
  info!("Create User with Short Username Response Body: {:?}", body);

  // Then: The response status should be 204 No Content
  assert_eq!(status, actix_web::http::StatusCode::BAD_REQUEST);
}

#[actix_rt::test]
#[serial]
async fn test_list_users() {
  init_logger();
  clear_users_table();

  // Given: Users exist in the database
  let pool = establish_connection();
  let mut app = test::init_service(App::new()
    .app_data(web::Data::new(pool.clone()))
    .service(user_routes())).await;

  let req = test::TestRequest::get()
    .uri("/users")
    .to_request();

  let resp = test::call_service(&mut app, req).await;

  // Log response status and body for debugging
  let status = resp.status();
  let body = test::read_body(resp).await;

  info!("List Users Response Status: {:?}", status);
  info!("List Users Response Body: {:?}", body);

  // Then: The response status should be 204 No Content
  assert_eq!(status, actix_web::http::StatusCode::OK);
}

#[actix_rt::test]
#[serial]
async fn test_get_user() {
  init_logger();
  clear_users_table();

  // Given: A user exists in the database
  let pool = establish_connection();
  let conn = &mut pool.get().expect("Failed to get DB connection from pool");

  let new_user = NewUser {
    username: "testuser".to_string(),
    email: "testuser@example.com".to_string(),
    password: "Password123!".to_string(),
  };
  diesel::insert_into(crate::models::schema::users::table)
    .values(&new_user)
    .execute(conn)
    .expect("Error inserting user");

  let user_id: Uuid = crate::models::schema::users::table
    .select(crate::models::schema::users::dsl::id)
    .first(conn)
    .expect("Error fetching user ID");

  // When: The user is requested by ID
  let mut app = test::init_service(App::new()
    .app_data(web::Data::new(pool.clone()))
    .service(user_routes())).await;

  let req = test::TestRequest::get()
    .uri(&format!("/users/{}", user_id))
    .to_request();

  let resp = test::call_service(&mut app, req).await;

  // Log response status and body for debugging
  let status = resp.status();
  let body = test::read_body(resp).await;

  info!("Get User Response Status: {:?}", status);
  info!("Get User Response Body: {:?}", body);

  // Then: The response status should be 200 OK
  assert_eq!(status, actix_web::http::StatusCode::OK);
}

#[actix_rt::test]
#[serial]
async fn test_update_user() {
  init_logger();
  clear_users_table();

  // Given: A user exists in the database
  let pool = establish_connection();
  let conn = &mut pool.get().expect("Failed to get DB connection from pool");

  let new_user = NewUser {
    username: "testuser".to_string(),
    email: "testuser@example.com".to_string(),
    password: "Password123!".to_string(),
  };
  diesel::insert_into(crate::models::schema::users::table)
    .values(&new_user)
    .execute(conn)
    .expect("Error inserting user");

  let user_id: Uuid = crate::models::schema::users::table
    .select(crate::models::schema::users::dsl::id)
    .first(conn)
    .expect("Error fetching user ID");

  let updated_user = UpdateUser {
    username: Some("updateduser".to_string()),
    email: None,
    password: Some("NewPassword123!".to_string()),
  };

  // When: The user is updated by ID
  let mut app = test::init_service(App::new()
    .app_data(web::Data::new(pool.clone()))
    .service(user_routes())).await;

  let req = test::TestRequest::put()
    .uri(&format!("/users/{}", user_id))
    .set_json(&updated_user)
    .to_request();

  let resp = test::call_service(&mut app, req).await;

  // Log response status and body for debugging
  let status = resp.status();
  let body = test::read_body(resp).await;

  info!("Update User Response Status: {:?}", status);
  info!("Update User Response Body: {:?}", body);

  // Then: The response status should be 200 OK
  assert_eq!(status, actix_web::http::StatusCode::OK);
}

#[actix_rt::test]
#[serial]
async fn test_delete_user() {
  init_logger();
  clear_users_table();

  // Given: A user exists in the database
  let pool = establish_connection();
  let conn = &mut pool.get().expect("Failed to get DB connection from pool");

  let new_user = NewUser {
    username: "testuser".to_string(),
    email: "testuser@example.com".to_string(),
    password: "Password123!".to_string(),
  };
  diesel::insert_into(crate::models::schema::users::table)
    .values(&new_user)
    .execute(conn)
    .expect("Error inserting user");

  let user_id: Uuid = crate::models::schema::users::table
    .select(crate::models::schema::users::dsl::id)
    .first(conn)
    .expect("Error fetching user ID");

  // When: The user is deleted by ID
  let mut app = test::init_service(App::new()
    .app_data(web::Data::new(pool.clone()))
    .service(user_routes())).await;

  let req = test::TestRequest::delete()
    .uri(&format!("/users/{}", user_id))
    .to_request();

  let resp = test::call_service(&mut app, req).await;

  // Log response status and body for debugging
  let status = resp.status();
  let body = test::read_body(resp).await;

  info!("Delete User Response Status: {:?}", status);
  info!("Delete User Response Body: {:?}", body);

  // Then: The response status should be 204 No Content
  assert_eq!(status, actix_web::http::StatusCode::NO_CONTENT);
}