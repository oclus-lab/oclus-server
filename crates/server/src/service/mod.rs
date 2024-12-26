use crate::config::Config;
use crate::infra::db::repo::user::{PreRegistrationRepo, UserRepo};
use crate::service::user::UserService;
use sqlx::{Pool, Postgres};

mod user;

#[derive(Clone, Debug)]
pub struct ServiceCollection {
    pub user_service: UserService,
}

impl ServiceCollection {
    pub fn new(config: &Config, db: Pool<Postgres>) -> Result<Self, anyhow::Error> {
        let user_repo = UserRepo::new(db.clone());
        let pre_registration_repo = PreRegistrationRepo::new(db);

        let user_service = UserService::new(&config.otp_secret, user_repo, pre_registration_repo)?;
        Ok(Self { user_service })
    }
}
