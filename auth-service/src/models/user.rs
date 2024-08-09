use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};
use crate::models::schema::users;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable, Serialize, Deserialize, Validate)]
#[diesel(table_name = users)]
pub struct NewUser {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    #[validate(custom(function = "validate_password_strength"))]
    pub password: String,
}

#[derive(AsChangeset, Serialize, Deserialize, Validate)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    #[validate(length(min = 3, max = 50))]
    pub username: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(min = 8))]
    #[validate(custom(function = "validate_password_strength"))]
    pub password: Option<String>,
}

fn validate_password_strength(password: &str) -> Result<(), validator::ValidationError> {
    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_lower = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_digit(10));
    let has_special = password.chars().any(|c| !c.is_alphanumeric());

    if has_upper && has_lower && has_digit && has_special {
        Ok(())
    } else {
        Err(ValidationError::new("password_strength"))
    }
}
