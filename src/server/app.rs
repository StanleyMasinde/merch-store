use axum::{Router, routing::get};

pub async fn start_server() {
    let app = Router::new().route("/", get(index_handler));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("App running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn index_handler() -> &'static str {
    "Hello, World"
}
