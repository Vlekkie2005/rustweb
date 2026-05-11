use rocket::http::Status;
use rocket::serde::json::{json, Json};
use rocket::State;
use crate::repositories::Repositories;
use crate::security::user_auth_guard::AuthenticatedUser;
use crate::security::user_id_validator_quard::UserId;
use crate::services::response_service::{ApiResponse, ResponseService};

#[rocket::get("/channels/<channel_id>/messages?<before>&<limit>")]
pub async fn get_messages(
    user: AuthenticatedUser,
    channel_id: UserId,
    before: Option<String>,
    limit: Option<i64>,
    repos: &State<Repositories>,
) -> (Status, Json<ApiResponse>) {
    match repos.participant.is_member(*channel_id, *user).await {
        Ok(true) => {}
        Ok(false) => return ResponseService::error("Forbidden", None, 403),
        Err(_) => return ResponseService::error("Database error", None, 500),
    }

    let before_dt: Option<chrono::DateTime<chrono::Utc>> = before
        .as_deref()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc));

    let limit = limit.unwrap_or(50).min(100);

    let messages = match repos.message.find_by_channel(*channel_id, before_dt, limit).await {
        Ok(msgs) => msgs,
        Err(_) => return ResponseService::error("Database error", None, 500),
    };

    ResponseService::success(json!(messages), 200)
}
