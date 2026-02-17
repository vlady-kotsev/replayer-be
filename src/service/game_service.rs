use sqlx::{Pool, Postgres};

use crate::{
    errors::AppResult,
    model::{CreateGameModel, GameModel},
    repository::{CreateGameDto, GameRepository},
};

#[derive(Clone)]
pub struct GameService {
    db: Pool<Postgres>,
}

impl GameService {
    pub fn new(db: Pool<Postgres>) -> GameService {
        Self { db }
    }
    pub async fn get_all_games(&self) -> AppResult<Vec<GameModel>> {
        let game_dtos = GameRepository::get_all(&self.db).await?;
        let game_models = game_dtos
            .into_iter()
            .map(|dto| dto.try_into())
            .flat_map(|m| m)
            .collect::<Vec<GameModel>>();
        Ok(game_models)
    }

    pub async fn create_game(&self, model: CreateGameModel) -> AppResult<()> {
        let create_game_dto: CreateGameDto = model.into();

        GameRepository::create(&self.db, create_game_dto).await
    }
}
