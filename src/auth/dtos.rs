use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use thiserror::Error;

use crate::{shared::ApiResponse};

#[derive(Deserialize)]
pub struct Signup {
    pub email: String,
    pub password: String,
    pub name: String,
}
#[derive(Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Error, Debug)]
pub enum AuthError {
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
}

impl AuthError {
    pub fn status_code(&self) -> StatusCode {
        match &self {
            AuthError::InvalidCredentials => StatusCode::BAD_REQUEST,
            AuthError::EmailAlreadyExists => StatusCode::CONFLICT,
            AuthError::NotFound => StatusCode::NOT_FOUND,
            AuthError::Database(_) | AuthError::Password(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status_code();
        let body = Json(ApiResponse {
            message: self.to_string(),
            success: false,
        });
        (status, body).into_response()
    }
}
