use crate::{
    errors::AppResult,
    repository::{CreateGameDto, FetchGameDto},
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
        let games = sqlx::query_as::<_, FetchGameDto>(sql).fetch_all(db).await?;
        Ok(games)
    }

    pub async fn create(db: &Pool<Postgres>, dto: CreateGameDto) -> AppResult<()> {
        let sql = include_str!("sql/create_game.sql");

        let key = Aes256Gcm::generate_key(OsRng);
        let key: [u8; 32] = key.try_into()?;
        let base64_encryption_key = STANDARD.encode(key);

        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let nonce: [u8; 12] = nonce.try_into()?;
        let base64_nonce = STANDARD.encode(nonce);

        sqlx::query(sql)
            .bind(dto.name)
            .bind(dto.developer)
            .bind(base64_encryption_key)
            .bind(base64_nonce)
            .execute(db)
            .await?;

        Ok(())
    }
}
