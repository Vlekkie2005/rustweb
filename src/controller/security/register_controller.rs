use rocket::http::{CookieJar, Status};
use rocket::State;
use rocket::serde::json::Json;
use validator::Validate;
use sqlx::types::JsonValue::Null;
use crate::models::user::{CreateUser};
use crate::repositories::UserRepository;
use crate::services::response_service::{ApiResponse, ResponseService};
use crate::services::security::tokens::JwtService;
use crate::services::security::user_auth::password_service::PasswordService;


#[rocket::post("/register", data = "<body>")]
pub async fn register(
    repo: &State<UserRepository>,
    jwt_svc: &State<JwtService>,
    jar: &CookieJar<'_>,
    body: Json<CreateUser>,
) -> (Status, Json<ApiResponse>) {
    if let Err(_) = body.validate() {
        return ResponseService::unprocessable("Validation failed");
    }

    let hashed_password = match PasswordService::hash(&body.password) {
        Ok(h) => h,
        Err(_) => return ResponseService::error("Failed to hash password", None, 500),
    };

    let new_user = CreateUser {
        username: body.username.clone(),
        email: body.email.clone(),
        password: hashed_password,
    };

    match repo.create(&new_user).await {
        Ok(user) => {
            let cookie = jwt_svc.create(user.id);
            jar.add_private(cookie);
            ResponseService::success(Null, 201)
        },
        Err(e) => match e {
            sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
                ResponseService::conflict("Email or username already exists")
            }
            _ => ResponseService::error("Failed to create user", None, 500),
        }
    }
}