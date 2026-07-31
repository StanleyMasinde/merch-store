use axum::{
    Router,
    routing::{get, post},
};

use crate::server::handlers;

pub async fn start_server() {
    let app = Router::new()
        .route("/", get(handlers::index))
        .route("/pay", post(handlers::pay))
        .route(
            "/products",
            get(handlers::list_products).post(handlers::create_product),
        )
        .route(
            "/products/{id}",
            get(handlers::create_product)
                .patch(handlers::update_product)
                .delete(handlers::delete_product),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("App running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
