use rocket::http::{CookieJar, Status};
use rocket::serde::json::{serde_json, Json};
use rocket::State;
use crate::models::postgres::user::{LoginUser};
use crate::repositories::Repositories;
use crate::services::security::tokens::JwtService;
use crate::services::security::user_auth::password_service::PasswordService;
use crate::services::response_service::{ApiResponse, ResponseService};

#[rocket::post("/login", data = "<body>")]
pub async fn login(
    repos: &State<Repositories>,
    body: Json<LoginUser>,
    jwt_svc: &State<JwtService>,
    jar: &CookieJar<'_> 
) -> (Status, Json<ApiResponse>) {
    let user = match repos.user.find_by_email(&body.email.to_lowercase()).await {
        Ok(Some(user)) => user,
        Ok(None) => return ResponseService::unauthorized("Invalid email or password"),
        Err(_) => return ResponseService::error("database error", None, 500)
    };

    if !PasswordService::verify(&body.password, &user.password) {
        return ResponseService::unauthorized("Invalid email or password");
    }

    let cookie = jwt_svc.create(user.id);
    jar.add_private(cookie);

    ResponseService::success(serde_json::json!({ "username": user.username, "id": user.id }), 200)
}