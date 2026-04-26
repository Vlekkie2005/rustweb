use sqlx::PgPool;
use uuid::Uuid;
use crate::models::user::{User, CreateUser};

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as!(User, "SELECT id, created_at, username, email, password FROM users")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(User,
            "SELECT id, created_at, username, email, password FROM users WHERE id = $1",
            id
        )
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(User,
            "SELECT id, created_at, username, email, password FROM users WHERE email = $1",
            email
        )
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn create(&self, data: &CreateUser) -> Result<User, sqlx::Error> {
        sqlx::query_as!(User,
        "INSERT INTO users (username, email, password)
         VALUES ($1, $2, $3)
         RETURNING id, created_at, username, email, password",
        data.username,
        data.email,
        data.password
    )
            .fetch_one(&self.pool)
            .await
    }

    pub async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
