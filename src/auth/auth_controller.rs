use actix_web::{
    HttpResponse,
    web
};

use crate::{
    auth::{
        auth_service::{self, set_cookie},
        dtos::{Login,Signup},
    },
    shared::{ApiResponse, AppError},
};
#[actix_web::post("/sign-up")]
pub async fn sign_up(payload: web::Json<Signup>) -> Result<HttpResponse, AppError> {
    let result = auth_service::signup(payload.into_inner()).await?;
    let cookie = set_cookie(&result.token);
    Ok(HttpResponse::Created().cookie(cookie).json(ApiResponse {
        success: true,
        message: result.token,
        data: Some(result.payload),
    }))
}
#[actix_web::post("/login")]
pub async fn login(payload: web::Json<Login>) -> Result<HttpResponse, AppError> {
    let result = auth_service::login(payload.into_inner()).await?;
    let cookie = set_cookie(&result.token);
    Ok(HttpResponse::Ok().cookie(cookie).json(ApiResponse {
        success: true,
        message: result.token,
        data: Some(result.payload),
    }))
}
