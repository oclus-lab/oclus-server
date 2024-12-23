use crate::db::repo::user::{PreRegistrationRepo, UserRepo};
use sqlx::{Pool, Postgres};

pub mod user;

#[derive(Clone, Debug)]
pub struct RepoCollection {
    pub user_repo: UserRepo,
    pub pre_registration_repo: PreRegistrationRepo,
}

impl RepoCollection {
    pub fn new(database: Pool<Postgres>) -> Self {
        let user_repo = UserRepo::new(database.clone());
        let pre_registration_repo = PreRegistrationRepo::new(database);
        
        Self {
            user_repo,
            pre_registration_repo,
        }
    }
}
