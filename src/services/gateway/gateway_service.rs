use std::collections::HashMap;
use sqlx::PgPool;
use tokio::sync::broadcast::Sender;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;
use crate::services::gateway::postgres_service::PostgresService;
use crate::services::gateway::redis_service::RedisService;

pub struct GatewayService {
    pub users: RwLock<HashMap<Uuid, Sender<String>>>,
    pub postgres: PostgresService,
    pub redis: RedisService,
}

impl GatewayService {
    pub async fn new(pool: PgPool, redis_url: &str) -> Self {
        Self {
            users: RwLock::new(HashMap::new()),
            postgres: PostgresService::new(pool),
            redis: RedisService::new(redis_url).await,
        }
    }

    pub async fn get_or_create(&self, user_id: &Uuid) -> Sender<String> {
        {
            let users = self.users.read().await;
            if let Some(tx) = users.get(user_id) {
                if tx.receiver_count() > 0 {
                    return tx.clone();
                }
            }
        }

        let mut users = self.users.write().await;
        if let Some(tx) = users.get(user_id) {
            if tx.receiver_count() > 0 {
                return tx.clone();
            }
        }

        let (tx, _) = broadcast::channel(100);
        users.insert(user_id.clone(), tx.clone());
        tx
    }

    pub async fn broadcast_message(&self, channel_id: Uuid, payload: String) {
        let participants = self.postgres.channel_participants(channel_id).await;
        let users = self.users.read().await;

        for p in participants {
            if let Some(tx) = users.get(&p.user_id) {
                let _ = tx.send(payload.clone());
            }
        }
    }

    pub async fn cleanup(&self) {
        let mut users = self.users.write().await;
        users.retain(|_, tx| tx.receiver_count() > 0);
    }
}