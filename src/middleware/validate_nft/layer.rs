use crate::{client::SolanaClient, middleware::ValidateNftService};
use serde::Deserialize;
use solana_keypair::Address;
use std::sync::Arc;
use tower::Layer;
use crate::util::deserialize_address;

#[derive(Deserialize)]
pub struct ValidateNftBody {
    #[serde(deserialize_with = "deserialize_address")]
    pub developer: Address,
    #[serde(deserialize_with = "deserialize_address")]
    pub player: Address,
    pub game_name: String,
}

#[derive(Clone)]
pub struct ValidateNftLayer {
    client: Arc<SolanaClient>,
    program_id: Address,
}

impl ValidateNftLayer {
    pub fn new(client: Arc<SolanaClient>, program_id: Address) -> ValidateNftLayer {
        Self { client, program_id }
    }
}

impl<S> Layer<S> for ValidateNftLayer {
    type Service = ValidateNftService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ValidateNftService {
            inner,
            client: self.client.clone(),
            program_id: self.program_id,
        }
    }
}
