use crate::models::postgres::channel::{Channel, ChannelType, CreateChannel, DmChannel};
use sqlx::PgPool;
use uuid::Uuid;

pub struct ChannelRepository {
    pool: PgPool,
}

impl ChannelRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, data: &CreateChannel) -> Result<Channel, sqlx::Error> {
        sqlx::query_as!(
            Channel,
            r#"INSERT INTO channel (name, type)
                VALUES ($1, $2)
                RETURNING
                    id,
                    name,
                    type AS "type!: ChannelType",
                    created_at"#,
            data.name,
            data.r#type as ChannelType,
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Channel>, sqlx::Error> {
        sqlx::query_as!(
            Channel,
            r#"SELECT id, name, type AS "type!: ChannelType", created_at
               FROM channel WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<Channel>, sqlx::Error> {
        sqlx::query_as!(
            Channel,
            r#"SELECT c.id, c.name, c.type AS "type!: ChannelType", c.created_at
               FROM channel c
               JOIN participant p ON p.channel_id = c.id
               WHERE p.user_id = $1"#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM channel WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn find_dm_by_user(&self, user_requested: Uuid, user_two: Uuid) -> Result<Option<DmChannel>, sqlx::Error> {
        sqlx::query_as!(
            DmChannel,
            r#"SELECT c.id, c.created_at, c.name, c.type AS "type: ChannelType", u.username AS channel_name
                FROM channel c
                JOIN participant p1 ON p1.channel_id = c.id AND p1.user_id = $1
                JOIN participant p2 ON p2.channel_id = c.id AND p2.user_id = $2
                JOIN participant p3 ON p3.channel_id = c.id AND p3.user_id != $1
                JOIN users u ON u.id = p3.user_id
                WHERE c.type = 'dm'::channel_type
                LIMIT 1"#,
            user_requested, user_two
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn find_dm_by_id(&self, user_id: Uuid, channel_id: Uuid) -> Result<Option<DmChannel>, sqlx::Error> {
        sqlx::query_as!(
        DmChannel,
            r#"SELECT c.id, c.created_at, c.name, c.type AS "type: ChannelType", u.username AS channel_name
                FROM channel c
                JOIN participant p ON p.channel_id = c.id AND p.user_id != $2
                JOIN users u ON u.id = p.user_id
                WHERE c.id = $1
                AND c.type = 'dm'::channel_type
                LIMIT 1"#,
            channel_id,
            user_id
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn find_dms(&self, user_id: Uuid) -> Result<Vec<DmChannel>, sqlx::Error> {
        sqlx::query_as!(
        DmChannel,
            r#"SELECT c.id, c.created_at, c.name, c.type AS "type: ChannelType", u.username AS channel_name
                FROM channel c
                JOIN participant p_other ON p_other.channel_id = c.id AND p_other.user_id != $1
                JOIN users u ON u.id = p_other.user_id
                JOIN participant p_self ON p_self.channel_id = c.id AND p_self.user_id = $1
                WHERE c.type = 'dm'"#,
            user_id,
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn create_dm(&self, user_requested: Uuid, user_two: Uuid) -> Result<DmChannel, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        let name = format!("{}-{}-dm", user_requested, user_two);

        let channel = sqlx::query_as!(
        Channel,
        r#"INSERT INTO channel (id, name, type)
            VALUES (gen_random_uuid(), $1, 'dm')
            RETURNING id, created_at, name, type AS "type: ChannelType""#,
            name
        )
        .fetch_one(&mut *tx)
        .await?;

        sqlx::query!(
            r#"INSERT INTO participant (channel_id, user_id, is_admin)
            VALUES ($1, $2, false), ($1, $3, false)"#,
            channel.id, user_requested, user_two
        )
        .execute(&mut *tx)
        .await?;

        let username = sqlx::query_scalar!(
            r#"SELECT username FROM users WHERE id = $1 LIMIT 1"#,
            user_two
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(DmChannel {
            id: channel.id,
            created_at: channel.created_at,
            name: channel.name,
            r#type: channel.r#type,
            channel_name: username,
        })
    }
}
