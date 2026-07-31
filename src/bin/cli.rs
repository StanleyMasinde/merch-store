use merch_store::server::types::app_config::AppConfig;
use toasty_cli::{Config, ToastyCli};

fn extract_config_path(args: &mut Vec<String>) -> String {
    if let Some(pos) = args.iter().position(|a| a == "--config") {
        args.remove(pos); // remove the flag
        if pos < args.len() {
            return args.remove(pos); // remove and return its value
        }
    }
    "config.toml".to_string()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut args: Vec<String> = std::env::args().collect();
    let config_path = extract_config_path(&mut args); // strips --config before clap ever sees it

    let config = Config::load()?;
    let AppConfig {
        daraja: _,
        database,
    } = AppConfig::with_path(config_path.into());

    let db = toasty::Db::builder()
        .models(toasty::models!(merch_store::*))
        .connect(&database.connection)
        .await?;

    let cli = ToastyCli::with_config(db, config);
    cli.parse_from(args).await?; // clean argv, no --config left in it
    Ok(())
}
