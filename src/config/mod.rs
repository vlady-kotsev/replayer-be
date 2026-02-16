use anyhow::Result;
use base64::{Engine, engine::general_purpose::STANDARD};
use serde::{Deserialize, Deserializer, de};
use tokio::fs::read_to_string;

#[derive(Deserialize)]
pub struct Config {
    pub app: AppConfig,
    pub solana: SolanaConfig,
}

#[derive(Deserialize)]
pub struct SolanaConfig {
    #[serde(deserialize_with = "deserialize_base64")]
    pub keypair_bytes: [u8; 64],
}

#[derive(Deserialize)]
pub struct AppConfig {
    pub port: u16,
    pub host: String,
    pub database_url: String,
}

pub async fn load_config() -> Result<Config> {
    let content = read_to_string("config/config.toml").await?;
    let config = toml::from_str::<Config>(&content)?;

    Ok(config)
}

fn deserialize_base64<'de, D>(deserializer: D) -> Result<[u8; 64], D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let bytes = STANDARD.decode(s).map_err(de::Error::custom)?;
    bytes
        .try_into()
        .map_err(|_| de::Error::custom("expected 64 bytes"))
}
