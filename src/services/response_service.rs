use rocket::serde::json::{serde_json, Json, Value};
use rocket::http::Status;
use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse {
    pub status: String,
    pub status_code: u16,
    pub timestamp: String,
    pub data: Option<Value>,
    pub message: Option<String>,
    pub details: Option<Value>,
}

pub struct ResponseService;

impl ResponseService {
    fn response(status: &str, status_code: u16, data: Option<Value>, message: Option<String>, details: Option<Value>) -> (Status, Json<ApiResponse>) {
        let rocket_status = Status::from_code(status_code).unwrap_or(Status::InternalServerError);

        let body = ApiResponse {
            status: status.to_string(),
            status_code,
            timestamp: chrono::Utc::now().to_string(),
            data,
            message,
            details,
        };

        (rocket_status, Json(body))
    }


    pub fn success<T: Serialize>(data: T, code: u16) -> (Status, Json<ApiResponse>) {
        let value = serde_json::to_value(data).ok();
        Self::response("success", code, value, None, None)
    }

    pub fn error(message: &str, details: Option<Value>, code: u16) -> (Status, Json<ApiResponse>) {
        Self::response("error", code, None, Some(message.to_string()), details)
    }

    // pub fn bad_request(message: &str) -> (Status, Json<ApiResponse>) {
    //     Self::error(message, None, 400)
    // }

    pub fn unauthorized(message: &str) -> (Status, Json<ApiResponse>) {
        Self::error(message, None, 401)
    }

    // pub fn not_found(message: &str) -> (Status, Json<ApiResponse>) {
    //     Self::error(message, None, 404)
    // }

    pub fn conflict(message: &str) -> (Status, Json<ApiResponse>) {
        Self::error(message, None, 409)
    }

    pub fn unprocessable(message: &str) -> (Status, Json<ApiResponse>) {
        Self::error(message, None, 422)
    }

    // pub fn too_many_requests(message: &str) -> (Status, Json<ApiResponse>) {
    //     Self::error(message, None, 429)
    // }
}