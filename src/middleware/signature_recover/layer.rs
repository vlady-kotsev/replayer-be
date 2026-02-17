use crate::middleware::signature_recover::SignatureRecoverService;
use crate::util::{deserialize_address, deserialize_signature};
use serde::Deserialize;
use solana_keypair::{Address, Signature};
use tower::Layer;

#[derive(Deserialize)]
pub struct SignatureRecoverBody {
    #[serde(deserialize_with = "deserialize_signature")]
    pub signature: Signature,
    pub message: String,
    #[serde(deserialize_with = "deserialize_address")]
    pub developer: Address,
}

#[derive(Clone)]
pub struct SignatureRecoverLayer;

impl<S> Layer<S> for SignatureRecoverLayer {
    type Service = SignatureRecoverService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SignatureRecoverService { inner }
    }
}
