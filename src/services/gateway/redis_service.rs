use deadpool_redis::{Config, Pool, Runtime, redis::cmd};
use uuid::Uuid;

pub struct RedisService {
    pool: Pool,
}
impl RedisService {
    pub async fn new(redis_url: &str) -> Self {
        let cfg = Config::from_url(redis_url);
        let pool = cfg.create_pool(Some(Runtime::Tokio1)).expect("Failed to create pool");
        Self { pool }
    }

    pub async fn publish(&self, channel: Uuid, payload: String) {
        if let Ok(mut conn) = self.pool.get().await {
            let _ = cmd("PUBLISH")
                .arg(channel.to_string())
                .arg(&payload)
                .query_async::<()>(&mut conn)
                .await;
        }
    }

    // pub async fn set(&self, key: &str, value: &str, expiry_secs: Option<u64>) {
    //     if let Ok(mut conn) = self.pool.get().await {
    //         let mut c = cmd("SET");
    //         c.arg(key).arg(value);
    //         if let Some(secs) = expiry_secs {
    //             c.arg("EX").arg(secs);
    //         }
    //         let _ = c.query_async::<()>(&mut conn).await;
    //     }
    // }
    //
    // pub async fn get(&self, key: &str) -> Option<String> {
    //     let mut conn = self.pool.get().await.ok()?;
    //     cmd("GET")
    //         .arg(key)
    //         .query_async::<Option<String>>(&mut conn)
    //         .await
    //         .ok()
    //         .flatten()
    // }
}