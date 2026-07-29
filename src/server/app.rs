use axum::{Router, routing::get};

pub async fn start_server() {
    let app = Router::new().route("/", get(index_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index_handler() -> &'static str {
    "Hello, World"
}
