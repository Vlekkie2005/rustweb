use crate::models::postgres::friend::{Friend, FriendStatus, Friends, InOutFriend};
use crate::services::user_helper::user_order;
use sqlx::PgPool;
use uuid::Uuid;

pub struct FriendRepository {
    pool: PgPool,
}

impl FriendRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        requester_id: Uuid,
        friend_id: Uuid,
    ) -> Result<Friend, sqlx::Error> {
        let (user_one_id, user_two_id) = user_order(requester_id, friend_id);

        sqlx::query_as!(
            Friend,
            r#"INSERT INTO friend (user_one_id, user_two_id, requested_by, status)
               VALUES ($1, $2, $3, 'pending'::friend_status)
               RETURNING
                   user_one_id,
                   user_two_id,
                   requested_by,
                   status AS "status!: FriendStatus",
                   created_at,
                   updated_at"#,
            user_one_id,
            user_two_id,
            requester_id,
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn accept(
        &self,
        requester_id: Uuid,
        target_id: Uuid,
    ) -> Result<Option<Friend>, sqlx::Error> {
        let (user_one_id, user_two_id) = user_order(requester_id, target_id);

        sqlx::query_as!(
            Friend,
            r#"UPDATE friend
               SET status = 'accepted'::friend_status, updated_at = NOW()
               WHERE user_one_id = $1
                 AND user_two_id = $2
                 AND status = 'pending'::friend_status
               RETURNING
                   user_one_id,
                   user_two_id,
                   requested_by,
                   status AS "status!: FriendStatus",
                   created_at,
                   updated_at"#,
            user_one_id,
            user_two_id,
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn block(
        &self,
        blocker_id: Uuid,
        blocked_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        sqlx::query!(
            "INSERT INTO user_block (blocker_id, blocked_id) VALUES ($1, $2)",
            blocker_id,
            blocked_id
        )
        .execute(&mut *tx)
        .await?;

        let (user_one_id, user_two_id) = user_order(blocker_id, blocked_id);
        sqlx::query!(
            "DELETE FROM friend WHERE user_one_id = $1 AND user_two_id = $2",
            user_one_id,
            user_two_id,
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }

    pub async fn unblock(
        &self,
        blocker_id: Uuid,
        blocked_id: Uuid
    ) -> Result<(), sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM user_block WHERE blocker_id = $1 AND blocked_id = $2",
            blocker_id,
            blocked_id
        )
        .execute(&self.pool)
        .await?;
        
        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }

    pub async fn remove(&self, user_id: Uuid, target_id: Uuid) -> Result<bool, sqlx::Error> {
        let (user_one_id, user_two_id) = user_order(user_id, target_id);

        let result = sqlx::query!(
            "DELETE FROM friend WHERE user_one_id = $1 AND user_two_id = $2",
            user_one_id,
            user_two_id,
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn find(
        &self,
        user_id: Uuid,
        target_id: Uuid,
    ) -> Result<Option<Friend>, sqlx::Error> {
        let (user_one_id, user_two_id) = user_order(user_id, target_id);

        sqlx::query_as!(
            Friend,
            r#"SELECT
                   user_one_id,
                   user_two_id,
                   requested_by,
                   status AS "status!: FriendStatus",
                   created_at,
                   updated_at
               FROM friend
               WHERE user_one_id = $1 AND user_two_id = $2"#,
            user_one_id,
            user_two_id,
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn find_friends(&self, user_id: Uuid) -> Result<Vec<Friends>, sqlx::Error> {
        sqlx::query_as!(
            Friends,
            r#"SELECT
                f.user_one_id,
                f.user_two_id,
                f.requested_by,
                f.status AS "status!: FriendStatus",
                f.created_at,
                f.updated_at,
                u.username AS friend_name
                FROM friend f
                JOIN users u ON u.id = CASE
                    WHEN f.user_one_id = $1 THEN f.user_two_id
                    ELSE f.user_one_id
                END
                WHERE (f.user_one_id = $1 OR f.user_two_id = $1)
                    AND f.status = 'accepted'"#,
            user_id,
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn find_incoming(&self, user_id: Uuid) -> Result<Vec<InOutFriend>, sqlx::Error> {
        sqlx::query_as!(
            InOutFriend,
            r#"SELECT
                f.user_one_id, f.user_two_id, f.requested_by,
                f.status AS "status!: FriendStatus",
                f.created_at, f.updated_at,
                u.username AS friend_name
                FROM friend f
                JOIN users u ON u.id = CASE
                    WHEN f.user_one_id = $1 THEN f.user_two_id
                    ELSE f.user_one_id
                END
                WHERE (f.user_one_id = $1 OR f.user_two_id = $1)
                AND f.requested_by != $1
                AND f.status = 'pending'::friend_status"#,
            user_id,
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn find_outgoing(&self, user_id: Uuid) -> Result<Vec<InOutFriend>, sqlx::Error> {
        sqlx::query_as!(
            InOutFriend,
            r#"SELECT
                f.user_one_id, f.user_two_id, f.requested_by,
                f.status AS "status!: FriendStatus",
                f.created_at, f.updated_at,
                u.username AS friend_name
                FROM friend f
                JOIN users u ON u.id = CASE
                    WHEN f.user_one_id = $1 THEN f.user_two_id
                    ELSE f.user_one_id
                END
                WHERE f.requested_by = $1
                AND f.status = 'pending'::friend_status"#,
            user_id,
        )
        .fetch_all(&self.pool)
        .await
    }
}