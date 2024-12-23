use crate::model::user::{PreRegistration, User};
use sqlx::{Pool, Postgres};

#[derive(Clone, Debug)]
pub struct UserRepo {
    database: Pool<Postgres>,
}

impl UserRepo {
    pub fn new(database: Pool<Postgres>) -> Self {
        Self { database }
    }

    pub async fn create(&self, email: &str, username: &str) -> Result<i64, anyhow::Error> {
        let id = sqlx::query_scalar(
            r#"INSERT INTO users (email, username) VALUES ($1, $2) RETURNING id"#,
        )
        .bind(email)
        .bind(username)
        .fetch_one(&self.database)
        .await?;

        Ok(id)
    }

    pub async fn get(&self, id: i64) -> Result<Option<User>, anyhow::Error> {
        let user = sqlx::query_as(r#"SELECT * FROM users WHERE id = $1"#)
            .bind(id)
            .fetch_optional(&self.database)
            .await?;

        Ok(user)
    }

    pub async fn get_by_email(&self, email: &str) -> Result<Option<User>, anyhow::Error> {
        let user = sqlx::query_as(r#"SELECT * FROM users WHERE email = $1"#)
            .bind(email)
            .fetch_optional(&self.database)
            .await?;

        Ok(user)
    }

    pub async fn update(&self, email: &str, username: &str) -> Result<(), anyhow::Error> {
        todo!()
    }

    pub async fn delete(&self, id: i64) -> Result<(), anyhow::Error> {
        todo!()
    }
}

#[derive(Clone, Debug)]
pub struct PreRegistrationRepo {
    database: Pool<Postgres>,
}

impl PreRegistrationRepo {
    pub fn new(database: Pool<Postgres>) -> Self {
        Self { database }
    }

    pub async fn create(&self, email: &str, otp: &str) -> Result<i64, anyhow::Error> {
        let id = sqlx::query_scalar(
            r#"INSERT INTO pre_registrations (email, otp) VALUES ($1, $2) RETURNING id"#,
        )
        .bind(email)
        .bind(otp)
        .fetch_one(&self.database)
        .await?;

        Ok(id)
    }

    pub async fn get(&self, id: i64) -> Result<Option<PreRegistration>, anyhow::Error> {
        let pre_registration = sqlx::query_as(r#"SELECT * FROM pre_registrations WHERE id = $1"#)
            .bind(id)
            .fetch_optional(&self.database)
            .await?;

        Ok(pre_registration)
    }

    pub async fn delete(&self, id: i64) -> Result<(), anyhow::Error> {
        todo!()
    }
}
