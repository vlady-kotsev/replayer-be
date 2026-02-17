use sqlx::{PgPool, Pool, Postgres};

use crate::errors::AppResult;

pub async fn create_connection_pool(database_url: &str) -> AppResult<Pool<Postgres>> {
    let pool = PgPool::connect(database_url).await?;
    Ok(pool)
}
