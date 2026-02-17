use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct FetchGameDto {
    pub id: Uuid,
    pub name: String,
    pub developer: String,
    pub encryption_key: String,
    pub nonce: String,
}

pub struct CreateGameDto {
    pub name: String,
    pub developer: String,
}
