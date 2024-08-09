use diesel::r2d2::ConnectionManager;
use diesel::pg::PgConnection;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    
    r2d2::Pool::builder()
      .test_on_check_out(true)
      .build(manager)
      .expect("Failed to create pool.")
}
