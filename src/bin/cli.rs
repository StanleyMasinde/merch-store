use merch_store::server::app::AppConfig;
use toasty_cli::{Config, ToastyCli};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::load()?;
    let AppConfig {
        daraja: _,
        database,
    } = AppConfig::load();

    let db = toasty::Db::builder()
        .models(toasty::models!(merch_store::*))
        .connect(&database.connection)
        .await?;

    let cli = ToastyCli::with_config(db, config);
    cli.parse_and_run().await?;

    Ok(())
}
