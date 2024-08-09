use diesel::prelude::*;
use crate::models::user::{User, NewUser};
use crate::config::database::DbPool;
use crate::models::schema::users::dsl::*;


pub struct UserRepository;

impl UserRepository {
    pub async fn create_user(pool: &DbPool, new_user: NewUser) -> Result<User, diesel::result::Error> {
        let conn = &mut pool.get().expect("Failed to get DB connection from pool");
        diesel::insert_into(users)
            .values(&new_user)
            .get_result(conn)
    }
}
