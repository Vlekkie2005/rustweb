use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Message {
    pub id: Uuid,
    pub channel_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub content: String,
    pub parent_id: Option<Uuid>,
    pub edited_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct UserMessage {
    pub id: Uuid,
    pub channel_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub content: String,
    pub parent_id: Option<Uuid>,
    pub edited_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub author_username: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct CreateMessage {
    pub channel_id: Uuid,
    pub user_id: Uuid,
    #[validate(length(min = 1, max = 2000))]
    pub content: String,
    pub parent_id: Option<Uuid>,
}