use sqlx::PgPool;
use uuid::Uuid;
use crate::models::postgres::message::CreateMessage;
use crate::repositories::Repositories;

pub struct PostgresService {
    repos: Repositories,
}

impl PostgresService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repos: Repositories::init(pool),
        }
    }

    pub async fn save_message(&self, data: &CreateMessage) -> Result<crate::models::postgres::message::Message, sqlx::Error> {
        self.repos.message.create(data).await
    }

    pub async fn is_member(&self, channel_id: Uuid, user_id: Uuid) -> bool {
        self.repos.participant
            .is_member(channel_id, user_id)
            .await
            .unwrap_or(false)
    }

    pub async fn update_last_read(&self, channel_id: Uuid, user_id: Uuid) {
        let _ = self.repos.participant.update_last_read(channel_id, user_id).await;
    }

    pub async fn channel_participants(&self, channel_id: Uuid) -> Vec<crate::models::postgres::participant::Participant> {
        self.repos.participant
            .find_by_channel(channel_id)
            .await
            .unwrap_or_default()
    }
}