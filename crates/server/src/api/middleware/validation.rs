use crate::api::error::ApiError;
use actix_web::dev::Payload;
use actix_web::web::Json;
use actix_web::{FromRequest, HttpRequest};
use serde::de::DeserializeOwned;
use std::future::Future;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use validator::Validate;

pub struct ValidatedJson<T>(pub T);

impl<T> Deref for ValidatedJson<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for ValidatedJson<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> FromRequest for ValidatedJson<T>
where
    T: DeserializeOwned + Validate + 'static,
{
    type Error = ApiError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let json = Json::<T>::from_request(req, payload);
        Box::pin(async move {
            let json = json
                .await
                .map_err(|err| ApiError::InvalidData(err.to_string()))?;

            json.validate()
                .map_err(|err| ApiError::InvalidData(err.to_string()))?;

            Ok(ValidatedJson(json.0))
        })
    }
}
