use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub jwt_exp: u64,
    pub database_url: String,
    pub redis_url: String,
}

impl AppConfig {
    pub fn load_envs() -> AppConfig {
        dotenvy::dotenv().ok();

        envy::from_env::<AppConfig>()
            .expect("Failed to load config")
    }

    pub async fn load_pgpool(&self) -> PgPool {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(30)
            .acquire_timeout(std::time::Duration::from_secs(5))
            .connect(&self.database_url)
            .await
            .expect("Failed to connect to database");

        sqlx::migrate!().run(&pool).await.expect("Migration failed");

        pool
    }
}