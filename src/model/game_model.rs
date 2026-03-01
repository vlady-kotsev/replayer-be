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

impl From<CreateGameModel> for CreateGameDto {
    fn from(model: CreateGameModel) -> Self {
        CreateGameDto {
            name: model.name,
            developer: model.developer.to_string(),
        }
    }
}

pub struct GameModel {
    pub id: Uuid,
    pub name: String,
    pub developer: Address,
}

impl TryFrom<FetchGameDto> for GameModel {
    type Error = AppError;

    fn try_from(dto: FetchGameDto) -> Result<Self, Self::Error> {
        let developer = dto
            .developer
            .parse()
            .map_err(|_| AppError::internal("Failed to parse developer address"))?;

        Ok(GameModel {
            id: dto.id,
            name: dto.name,
            developer,
        })
    }
}
