use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct CreateUser {
    #[validate(length(min = 3, max = 30))]
    pub username: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}