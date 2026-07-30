use axum::{
    Json, Router,
    routing::{get, post},
};
use daraja_sdk::mpesa::{self, MpesaExpress};
use serde::Deserialize;

pub async fn start_server() {
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/pay", post(pay_handler));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("App running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn index_handler() -> &'static str {
    "Hello, World"
}

#[derive(Deserialize)]
struct CreatePaymentRequest {
    phone_number: u64,
    amount: u32,
}

#[derive(Deserialize)]
pub struct AppConfig {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub passkey: String,
    pub callback_url: String,
    pub business_shortcode: u32,
}

impl AppConfig {
    pub fn load() -> Self {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/config.toml");
        let contents = std::fs::read_to_string(path)
            .expect("copy config.toml.example to config.toml and add credentials");
        toml::from_str(&contents).expect("config.toml is malformed")
    }
}

async fn pay_handler(Json(payload): Json<CreatePaymentRequest>) -> String {
    let config = AppConfig::load();
    let client = mpesa::Client::with_credentials(&config.consumer_key, &config.consumer_secret);
    let token = client.generate_access_token().await.unwrap();
    println!("{}", token.access_token);

    let response = MpesaExpress::new()
        .access_token(&token.access_token)
        .passkey(&config.passkey)
        .business_short_code(config.business_shortcode)
        .party_a(payload.phone_number)
        .party_b(config.business_shortcode)
        .phone_number(payload.phone_number)
        .amount(payload.amount)
        .account_reference("Order123")
        .tx_description("Payment")
        .call_back_url("https://your-domain.com/callback")
        .send_prompt()
        .await
        .unwrap();

    println!("{}", response.customer_message);
    response.customer_message
}
