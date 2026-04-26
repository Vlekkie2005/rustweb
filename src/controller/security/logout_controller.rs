use rocket::http::{CookieJar, Status};
use rocket::serde::json::Json;
use rocket::serde::json::Value::Null;
use crate::services::response_service::{ApiResponse, ResponseService};

#[rocket::post("/logout")]
pub fn logout(jar: &CookieJar<'_>) -> (Status, Json<ApiResponse>) {
    jar.remove_private("jwt");

    ResponseService::success(Null, 200)
}