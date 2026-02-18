use std::sync::Arc;

use chrono::{Duration, Utc};
use solana_keypair::{Keypair, Signer};

use crate::model::{GetKeyModel, SignatureModel};

#[derive(Clone)]
pub struct SignerService {
    pub authorizer: Arc<Keypair>,
    pub valid_period: i64, // hours
}

impl SignerService {
    pub fn new(authorizer: Arc<Keypair>, valid_period: i64) -> SignerService {
        Self {
            authorizer,
            valid_period,
        }
    }

    pub fn sign_message(&self, model: &GetKeyModel) -> SignatureModel {
        let valid_period = (Utc::now() + Duration::hours(self.valid_period)).timestamp();

        let payload = [
            &valid_period.to_le_bytes(),
            model.name.as_bytes(),
            model.developer.as_array(),
            model.player.as_array(),
        ]
        .concat();

        let signature = self.authorizer.sign_message(&payload);

        SignatureModel {
            signature,
            valid_period,
        }
    }
}
