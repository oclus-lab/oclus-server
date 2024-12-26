use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use url::Url;

pub mod repo;

pub async fn init_db(db_url: &Url) -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = PgPoolOptions::new().connect(db_url.as_str()).await?;
    Ok(pool)
}