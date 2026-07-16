use serde::Serialize;

use axum::{Json, http::StatusCode, response::IntoResponse};
use thiserror::Error;
use tracing::error;
#[derive(Debug,Serialize)]
pub struct ApiResponse<T:Serialize> {
    pub success: bool,
    pub message: String,
    pub data:Option<T>
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
            AppError::UnAuthorized=>StatusCode::UNAUTHORIZED,
            AppError::InvalidToken =>StatusCode::BAD_REQUEST,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Database(_) | AppError::Password(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        error!("{self:?}");
        let status = self.status_code();
        let body = Json(ApiResponse::<String> {
            message: self.to_string(),
            success: false,
            data:None
        });
        (status, body).into_response()
    }
}
