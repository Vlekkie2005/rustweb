use sqlx::PgPool;
use crate::repositories::postgres::channel_repository::ChannelRepository;
use crate::repositories::postgres::message_repository::MessageRepository;
use crate::repositories::postgres::participant_repository::ParticipantRepository;
use crate::repositories::postgres::user_repository::UserRepository;
use crate::repositories::postgres::friend_repository::FriendRepository;

mod postgres;

pub struct Repositories {
    pub user: UserRepository,
    pub channel: ChannelRepository,
    pub message: MessageRepository,
    pub participant: ParticipantRepository,
    pub friend: FriendRepository,
}

impl Repositories {
    pub fn init(pool: PgPool) -> Self {
        Self {
            user: UserRepository::new(pool.clone()),
            channel: ChannelRepository::new(pool.clone()),
            message: MessageRepository::new(pool.clone()),
            participant: ParticipantRepository::new(pool.clone()),
            friend: FriendRepository::new(pool),
        }
    }
}