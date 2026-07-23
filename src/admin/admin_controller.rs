use axum::{Json, extract::Path, http::StatusCode};
use uuid::Uuid;
use crate::{
    admin::{admin_service, dtos::Update_dto},
    models::{User, UserSelect},
    shared::{ApiResponse, AppError},
};


pub async fn get_users()  {
    let users = admin_service::get_users().await?;
    Ok((
        StatusCode::OK,
        Json(ApiResponse {
            success: true,
            message: String::from("users fetched successfully"),
            data: Some(users),
        }),
    ))
}

pub async fn get_user(
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<ApiResponse<UserSelect>>), AppError> {
    let user = admin_service::get_user_by_id(id).await?;
    println!("{:?}",user);
    Ok((
        StatusCode::OK,
        Json(ApiResponse {
            success: true,
            message: String::from("user fetched successfully"),
            data: Some(user),
        }),
    ))
}
pub async fn update_user(
    Path(id): Path<Uuid>,
    Json(payload): Json<Update_dto>,
) -> Result<(StatusCode, Json<ApiResponse<User>>), AppError> {
    let user = admin_service::update_user(id, payload).await?;
    Ok((
        StatusCode::OK,
        Json(ApiResponse {
            success: true,
            message: String::from("user updated successfully"),
            data: Some(user),
        }),
    ))
}

pub async fn delete_user(
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<ApiResponse<String>>), AppError> {
    let result = admin_service::delete_user(id).await?;
    Ok((
        StatusCode::OK,
        Json(ApiResponse {
            success: true,
            message: String::from("user deleted"),
            data: Some(result),
        }),
    ))
}
