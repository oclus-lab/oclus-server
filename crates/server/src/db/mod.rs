use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

pub mod repo;

pub async fn init_database(database_url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = PgPoolOptions::new().connect(database_url).await?;
    Ok(pool)
}