use rocket::http::{CookieJar, Status};
use rocket::serde::json::Json;
use rocket::State;
use sqlx::types::JsonValue::Null;
use crate::models::user::{LoginUser};
use crate::repositories::UserRepository;
use crate::services::security::tokens::JwtService;
use crate::services::security::user_auth::password_service::PasswordService;
use crate::services::response_service::{ApiResponse, ResponseService};

#[rocket::post("/login", data = "<body>")]
pub async fn login(
    repo: &State<UserRepository>,
    body: Json<LoginUser>,
    jwt_svc: &State<JwtService>,
    jar: &CookieJar<'_>
) -> (Status, Json<ApiResponse>) {
    let user = match repo.find_by_email(&body.email).await {
        Ok(Some(user)) => user,
        Ok(None) => return ResponseService::unauthorized("Invalid email or password"),
        Err(_) => return ResponseService::error("database error", None, 500)
    };

    if !PasswordService::verify(&body.password, &user.password) {
        return ResponseService::unauthorized("Invalid email or password");
    }

    let cookie = jwt_svc.create(user.id);
    jar.add_private(cookie);

    ResponseService::success(Null, 200)
}