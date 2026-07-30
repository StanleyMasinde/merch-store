use axum::{
    Json, Router,
    routing::{get, post},
};
use daraja_sdk::mpesa::{self, GenerateAccessTokenResponse, MpesaExpress};
use jiff::{SignedDuration, Timestamp};
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
    pub daraja: Daraja,
    pub database: DbConfig,
}

#[derive(Deserialize)]
pub struct Daraja {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub passkey: String,
    pub callback_url: String,
    pub business_shortcode: u32,
}

#[derive(Deserialize)]
pub struct DbConfig {
    pub connection: String,
}

impl AppConfig {
    pub fn load() -> Self {
        let path = std::env::args()
            .nth(1)
            .unwrap_or_else(|| "config.toml".to_string());
        let contents = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("couldn't read config at {path}"));
        toml::from_str(&contents).expect("config.toml is malformed")
    }
}

#[derive(toasty::Model)]
struct AppCache {
    #[key]
    #[auto]
    id: u64,

    #[column(type = varchar(255))]
    #[unique]
    key: String,

    #[column(type = varchar(255))]
    value: String,
    expires_at: jiff::Timestamp,
}

async fn generate_access_token(
    consumer_key: &str,
    consumer_secret: &str,
) -> GenerateAccessTokenResponse {
    let client = mpesa::Client::with_credentials(consumer_key, consumer_secret);

    client.generate_access_token().await.unwrap()
}

async fn pay_handler(Json(payload): Json<CreatePaymentRequest>) -> String {
    let AppConfig { daraja, database } = AppConfig::load();
    let mut db = toasty::Db::builder()
        .models(toasty::models!(crate::*))
        .connect(&database.connection)
        .await
        .unwrap();

    let get_token_req = AppCache::filter_by_key("mpesa_access_token")
        .get(&mut db)
        .await;

    let access_token = match get_token_req {
        Ok(mut db_access_token) => {
            if db_access_token.expires_at < Timestamp::now() {
                let token =
                    generate_access_token(&daraja.consumer_key, &daraja.consumer_secret).await;
                let now = Timestamp::now();
                let expires_at =
                    now.checked_add(SignedDuration::from_secs(token.expires_in.parse().unwrap()));

                toasty::update!(db_access_token {
                    value: &token.access_token,
                    expires_at: expires_at.unwrap()
                })
                .exec(&mut db)
                .await
                .unwrap();

                token.access_token
            } else {
                db_access_token.value
            }
        }
        Err(_) => {
            let now = Timestamp::now();
            let new_access_token =
                generate_access_token(&daraja.consumer_key, &daraja.consumer_secret).await;
            let expires_at = now
                .checked_add(SignedDuration::from_secs(
                    new_access_token.expires_in.parse().unwrap(),
                ))
                .unwrap();

            AppCache::create()
                .key("mpesa_access_token")
                .value(&new_access_token.access_token)
                .expires_at(expires_at)
                .exec(&mut db)
                .await
                .unwrap();

            new_access_token.access_token
        }
    };

    println!("{}", access_token);

    let response = MpesaExpress::new()
        .access_token(&access_token)
        .passkey(&daraja.passkey)
        .business_short_code(daraja.business_shortcode)
        .party_a(payload.phone_number)
        .party_b(daraja.business_shortcode)
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
