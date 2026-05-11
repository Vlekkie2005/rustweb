use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, sqlx::Type, Clone, Copy)]
#[sqlx(type_name = "channel_type", rename_all = "snake_case")]
pub enum FriendStatus {
    Pending,
    Accepted,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Friend {
    pub user_one_id: Uuid,
    pub user_two_id: Uuid,
    pub requested_by: Uuid,
    pub status: FriendStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct InOutFriend {
    pub user_one_id: Uuid,
    pub user_two_id: Uuid,
    pub requested_by: Uuid,
    pub status: FriendStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub friend_name: String
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Friends {
    pub user_one_id: Uuid,
    pub user_two_id: Uuid,
    pub requested_by: Uuid,
    pub status: FriendStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub friend_name: String
}

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateFriendStatus {
    pub status: FriendStatus,
}