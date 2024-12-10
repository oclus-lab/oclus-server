use crate::domain::user::model::auth::*;
use crate::domain::user::model::user::*;
use std::future::Future;

pub trait UserService {
    fn register_user(
        &self,
        req: RegisterUserRequest,
    ) -> impl Future<Output = Result<(), RegisterUserError>>;

    fn update_user(
        &self,
        id: i64,
        req: UpdateUserRequest,
    ) -> impl Future<Output = Result<(), UpdateUserError>>;

    fn update_user_password(
        &self,
        id: i64,
        req: UpdateUserPasswordRequest,
    ) -> impl Future<Output = Result<(), UpdateUserPasswordError>>;

    fn delete_user(&self, id: i64) -> impl Future<Output = Result<(), DeleteUserError>>;

    fn get_user(&self, id: i64) -> impl Future<Output = Result<Option<User>, anyhow::Error>>;
}

pub trait AuthService {
    fn login(&self, req: LoginRequest) -> impl Future<Output = Result<TokenPair, LoginError>>;
    fn refresh_tokens(
        &self,
        token: String,
    ) -> impl Future<Output = Result<TokenPair, RefreshTokensError>>;
}
