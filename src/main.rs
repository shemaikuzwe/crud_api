use axum::{
    Router,
    routing::{post},
};
use crud_api::auth::auth_controller;
use tokio;

#[tokio::main]
async fn main() {
    // tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/auth/signup", post(auth_controller::sign_up))
        .route("/auth/login", post(auth_controller::login));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
