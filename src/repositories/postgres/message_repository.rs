use crate::models::postgres::message::{CreateMessage, Message, UserMessage};
use sqlx::PgPool;
use uuid::Uuid;

pub struct MessageRepository {
    pool: PgPool,
}

impl MessageRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, data: &CreateMessage) -> Result<Message, sqlx::Error> {
         sqlx::query_as!(
            Message,
            r#"INSERT INTO message (channel_id, user_id, content, parent_id)
               VALUES ($1, $2, $3, $4)
               RETURNING *"#,
            data.channel_id,
            data.user_id,
            data.content,
            data.parent_id,
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Message>, sqlx::Error> {
        sqlx::query_as!(Message, "SELECT * FROM message WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn find_by_channel(
        &self,
        channel_id: Uuid,
        before: Option<chrono::DateTime<chrono::Utc>>,
        limit: i64,
    ) -> Result<Vec<UserMessage>, sqlx::Error> {
        sqlx::query_as!(
        UserMessage,
            r#"SELECT
                m.id,
                m.channel_id,
                m.user_id,
                m.content,
                m.parent_id,
                m.edited_at,
                m.deleted_at,
                m.created_at,
                u.username as author_username
               FROM message m
               JOIN users u ON u.id = m.user_id
               WHERE m.channel_id = $1
               AND ($2::timestamptz IS NULL OR m.created_at < $2)
               AND m.deleted_at IS NULL
               ORDER BY m.created_at DESC
               LIMIT $3"#,
            channel_id,
            before,
            limit,
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn find_thread(&self, parent_id: Uuid) -> Result<Vec<Message>, sqlx::Error> {
        sqlx::query_as!(
            Message,
            "SELECT * FROM message
             WHERE parent_id = $1
             AND deleted_at IS NULL
             ORDER BY created_at ASC",
            parent_id,
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn edit(
        &self,
        id: Uuid,
        user_id: Uuid,
        content: String,
    ) -> Result<Option<Message>, sqlx::Error> {
        sqlx::query_as!(
            Message,
            r#"UPDATE message
               SET content = $1, edited_at = now()
               WHERE id = $2 AND user_id = $3
               RETURNING *"#,
            content,
            id,
            user_id,
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn delete(&self, id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM message WHERE id = $1 AND user_id = $2",
            id,
            user_id,
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
