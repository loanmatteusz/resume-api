use actix_web::{HttpResponse, ResponseError};
use actix_web::http::StatusCode;
use std::fmt;
use serde_json::json;
use crate::errors::error::AuthError;

impl ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match *self {
            AuthError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::UserNotFound => StatusCode::NOT_FOUND,
            AuthError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            AuthError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(json!({ "error": self.to_string() }))
    }
}
