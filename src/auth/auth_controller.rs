use crate::{
    auth::{
        auth_service,
        dtos::{AuthError, Login, Signup},
    },
    shared::ApiResponse,
};

use axum::{Json, http::StatusCode};
pub async fn sign_up(
    Json(payload): Json<Signup>,
) -> Result<(StatusCode, Json<ApiResponse<String>>), AuthError> {
    let result = auth_service::signup(payload).await?;
    Ok((
        StatusCode::CREATED,
        Json(ApiResponse {
            success: true,
            message: result,
        }),
    ))
}

pub async fn login(
    Json(payload): Json<Login>,
) -> Result<(StatusCode, Json<ApiResponse<String>>), AuthError> {
    let result = auth_service::login(payload).await?;
    Ok((
        StatusCode::OK,
        Json(ApiResponse {
            success: true,
            message: result,
        }),
    ))
}
