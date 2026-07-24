use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;

use thiserror::Error;
use tracing::error;
#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("email already exists")]
    EmailAlreadyExists,
    #[error("internal server error")]
    InternalServerError,
    #[error("invalid token")]
    InvalidToken,
    #[error("resource not found")]
    NotFound,
    #[error("Unauthorized")]
    UnAuthorized,
    #[error("Database error")]
    Database(#[from] diesel::result::Error),

    #[error("password error")]
    Password(#[from] bcrypt::BcryptError),
    #[error("failed to parse uuid")]
    Uuid(#[from] uuid::Error),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match &self {
            AppError::InvalidCredentials | AppError::Uuid(_) => StatusCode::BAD_REQUEST,
            AppError::EmailAlreadyExists => StatusCode::CONFLICT,
            AppError::UnAuthorized => StatusCode::UNAUTHORIZED,
            AppError::InvalidToken => StatusCode::BAD_REQUEST,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Database(_) | AppError::Password(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
impl ResponseError for AppError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        error!("{self:?}");
        HttpResponse::build(self.status_code()).json(ApiResponse::<()> {
            data: None,
            message: self.to_string(),
            success: false,
        })
    }
    fn status_code(&self) -> actix_web::http::StatusCode {
        self.status_code()
    }
}
