use crate::model::{KeyModel, SignatureModel};
use base64::{Engine, engine::general_purpose::STANDARD};
use serde::Serialize;

#[derive(Serialize)]
pub struct KeyResponse {
    pub encryption_key: String,
    pub nonce: String,
    pub signature: String,
    pub valid_period: i64,
}

impl From<(KeyModel, SignatureModel)> for KeyResponse {
    fn from((key_model, signature_model): (KeyModel, SignatureModel)) -> Self {
        let encryption_key = STANDARD.encode(key_model.encryption_key.as_slice());

        let nonce = STANDARD.encode(key_model.nonce.as_slice());

        let signature = signature_model.signature.to_string();

        KeyResponse {
            encryption_key,
            nonce,
            signature,
            valid_period: signature_model.valid_period,
        }
    }
}
