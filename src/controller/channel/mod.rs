use crate::controller::channel::channel::get_messages;
use crate::controller::channel::dm::{get_dms, get_or_create_dm, get_dm_info};
use crate::controller::channel::friend::{delete_friend_pending_request, friend_accept, friend_block, friend_remove, friend_request, friend_unblock, friends, get_friend_pending_requests, get_friend_requests};

pub mod friend;
pub mod dm;
mod channel;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        // friends
        friend_request,
        friend_accept,
        friend_remove,
        friend_block,
        friend_unblock,
        friends,
        get_friend_requests,
        get_friend_pending_requests,
        delete_friend_pending_request,
        // dms
        get_or_create_dm,
        get_dms,
        get_dm_info,
        get_messages,
    ]
}