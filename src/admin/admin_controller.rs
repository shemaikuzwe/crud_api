use crate::{
    admin::{admin_service, dtos::Update_dto},
    models::{User, UserSelect},
    shared::{ApiResponse, AppError},
};
use actix_web::web;
use uuid::Uuid;

#[actix_web::get("/users")]
pub async fn get_users() -> Result<web::Json<ApiResponse<Vec<UserSelect>>>, actix_web::Error> {
    let users = admin_service::get_users().await?;
    Ok(web::Json(ApiResponse {
        success: true,
        message: String::from("users fetched successfully"),
        data: Some(users),
    }))
}

#[actix_web::get("/users/{id}")]
pub async fn get_user(id: web::Path<Uuid>) -> Result<web::Json<ApiResponse<UserSelect>>, AppError> {
    let user = admin_service::get_user_by_id(id.into_inner()).await?;
    println!("{:?}", user);
    Ok(web::Json(ApiResponse {
        success: true,
        message: String::from("user fetched successfully"),
        data: Some(user),
    }))
}
#[actix_web::put("/users/{id}")]
pub async fn update_user(
    id: web::Path<Uuid>,
    payload: web::Json<Update_dto>,
) -> Result<web::Json<ApiResponse<User>>, AppError> {
    let user = admin_service::update_user(id.into_inner(), payload.into_inner()).await?;
    Ok(web::Json(ApiResponse {
        success: true,
        message: String::from("user updated successfully"),
        data: Some(user),
    }))
}
#[actix_web::delete("/users/{id}")]
pub async fn delete_user(id: web::Path<Uuid>) -> Result<web::Json<ApiResponse<String>>, AppError> {
    let result = admin_service::delete_user(id.into_inner()).await?;
    Ok(web::Json(ApiResponse {
        success: true,
        message: String::from("user deleted"),
        data: Some(result),
    }))
}
