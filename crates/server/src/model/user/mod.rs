use sqlx::FromRow;

pub mod registration;

#[derive(FromRow, Clone, Debug)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub srp_verifier: Vec<u8>,
    pub srp_salt: Vec<u8>,
}