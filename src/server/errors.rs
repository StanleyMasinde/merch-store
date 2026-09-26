use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("A database error occoured.")]
    DatabaseError(String),

    #[error("Model not found.")]
    ModelNotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::DatabaseError(ref err) => {
                tracing::error!("Database error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            AppError::ModelNotFound => (StatusCode::NOT_FOUND, self.to_string()),
        };

        let body = Json(json!({
            "message": error_message
        }));

        (status, body).into_response()
    }
}
