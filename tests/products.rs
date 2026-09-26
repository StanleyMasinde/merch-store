use axum_test::TestServer;
use merch_store::server::{
    app::{AppState, app},
    types::app_config::AppConfig,
};

#[tokio::test]
async fn name() {
    let AppConfig {
        daraja: _,
        database,
    } = AppConfig::load();

    let config = AppConfig::load();

    let db = toasty::Db::builder()
        .models(toasty::models!(crate::*))
        .connect(&database.connection)
        .await
        .unwrap();
    let state = AppState { db, config };
    let server = TestServer::new(app(state));

    let response = server.get("/products").await;

    response.assert_status_ok();
}
