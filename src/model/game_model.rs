use aes_gcm::{
    Aes256Gcm, Key,
    aead::{consts::U12, generic_array::GenericArray},
};
use base64::{Engine, engine::general_purpose::STANDARD};
use solana_keypair::Address;
use uuid::Uuid;

use crate::{
    errors::AppError,
    repository::{CreateGameDto, FetchGameDto},
};

pub struct CreateGameModel {
    pub name: String,
    pub developer: Address,
}

impl Into<CreateGameDto> for CreateGameModel {
    fn into(self) -> CreateGameDto {
        CreateGameDto {
            name: self.name,
            developer: self.developer.to_string(),
        }
    }
}

pub struct GameModel {
    pub id: Uuid,
    pub name: String,
    pub developer: Address,
    pub encryption_key: Key<Aes256Gcm>,
    pub nonce: GenericArray<u8, U12>,
}

impl TryFrom<FetchGameDto> for GameModel {
    type Error = AppError;

    fn try_from(dto: FetchGameDto) -> Result<Self, Self::Error> {
        let nonce_bytes: [u8; 12] = STANDARD
            .decode(dto.nonce)?
            .try_into()
            .map_err(|_| AppError::internal("Can't deserialize nonce"))?;
        let nonce = GenericArray::from_slice(&nonce_bytes).to_owned();

        let encryption_key_bytes: [u8; 32] = STANDARD
            .decode(dto.encryption_key)?
            .try_into()
            .map_err(|_| AppError::internal("Can't deserialize encryption key"))?;
        let encryption_key: Key<Aes256Gcm> = encryption_key_bytes.try_into()?;

        Ok(GameModel {
            id: dto.id,
            name: dto.name,
            developer: Address::from_str_const(&dto.developer),
            encryption_key,
            nonce,
        })
    }
}
