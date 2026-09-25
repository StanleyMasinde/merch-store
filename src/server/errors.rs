use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("The resource {0}, could not be found.")]
    ResourceNotFound(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::ResourceNotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
        };

        let body = Json(json!({
            "message": error_message
        }));

        (status, body).into_response()
    }
}
