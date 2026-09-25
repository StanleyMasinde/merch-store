use axum::{
    Json,
    http::{Method, StatusCode, Uri},
    response::IntoResponse,
};
use serde_json::json;

pub async fn index() -> &'static str {
    "Hello, World"
}

pub async fn global_fallback_handler(method: Method, uri: Uri) -> impl IntoResponse {
    let error_message = format!("Route '{}' with method '{}' not found", uri, method);

    let body = Json(json!({
        "message": error_message
    }));

    (StatusCode::NOT_FOUND, body)
}
