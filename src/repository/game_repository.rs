use crate::{
    errors::{AppError, AppResult},
    model::KeyModel,
    repository::{CreateGameDto, FetchGameDto, GetKeyDto},
};
use aes_gcm::{
    Aes256Gcm,
    aead::{AeadCore, KeyInit, OsRng},
};
use base64::{Engine, engine::general_purpose::STANDARD};
use sqlx::{Pool, Postgres};

pub struct GameRepository;

impl GameRepository {
    pub async fn get_all(db: &Pool<Postgres>) -> AppResult<Vec<FetchGameDto>> {
        let sql = include_str!("sql/get_all_games.sql");
        let games = sqlx::query_as::<_, FetchGameDto>(sql)
            .fetch_all(db)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;
        Ok(games)
    }

    pub async fn create(db: &Pool<Postgres>, dto: CreateGameDto) -> AppResult<KeyModel> {
        let sql = include_str!("sql/create_game.sql");

        let key = Aes256Gcm::generate_key(OsRng);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        let base64_encryption_key = STANDARD.encode(key.as_slice());
        let base64_nonce = STANDARD.encode(nonce.as_slice());

        sqlx::query(sql)
            .bind(dto.name)
            .bind(dto.developer)
            .bind(base64_encryption_key)
            .bind(base64_nonce)
            .execute(db)
            .await
            .map_err(|e| AppError::bad_request(e.to_string()))?;

        Ok(KeyModel {
            encryption_key: key,
            nonce,
        })
    }

    pub async fn get_game_key(
        db: &Pool<Postgres>,
        dto: GetKeyDto,
    ) -> AppResult<Option<FetchGameDto>> {
        let sql = include_str!("sql/get_game_key.sql");
        let game_key = sqlx::query_as::<_, FetchGameDto>(sql)
            .bind(dto.game_name)
            .bind(dto.developer)
            .fetch_optional(db) // returns None if no match
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;
        Ok(game_key)
    }
}
