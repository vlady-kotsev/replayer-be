use std::sync::Arc;

use chrono::{Duration, Utc};
use solana_keypair::{Address, Keypair, Signature, Signer};
use tracing::info;

#[derive(Clone)]
pub struct SignerService {
    pub authorizer: Arc<Keypair>,
    pub valid_for: i64, // hours
}

impl SignerService {
    pub fn new(authorizer: Arc<Keypair>, valid_for: i64) -> SignerService {
        Self {
            authorizer,
            valid_for,
        }
    }

    pub fn sign_message(&self, game_name: String, game_developer: Address) -> (Signature, i64) {
        let valid_period = (Utc::now() + Duration::hours(self.valid_for)).timestamp();

        let payload = [
            &valid_period.to_le_bytes(),
            game_name.as_bytes(),
            game_developer.as_array(),
        ]
        .concat();

        let signature = self.authorizer.sign_message(&payload);
        info!(
            "{}",
            self.authorizer.sign_message("test".as_bytes()).to_string()
        );
        (signature, valid_period)
    }
}
