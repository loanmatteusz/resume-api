use diesel::prelude::*;
use crate::models::user::{User};
use crate::models::schema::users::dsl::*;
use crate::config::database::DbPool;

pub struct UserRepository;

impl UserRepository {
    pub async fn find_by_email(pool: &DbPool, user_email: &str) -> Result<User, diesel::result::Error> {
        let conn = &mut pool.get().expect("Failed to get DB connection from pool");
        users.filter(email.eq(user_email)).first::<User>(conn)
    }
}
