use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub enum CustomErrorEnum {
    BadRequest,
    TaskNotFound,
    InternalServerError,
}

impl CustomErrorEnum {
    pub fn text(self, text: String) -> CustomError {
        CustomError { num: self, text }
    }
}

pub struct CustomError {
    pub num: CustomErrorEnum,
    pub text: String,
}

impl IntoResponse for CustomError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self.num {
            CustomErrorEnum::InternalServerError => {
                (StatusCode::BAD_REQUEST, "Internal Server Error")
            }
            CustomErrorEnum::BadRequest => (StatusCode::BAD_REQUEST, "Bad Request"),
            CustomErrorEnum::TaskNotFound => (StatusCode::NOT_FOUND, "Task Not Found"),
        };

        (
            status,
            Json(json!({"error": error_message, "message": self.text })),
        )
            .into_response()
    }
}
