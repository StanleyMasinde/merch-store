use axum::{
    Router,
    routing::{get, post},
};

use toasty::Db;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::server::{handlers, types::app_config::AppConfig};

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub config: AppConfig,
}

pub async fn start_server() {
    let AppConfig {
        daraja: _,
        database,
    } = AppConfig::load();
    let db = toasty::Db::builder()
        .models(toasty::models!(crate::*))
        .connect(&database.connection)
        .await
        .unwrap();

    let config = AppConfig::load();

    let state = AppState { db, config };
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
        )
        .with_state(state)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::info!("App running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
