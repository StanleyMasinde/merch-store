use merch_store::server::types::app_config::AppConfig;
use toasty_cli::{Config, ToastyCli};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let config = Config::load()?;
    let AppConfig {
        daraja: _,
        database,
    } = AppConfig::with_path("config.toml".into());

    let db = toasty::Db::builder()
        .models(toasty::models!(merch_store::*))
        .connect(&database.connection)
        .await?;

    let cli = ToastyCli::with_config(db, config);
    cli.parse_from(args).await?;
    Ok(())
}
