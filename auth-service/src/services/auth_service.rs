use std::env;
use jsonwebtoken::{encode, Header, EncodingKey};
use crate::models::auth::{Claims, LoginRequest, LoginResponse};
use crate::config::database::DbPool;
use crate::repositories::user_repository::UserRepository;
use crate::utils::verify_password::verify_password;

pub struct AuthService;

impl AuthService {
    pub async fn authenticate(
        pool: &DbPool,
        login_request: &LoginRequest,
    ) -> Result<LoginResponse, &'static str> {
        let user = UserRepository::find_by_email(pool, &login_request.email).await;

        match user {
            Ok(user) => {
                if verify_password(&user.password, &login_request.password) {
                    let token = Self::generate_token(&user.id.to_string()).await;
                    match token {
                        Ok(token) => Ok(LoginResponse { token }),
                        Err(_e) => Err("Token not created!"),
                    }
                } else {
                    Err("Invalid Credentials")
                }
            },
            Err(diesel::result::Error::NotFound) => Err("User not found"),
            Err(_e) => Err("Error occurred"),
        }
    }

    async fn generate_token(user_id: &str) -> Result<String, String> {
        let expiration = 3600 * 60;
        let claims = Claims {
            sub: user_id.to_owned(),
            exp: expiration as usize,
        };

        let secret = env::var("SECRET").map_err(|_| "SECRET not set".to_string())?;
        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
            .map_err(|e| e.to_string())?;

        Ok(token.to_string())
    }
}
