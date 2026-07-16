use crate::{
    auth::{
        auth_service,
        dtos::{Login, Payload, Signup},
    },
    shared::{ApiResponse, AppError},
};

use axum::{Json, http::StatusCode};
use axum_extra::extract::CookieJar;

pub async fn sign_up(
    Json(payload): Json<Signup>,
) -> Result<(StatusCode, CookieJar, Json<ApiResponse<Payload>>), AppError> {
    let result = auth_service::signup(payload).await?;
    Ok((
        StatusCode::CREATED,
        result.jar,
        Json(ApiResponse {
            success: true,
            message: result.token,
            data: Some(result.payload),
        }),
    ))
}

pub async fn login(
    Json(payload): Json<Login>,
) -> Result<(StatusCode, CookieJar, Json<ApiResponse<Payload>>), AppError> {
    let result = auth_service::login(payload).await?;

    Ok((
        StatusCode::OK,
        result.jar,
        Json(ApiResponse {
            success: true,
            message: result.token,
            data: Some(result.payload),
        }),
    ))
}
