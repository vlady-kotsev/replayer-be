use sqlx::{PgPool, Pool, Postgres};

use crate::errors::{AppError, AppResult};

pub async fn create_connection_pool(database_url: &str) -> AppResult<Pool<Postgres>> {
    let pool = PgPool::connect(database_url)
        .await
        .map_err(|e| AppError::internal(e.to_string()))?;
    Ok(pool)
}
