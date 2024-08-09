use std::io;

use actix_web::{App, HttpServer, middleware, web};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use env_logger::Env;

use auth_service::config::database::establish_connection;
use auth_service::middleware::rate_limiter::configure_rate_limiter;
use auth_service::routes::auth_routes;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[tokio::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let pool = establish_connection();

    // Executa migrações
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    conn.run_pending_migrations(MIGRATIONS).expect("Failed to run migrations");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .wrap(configure_rate_limiter())
            .service(auth_routes())
    })
        .bind("127.0.0.1:8081")?
        .run()
        .await
}
