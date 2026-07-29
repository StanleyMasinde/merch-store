use merch_store::server::app::start_server;

#[tokio::main]
async fn main() {
    start_server().await
}
