use axum::{
    Router,
    routing::{get, post},
};
use crud_api::{admin::admin_controller, auth::auth_controller};
use tokio;

#[tokio::main]
async fn main() {
    // tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/auth/signup", post(auth_controller::sign_up))
        .route("/auth/login", post(auth_controller::login))
        .route("/admin/users", get(admin_controller::get_users))
        .route(
            "/admin/users/{id}",
            get(admin_controller::get_user)
                .put(admin_controller::update_user)
                .delete(admin_controller::delete_user),
        );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
