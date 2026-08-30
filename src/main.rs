use crud_api::{
    admin::admin_controller, auth::auth_controller, config,
    middleware::auth_middleware::auth_middleware,
};
use tokio;
// use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;
use yam::router::router::Router;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("tower_http=debug,crud_api=debug"))
        .init();
    let mut app = Router::new(yam::router::router::RouterConfig {
        route_prefix: "/v1".into(),
        ..Default::default()
    });
    app.post("/auth/signup", auth_controller::sign_up)
        .post("/auth/login", auth_controller::login)
        .get("/admin/users", admin_controller::get_users)
        .get("/admin/users/{id}", admin_controller::get_user)
        .put("/admin/users/{id}", admin_controller::update_user)
        .delete("/admin/users/{id}", admin_controller::delete_user)
        // .middleware(TraceLayer::new_for_http())
        .middleware(auth_middleware);
    let port = config().port;
    let listener = tokio::net::TcpListener::bind(format!("localhost:{port}"))
        .await
        .unwrap();
    info!("Server started on http://localhost:{:?}", port);

    app.serve(listener).await.unwrap()
}
