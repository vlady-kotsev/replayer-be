use base64::{Engine, engine::general_purpose::STANDARD};
use serde::Serialize;

use crate::model::{GameModel, KeyModel};

#[derive(Serialize)]
pub struct GameResponse {
    pub name: String,
    pub developer: String,
}

impl From<GameModel> for GameResponse {
    fn from(model: GameModel) -> Self {
        GameResponse {
            name: model.name,
            developer: model.developer.to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct CreateGameResponse {
    pub encryption_key: String,
    pub nonce: String,
}

impl From<KeyModel> for CreateGameResponse {
    fn from(key_model: KeyModel) -> Self {
        CreateGameResponse {
            encryption_key: STANDARD.encode(key_model.encryption_key.as_slice()),
            nonce: STANDARD.encode(key_model.nonce.as_slice()),
        }
    }
}
