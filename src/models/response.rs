use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
}