use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::json::Value::Null;
use crate::security::user_auth_guard::AuthenticatedUser;
use crate::services::response_service::{ApiResponse, ResponseService};

#[rocket::post("/fetch-auth")]
pub fn fetch_auth(_user: AuthenticatedUser) -> (Status, Json<ApiResponse>) {
    ResponseService::success(Null, 200)
}