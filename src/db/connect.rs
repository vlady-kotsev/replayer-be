use anyhow::Result;
use sqlx::{PgPool, Pool, Postgres};

pub async fn create_connection_pool() -> Result<Pool<Postgres>> {
    let pool = PgPool::connect("postgres://postgres:postgres@localhost/replayer").await?;
    Ok(pool)
}
