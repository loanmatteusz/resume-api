use std::env;
use actix_web::{Error, HttpRequest, HttpResponse, Result};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

async fn validate_token(req: HttpRequest) -> Result<HttpResponse, Error> {
    let auth_header = req.headers().get("Authorization");

    let token = match auth_header
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
    {
        Some(token) => token,
        None => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let secret = env::var("SECRET").map_err(|_| "SECRET not set".to_string())
        .unwrap().to_string();
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    let validation = Validation::new(Algorithm::HS256);

    match decode::<Claims>(token, &decoding_key, &validation) {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Ok(HttpResponse::Unauthorized().finish()),
    }
}
