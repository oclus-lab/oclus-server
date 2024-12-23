use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use validator::ValidationErrors;

#[derive(Clone, Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub kind: ApiErrorKind,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ApiErrorKind {
    Unknown,
    Unauthorized,
    Conflict(String), // the field already exist (e.g. user email conflict)
    InvalidData(String),
}

impl ApiError {
    pub fn new(status: StatusCode, kind: ApiErrorKind) -> Self {
        ApiError { status, kind }
    }
}

impl From<ApiErrorKind> for ApiError {
    fn from(value: ApiErrorKind) -> Self {
        match &value {
            ApiErrorKind::Unknown => ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, value),
            ApiErrorKind::Unauthorized => ApiError::new(StatusCode::UNAUTHORIZED, value),
            ApiErrorKind::Conflict(_field) => ApiError::new(StatusCode::CONFLICT, value),
            ApiErrorKind::InvalidData(_detail) => ApiError::new(StatusCode::BAD_REQUEST, value),
        }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.kind)
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        self.status
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status).json(Json(&self.kind))
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(value: anyhow::Error) -> Self {
        log::error!("{:?}", value);
        ApiError::from(ApiErrorKind::Unknown)
    }
}

impl From<ValidationErrors> for ApiError {
    fn from(value: ValidationErrors) -> Self {
        log::info!("invalid data provided: {:?}", value);
        ApiError::from(ApiErrorKind::InvalidData(value.to_string()))
    }
}
