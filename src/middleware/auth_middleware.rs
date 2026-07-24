use actix_web::{
    HttpMessage, body::MessageBody, dev::{ServiceRequest, ServiceResponse}, middleware::Next,
};

use crate::{auth::auth_service::verify_jwt, shared::AppError};
pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let req_path = req.uri().path();

    if req_path.starts_with("/auth") {
        return next.call(req).await;
    }
    let token = get_token(&req).ok_or(AppError::UnAuthorized)?;
    let user = verify_jwt(token)?;
    req.extensions_mut().insert(user);
    next.call(req).await
}
fn get_token(req: &ServiceRequest) -> Option<String> {
    let cookie = req.cookie("auth.token");
    cookie.map(|c| c.value().to_string()).or_else(|| {
        req.headers()
            .get("Authorization")?
            .to_str()
            .ok()?
            .strip_prefix("Bearer ")
            .map(|s| s.to_string())
    })
}
