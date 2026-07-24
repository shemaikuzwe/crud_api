use actix_web::{
    App, HttpServer,
    middleware::{self, Logger},
    web,
};
use crud_api::{
    admin::admin_controller::{delete_user, get_user, get_users, update_user},
    auth::auth_controller::{login, sign_up},
    config,
    middleware::auth_middleware::auth_middleware,
};
use env_logger::Env;
use tokio::{self, io};
use tracing::info;


#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let port = config().port;
    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(middleware::from_fn(auth_middleware))
            .service(
                web::scope("/v1")
                    .service(
                        web::scope("/admin")
                            .service(get_users)
                            .service(get_user)
                            .service(update_user)
                            .service(delete_user),
                    )
                    .service(web::scope("/auth").service(login).service(sign_up)),
            )
    })
    .bind(format!("localhost:{port}"))?;
    info!("Server started on http://localhost:{:?}", port);
    server.run().await
}
