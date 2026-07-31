use std::path::PathBuf;

use serde::Deserialize;

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

#[derive(Deserialize)]
pub struct AppConfig {
    pub daraja: Daraja,
    pub database: DbConfig,
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

    pub fn with_path(path: PathBuf) -> Self {
        let contents = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Couldn't read config at {}", path.to_string_lossy()));
        toml::from_str(&contents).expect("Config is malformed.")
    }
}
