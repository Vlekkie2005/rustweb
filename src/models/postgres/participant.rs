use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Participant {
    pub channel_id: Uuid,
    pub user_id: Uuid,
    pub joined_at: Option<DateTime<Utc>>,
    pub last_read_at: Option<DateTime<Utc>>,
    pub is_admin: bool,
}

#[derive(Debug, Validate, Deserialize)]
pub struct CreateParticipant {
    pub channel_id: Uuid,
    pub user_id: Uuid,
    pub is_admin: bool,
}
