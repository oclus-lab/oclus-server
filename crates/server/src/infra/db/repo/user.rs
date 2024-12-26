use crate::model::user::registration::PreRegistration;
use crate::model::user::User;
use sqlx::{Pool, Postgres};

#[derive(Clone, Debug)]
pub struct UserRepo {
    db: Pool<Postgres>,
}

impl UserRepo {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        email: &str,
        username: &str,
        srp_verifier: &[u8],
        srp_salt: &[u8],
    ) -> Result<i64, anyhow::Error> {
        let id = sqlx::query_scalar(
            r#"INSERT INTO users (email, username, srp_verifier, srp_salt) 
               VALUES ($1, $2, $3, $4) 
               RETURNING id"#,
        )
        .bind(email)
        .bind(username)
        .bind(srp_verifier)
        .bind(srp_salt)
        .fetch_one(&self.db)
        .await?;

        Ok(id)
    }

    pub async fn get(&self, id: i64) -> Result<Option<User>, anyhow::Error> {
        let user = sqlx::query_as(r#"SELECT * FROM users WHERE id = $1"#)
            .bind(id)
            .fetch_optional(&self.db)
            .await?;

        Ok(user)
    }

    pub async fn get_by_email(&self, email: &str) -> Result<Option<User>, anyhow::Error> {
        let user = sqlx::query_as(r#"SELECT * FROM users WHERE email = $1"#)
            .bind(email)
            .fetch_optional(&self.db)
            .await?;

        Ok(user)
    }

    pub async fn update(&self, id: i64, email: &str, username: &str) -> Result<(), anyhow::Error> {
        sqlx::query(
            r#"UPDATE users 
               SET email = $1 
               SET username = $2 
               WHERE id = $3"#,
        )
        .bind(email)
        .bind(username)
        .bind(id)
        .execute(&self.db)
        .await?;

        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), anyhow::Error> {
        sqlx::query(r#"DELETE FROM users WHERE id = $1"#)
            .bind(id)
            .execute(&self.db)
            .await?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct PreRegistrationRepo {
    db: Pool<Postgres>,
}

impl PreRegistrationRepo {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self { db }
    }

    pub async fn create(&self, email: &str, otp: &str) -> Result<i64, anyhow::Error> {
        let id = sqlx::query_scalar(
            r#"INSERT INTO pre_registrations (email, otp) VALUES ($1, $2) RETURNING id"#,
        )
        .bind(email)
        .bind(otp)
        .fetch_one(&self.db)
        .await?;

        Ok(id)
    }

    pub async fn get(&self, id: i64) -> Result<Option<PreRegistration>, anyhow::Error> {
        let pre_registration = sqlx::query_as(r#"SELECT * FROM pre_registrations WHERE id = $1"#)
            .bind(id)
            .fetch_optional(&self.db)
            .await?;

        Ok(pre_registration)
    }

    pub async fn delete_by_email(&self, email: &str) -> Result<(), anyhow::Error> {
        sqlx::query(r#"DELETE FROM pre_registrations WHERE email = $1"#)
            .bind(email)
            .execute(&self.db)
            .await?;

        Ok(())
    }
}
