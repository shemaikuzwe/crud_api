use crate::{
    auth::{
        auth_service,
        dtos::{Login, Payload, Signup},
    },
    shared::{ApiResponse, AppError},
};

use axum::{Json, http::StatusCode};
pub async fn sign_up(
    Json(payload): Json<Signup>,
) -> Result<(StatusCode, Json<ApiResponse<Payload>>), AppError> {
    let result = auth_service::signup(payload).await?;
    Ok((
        StatusCode::CREATED,
        Json(ApiResponse {
            success: true,
            message: result.0,
            data: Some(result.1),
        }),
    ))
}

pub async fn login(
    Json(payload): Json<Login>,
) -> Result<(StatusCode, Json<ApiResponse<Payload>>), AppError> {
    let result = auth_service::login(payload).await?;
    Ok((
        StatusCode::OK,
        Json(ApiResponse {
            success: true,
            message: result.0,
            data: Some(result.1),
        }),
    ))
}
