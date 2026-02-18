use sqlx::{Pool, Postgres};

use crate::{
    AppResult,
    errors::AppError,
    model::{GetKeyModel, KeyModel},
    repository::GameRepository,
};

#[derive(Clone)]
pub struct KeyService {
    db: Pool<Postgres>,
}

impl KeyService {
    pub fn new(db: Pool<Postgres>) -> KeyService {
        Self { db }
    }

    pub async fn get_game_key(&self, model: GetKeyModel) -> AppResult<KeyModel> {
        let Some(dto) = GameRepository::get_game_key(&self.db, model.into()).await? else {
            return Err(AppError::not_found("Game key not found"));
        };

        dto.try_into()
    }
}
