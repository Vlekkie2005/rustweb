use crate::models::postgres::participant::{CreateParticipant, Participant};
use sqlx::PgPool;
use uuid::Uuid;

pub struct ParticipantRepository {
    pool: PgPool,
}

impl ParticipantRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn join(&self, data: &CreateParticipant) -> Result<Participant, sqlx::Error> {
        sqlx::query_as!(
            Participant,
            r#"INSERT INTO participant (channel_id, user_id, is_admin)
               VALUES ($1, $2, $3)
               RETURNING *"#,
            data.channel_id,
            data.user_id,
            data.is_admin,
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn find_by_channel(&self, channel_id: Uuid) -> Result<Vec<Participant>, sqlx::Error> {
        sqlx::query_as!(
            Participant,
            "SELECT * FROM participant WHERE channel_id = $1",
            channel_id,
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<Participant>, sqlx::Error> {
        sqlx::query_as!(
            Participant,
            "SELECT * FROM participant WHERE user_id = $1",
            user_id,
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn is_member(&self, channel_id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "SELECT 1 as exists FROM participant
             WHERE channel_id = $1 AND user_id = $2",
            channel_id,
            user_id,
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.is_some())
    }

    pub async fn update_last_read(
        &self,
        channel_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE participant SET last_read_at = now()
             WHERE channel_id = $1 AND user_id = $2",
            channel_id,
            user_id,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn leave(&self, channel_id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM participant WHERE channel_id = $1 AND user_id = $2",
            channel_id,
            user_id,
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
