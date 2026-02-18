use crate::{
    errors::AppError,
    repository::{FetchGameDto, GetKeyDto},
};
use aes_gcm::{
    Aes256Gcm, Key,
    aead::{consts::U12, generic_array::GenericArray},
};
use base64::{Engine, engine::general_purpose::STANDARD};
use solana_keypair::Address;

#[derive(Clone)]
pub struct GetKeyModel {
    pub name: String,
    pub developer: Address,
    pub player: Address,
}

impl From<GetKeyModel> for GetKeyDto {
    fn from(model: GetKeyModel) -> Self {
        GetKeyDto {
            game_name: model.name,
            developer: model.developer.to_string(),
        }
    }
}

pub struct KeyModel {
    pub encryption_key: Key<Aes256Gcm>,
    pub nonce: GenericArray<u8, U12>,
}

impl TryFrom<FetchGameDto> for KeyModel {
    type Error = AppError;

    fn try_from(dto: FetchGameDto) -> Result<Self, Self::Error> {
        let nonce_bytes: [u8; 12] = STANDARD
            .decode(dto.nonce)
            .map_err(|e| AppError::internal(e.to_string()))?
            .try_into()
            .map_err(|_| AppError::internal("Can't deserialize nonce"))?;

        let nonce = GenericArray::from_slice(&nonce_bytes).to_owned();

        let encryption_key_bytes: [u8; 32] = STANDARD
            .decode(dto.encryption_key)
            .map_err(|e| AppError::internal(e.to_string()))?
            .try_into()
            .map_err(|_| AppError::internal("Can't deserialize encryption key"))?;

        let encryption_key: Key<Aes256Gcm> = encryption_key_bytes.into();

        Ok(KeyModel {
            encryption_key,
            nonce,
        })
    }
}
