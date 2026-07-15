use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;

use crate::{
    auth::auth_service::verify_jwt,
    shared::{AppError},
};
pub async fn auth_middleware(mut req: Request, next: Next) -> Response {
    let req_path = req.uri().path();

    if req_path.starts_with("/auth") {
        return next.run(req).await;
    }

    let Some(token) = get_token(&req) else {
        return AppError::UnAuthorized.into_response();
    };

    let user = match verify_jwt(token) {
        Ok(payload) => payload,
        Err(err) => return err.into_response(),
    };
    req.extensions_mut().insert(user);
    next.run(req).await
}
fn get_token(req: &Request) -> Option<String> {
    let cookie_jar = CookieJar::from_headers(req.headers());
    cookie_jar
        .get("auth.token")
        .map(|c| c.value().to_string())
        .or_else(|| {
            req.headers()
                .get("Authorization")?
                .to_str()
                .ok()?
                .strip_prefix("Bearer ")
                .map(|s| s.to_string())
        })
}
