use crate::config::Config;
use solana_keypair::Keypair;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
    pub config: Arc<Config>,
    pub app_keypair: Arc<Keypair>,
}
