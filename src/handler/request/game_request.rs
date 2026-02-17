use crate::{model::CreateGameModel, util::deserialize_address};
use serde::Deserialize;
use solana_keypair::Address;

#[derive(Deserialize)]
pub struct CreateGameRequest {
    pub name: String,
    #[serde(deserialize_with = "deserialize_address")]
    pub developer: Address,
}

impl Into<CreateGameModel> for CreateGameRequest {
    fn into(self) -> CreateGameModel {
        CreateGameModel {
            name: self.name,
            developer: self.developer,
        }
    }
}
