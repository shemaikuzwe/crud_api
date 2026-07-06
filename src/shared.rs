use serde::Serialize;

use axum::{Json, http::StatusCode, response::IntoResponse};
use thiserror::Error;
use uuid::Error;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: T,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("email already exists")]
    EmailAlreadyExists,

    #[error("resource not found")]
    NotFound,
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
            AppError::InvalidCredentials |AppError::Uuid(_) => StatusCode::BAD_REQUEST,
            AppError::EmailAlreadyExists => StatusCode::CONFLICT,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Database(_) | AppError::Password(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status_code();
        let body = Json(ApiResponse {
            message: self.to_string(),
            success: false,
        });
        (status, body).into_response()
    }
}
