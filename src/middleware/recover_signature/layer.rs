use crate::middleware::recover_signature::RecoverSignatureService;
use crate::utils::{deserialize_address, deserialize_signature};
use serde::Deserialize;
use solana_keypair::{Address, Signature};
use tower::Layer;

#[derive(Deserialize)]
pub struct RecoverSignatureBody {
    #[serde(deserialize_with = "deserialize_signature")]
    pub signature: Signature,
    pub message: String,
    #[serde(deserialize_with = "deserialize_address")]
    pub developer: Address,
}

#[derive(Clone)]
pub struct RecoverSignatureLayer;

impl<S> Layer<S> for RecoverSignatureLayer {
    type Service = RecoverSignatureService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RecoverSignatureService { inner }
    }
}
