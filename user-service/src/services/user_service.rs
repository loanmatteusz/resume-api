use crate::models::user::{User, NewUser};
use crate::config::database::DbPool;
use crate::repositories::user_repository::UserRepository;
use crate::utils::hash_password::hash_password;

pub struct UserService;

impl UserService {
    pub async fn create_user(pool: &DbPool, new_user: NewUser) -> Result<User, String> {
        let encrypted_password = hash_password(&new_user.password)
            .expect("Error to encrypt password");

        let new_user = NewUser {
            password: encrypted_password,
            ..new_user
        };

        UserRepository::create_user(pool, new_user).await.map_err(|e| e.to_string())
    }
}
