use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate};

#[derive(Debug, Deserialize, Serialize, sqlx::Type, Clone, Copy)]
#[sqlx(type_name = "channel_type", rename_all = "snake_case")]
pub enum ChannelType {
    Dm,
    Group,
    ServerChannel,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Channel {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub name: String,
    pub r#type: ChannelType,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct DmChannel {
    pub id: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub name: String,
    pub r#type: ChannelType,
    pub channel_name: String
}

#[derive(Debug, Validate, Deserialize)]
pub struct CreateChannel {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub r#type: ChannelType,
}

