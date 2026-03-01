use crate::model::CreateGameModel;
use crate::utils::deserialize_address;
use serde::Deserialize;
use solana_keypair::Address;

#[derive(Deserialize)]
pub struct CreateGameRequest {
    pub name: String,
    #[serde(deserialize_with = "deserialize_address")]
    pub developer: Address,
}

impl From<CreateGameRequest> for CreateGameModel {
    fn from(req: CreateGameRequest) -> Self {
        CreateGameModel {
            name: req.name,
            developer: req.developer,
        }
    }
}
