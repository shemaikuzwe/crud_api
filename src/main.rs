use axum::{
    Router, middleware as axum_middleware, routing::{get, post},
};
use crud_api::{admin::admin_controller, auth::auth_controller, middleware::{self, auth_middleware::auth_middleware}};
use tokio;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter(EnvFilter::new("tower_http=debug,crud_api=debug")).init();
    let app = Router::new()
        .route("/auth/signup", post(auth_controller::sign_up))
        .route("/auth/login", post(auth_controller::login))
        .route("/admin/users", get(admin_controller::get_users))
        .route(
            "/admin/users/{id}",
            get(admin_controller::get_user)
                .put(admin_controller::update_user)
                .delete(admin_controller::delete_user),
        )
        .layer(TraceLayer::new_for_http())
        .layer(axum_middleware::from_fn(auth_middleware));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
