use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

#[derive(Clone, Debug)]
pub struct OclusDatabase {
    pool: Pool<Postgres>,
}

impl OclusDatabase {
    pub async fn new(url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new().connect(url).await?;
        Ok(Self { pool })
    }
}