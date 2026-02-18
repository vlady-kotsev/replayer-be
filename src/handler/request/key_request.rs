use crate::model::GetKeyModel;
use crate::util::deserialize_address;
use serde::Deserialize;
use solana_keypair::Address;

#[derive(Deserialize)]
pub struct GetKeyRequest {
    pub name: String,
    #[serde(deserialize_with = "deserialize_address")]
    pub developer: Address,
    #[serde(deserialize_with = "deserialize_address")]
    pub player: Address,
}

impl From<GetKeyRequest> for GetKeyModel {
    fn from(req: GetKeyRequest) -> Self {
        GetKeyModel {
            name: req.name,
            developer: req.developer,
            player: req.player,
        }
    }
}
