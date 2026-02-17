use anyhow::Result;
use sqlx::{Pool, Postgres};

use crate::repository::{CreateGameDto, FetchGameDto};

pub struct GameRepository;

impl GameRepository {
    pub async fn get_all_games(db: &Pool<Postgres>) -> Result<Vec<FetchGameDto>> {
        let sql = include_str!("sql/get_all_games.sql");
        let games = sqlx::query_as::<_, FetchGameDto>(sql).fetch_all(db).await?;
        Ok(games)
    }

    pub async fn create_game(db: &Pool<Postgres>, dto: CreateGameDto) -> Result<()> {
        let sql = include_str!("sql/create_game.sql");
        sqlx::query(sql)
            .bind(dto.name)
            .bind(dto.developer)
            .bind(dto.encryption_key)
            .execute(db)
            .await?;

        Ok(())
    }
}
