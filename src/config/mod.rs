use crate::{
    errors::{AppError, AppResult},
    util::{deserialize_address, deserialize_keypair},
};
use serde::Deserialize;
use solana_keypair::Address;
use tokio::fs::read_to_string;

#[derive(Deserialize)]
pub struct Config {
    pub app: AppConfig,
    pub solana: SolanaConfig,
}

#[derive(Deserialize)]
pub struct SolanaConfig {
    #[serde(deserialize_with = "deserialize_keypair")]
    pub keypair_bytes: [u8; 64],
    pub rpc_url: String,
    #[serde(deserialize_with = "deserialize_address")]
    pub program_id: Address,
}

#[derive(Deserialize)]
pub struct AppConfig {
    pub port: u16,
    pub host: String,
    pub database_url: String,
}

pub async fn load_config() -> AppResult<Config> {
    let config_file = std::env::var("CONFIG_DIR").unwrap_or(String::from("config/config.toml"));

    let content = read_to_string(config_file)
        .await
        .map_err(|e| AppError::internal(e.to_string()))?;
    let config =
        toml::from_str::<Config>(&content).map_err(|e| AppError::internal(e.to_string()))?;

    Ok(config)
}
