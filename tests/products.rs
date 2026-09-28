mod common;

use common::{delete_product_by_slug, seed_product, test_server, test_server_with_state};

#[tokio::test]
async fn list_products_returns_seeded_product() {
    let (server, state) = test_server_with_state().await;
    let product = seed_product(&state).await;

    let response = server.get("/products").await;
    response.assert_status_ok();

    let body: serde_json::Value = response.json();
    let products = body.as_array().expect("expected /products to return a JSON array");
    assert!(
        products
            .iter()
            .any(|p| p.get("slug").and_then(|s| s.as_str()) == Some(product.slug.as_str())),
        "expected seeded product {} in response: {body}",
        product.slug
    );

    delete_product_by_slug(&state, &product.slug).await;
}

#[tokio::test]
async fn show_product_returns_seeded_product() {
    let (server, state) = test_server_with_state().await;
    let product = seed_product(&state).await;

    let response = server.get(&format!("/products/{}", product.id)).await;
    response.assert_status_ok();

    let body: serde_json::Value = response.json();
    assert_eq!(
        body.get("slug").and_then(|s| s.as_str()),
        Some(product.slug.as_str())
    );
    assert_eq!(
        body.get("name").and_then(|s| s.as_str()),
        Some(product.name.as_str())
    );
    assert_eq!(body.get("is_active").and_then(|v| v.as_bool()), Some(true));

    delete_product_by_slug(&state, &product.slug).await;
}

#[tokio::test]
async fn show_product_returns_404_for_unknown_id() {
    let server = test_server().await;

    let response = server.get(&format!("/products/{}", u64::MAX)).await;
    response.assert_status_not_found();

    let body: serde_json::Value = response.json();
    assert_eq!(
        body.get("message").and_then(|s| s.as_str()),
        Some("Model not found.")
    );
}
