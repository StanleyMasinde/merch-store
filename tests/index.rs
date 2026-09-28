mod common;

use common::test_server;

#[tokio::test]
async fn index_returns_hello_world() {
    let server = test_server().await;

    let response = server.get("/").await;
    response.assert_status_ok();
    assert_eq!(response.text(), "Hello, World");
}

#[tokio::test]
async fn unknown_route_returns_404() {
    let server = test_server().await;

    let response = server.get("/does-not-exist").await;
    response.assert_status_not_found();

    let body: serde_json::Value = response.json();
    let message = body.get("message").and_then(|m| m.as_str()).unwrap_or("");
    assert!(
        message.contains("not found"),
        "expected a not-found message, got: {body}"
    );
}
