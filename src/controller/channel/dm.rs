use rocket::http::Status;
use rocket::serde::json::{json, Json};
use rocket::State;
use crate::repositories::Repositories;
use crate::security::user_auth_guard::AuthenticatedUser;
use crate::security::user_id_validator_quard::UserId;
use crate::services::response_service::{ApiResponse, ResponseService};

#[rocket::post("/dms/<target_id>")]
pub async fn get_or_create_dm(
    user: AuthenticatedUser,
    target_id: UserId,
    repos: &State<Repositories>,
) -> (Status, Json<ApiResponse>) {
    if *user == *target_id {
        return ResponseService::error("You cannot DM yourself", None, 400);
    }

    if repos.user.find_by_id(*target_id).await.ok().flatten().is_none() {
        return ResponseService::error("User not found", None, 404);
    }

    let channel = match repos.channel.find_dm_by_user(*user, *target_id).await {
        Ok(Some(existing)) => existing,
        Ok(None) => {
            match repos.channel.create_dm(*user, *target_id).await {
                Ok(channel) => channel,
                Err(_) => return ResponseService::error("Database error", None, 500),
            }
        }
        Err(_) => return ResponseService::error("Database error", None, 500),
    };

    ResponseService::success(json!(channel), 200)
}

#[rocket::get("/dms/<channel_id>")]
pub async fn get_dm_info(
    user: AuthenticatedUser,
    channel_id: UserId,
    repos: &State<Repositories>,
) -> (Status, Json<ApiResponse>) {
    match repos.participant.is_member(*channel_id, *user).await {
        Ok(true) => {}
        Ok(false) => return ResponseService::error("User is not a member", None, 401),
        Err(_) => return ResponseService::error("Database error", None, 500),
    }

    let dm = match repos.channel.find_dm_by_id(*user, *channel_id).await {
        Ok(Some(existing)) => existing,
        Ok(None) => return ResponseService::error("Unable to find channel", None, 404),
        Err(_) => return ResponseService::error("Database error", None, 500),
    };

    ResponseService::success(json!(dm), 200)
}

#[rocket::get("/dms")]
pub async fn get_dms(
    user: AuthenticatedUser,
    repos: &State<Repositories>,
) -> (Status, Json<ApiResponse>) {
    let channels = match repos.channel.find_dms(*user).await {
        Ok(channels) => channels,
        Err(_) => return ResponseService::error("Database error", None, 500),
    };

    ResponseService::success(json!(channels), 200)
}