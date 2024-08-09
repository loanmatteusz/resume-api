use actix_web::{middleware, web, App, HttpServer};
use env_logger::Env;

mod config;
use config::database::establish_connection;

use user_service::routes::user_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    
    let pool = establish_connection();
    let data = web::Data::new(pool.clone());

    println!("Server is running on port 8080!");
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(middleware::Logger::default())
            .service(user_routes())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
