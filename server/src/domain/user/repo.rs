use crate::domain::user::model::user::{
    DeleteUserError, RegisterUserError, RegisterUserRequest, UpdateUserError, UpdateUserRequest,
    User,
};
use std::future::Future;

pub trait UserRepository {
    fn create_user(
        &self,
        email: &str,
        username: &str,
        password_hash: &str,
    ) -> impl Future<Output = Result<User, anyhow::Error>>;

    fn update_user(
        &self,
        email: &str,
        username: &str,
        password_hash: &str,
    ) -> impl Future<Output = Result<(), anyhow::Error>>;

    fn get_user(&self, id: i64) -> impl Future<Output = Result<Option<User>, anyhow::Error>>;

    fn get_user_by_email(
        &self,
        email: &str,
    ) -> impl Future<Output = Result<Option<User>, anyhow::Error>>;

    fn delete_user(&self, id: i64) -> impl Future<Output = Result<(), anyhow::Error>>;
}
