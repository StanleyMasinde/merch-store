//! Shared helpers for integration tests.
//!
//! Tests need a migrated MySQL database. By default they use the
//! `database.test_connection` from `config.toml`; set `TEST_DATABASE_URL` to
//! override it (same URL format,
//! e.g. `mysql://user:password@localhost:3306/merch_store_test`), e.g. in CI.
//! Whichever database is used must already have migrations applied (see
//! "Database Migrations" in the README).
//!
//! Each test seeds rows with unique slugs and deletes them when done, so tests can
//! run in parallel against a shared database without interfering with each other.

#![allow(dead_code)]

use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

use axum_test::TestServer;
use merch_store::server::{
    app::{AppState, app},
    models::Product,
    types::app_config::AppConfig,
};

static SEED_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Database URL for tests: `TEST_DATABASE_URL` wins, otherwise the
/// `test_connection` from `config.toml`.
pub fn test_db_url() -> String {
    if let Ok(url) = std::env::var("TEST_DATABASE_URL") {
        return url;
    }

    // Read the test field directly: this helper only ever runs in tests,
    // so the test database is always the right choice here.
    AppConfig::with_path("config.toml".into())
        .database
        .test_connection
}

/// Application state wired to the test database.
///
/// Uses `AppConfig::with_path` instead of `AppConfig::load` because `load` reads
/// `std::env::args`, which holds the test harness arguments under `cargo test`.
pub async fn test_state() -> AppState {
    let connection = test_db_url();
    let mut db = toasty::Db::builder()
        .models(toasty::models!(merch_store::*))
        .connect(&connection)
        .await
        .expect("failed to connect to test database (set TEST_DATABASE_URL or check config.toml)");

    // Fail fast with a clear message when the database has no schema yet.
    Product::all()
        .exec(&mut db)
        .await
        .expect("test database is missing tables; apply migrations to it first");

    let config = AppConfig::with_path("config.toml".into());

    AppState { db, config }
}

/// Test server backed by the test database.
pub async fn test_server() -> TestServer {
    TestServer::new(app(test_state().await))
}

/// Test server plus a clone of its state, for tests that seed rows directly.
pub async fn test_server_with_state() -> (TestServer, AppState) {
    let state = test_state().await;
    let server = TestServer::new(app(state.clone()));
    (server, state)
}

/// A slug that is unique across tests and processes, so parallel tests never
/// collide on the products slug unique index.
pub fn unique_slug(prefix: &str) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let n = SEED_COUNTER.fetch_add(1, Ordering::SeqCst);

    format!("{prefix}-{}-{nanos}-{n}", std::process::id())
}

/// Insert a product directly into the test database.
pub async fn seed_product(state: &AppState) -> Product {
    let slug = unique_slug("test-product");
    let mut db = state.db.clone();

    toasty::create!(Product {
        name: format!("Test product {slug}"),
        slug: slug,
        description: "Seeded by integration tests".to_string(),
        featured_image_url: "https://example.test/featured.png".to_string(),
        is_active: true
    })
    .exec(&mut db)
    .await
    .expect("failed to seed test product")
}

/// Delete a seeded product by slug. Best-effort cleanup so repeated runs do not
/// accumulate test rows; slugs stay unique regardless.
pub async fn delete_product_by_slug(state: &AppState, slug: &str) {
    let mut db = state.db.clone();

    Product::filter_by_slug(slug)
        .delete()
        .exec(&mut db)
        .await
        .expect("failed to delete seeded product");
}
