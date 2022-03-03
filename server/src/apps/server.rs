use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{http, middleware::Logger, web, App, HttpServer};

use crate::{
    entity::user, infra::db_conn, postgres_impl::user::UserQueryImpl,
    services::security::SecurityService,
};

pub async fn serve() -> std::io::Result<()> {
    // init infra
    let db = Arc::new(db_conn().await);

    // repo
    let user_query_repo: Arc<dyn user::UserQueryRepo> = Arc::new(UserQueryImpl::new(db));

    // init the core svc
    let security_service = Arc::new(SecurityService::new(user_query_repo.clone()));

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // setup system

    let addr = "0.0.0.0:8000";
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .send_wildcard()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::from(security_service.clone()))
            .wrap(Logger::new(
                r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T ms"#,
            ))
            .service(crate::handlers::health_handler)
            .service(crate::handlers::login_handler)
    })
    .bind(addr)?;

    println!("Listening in {}", addr);
    server.run().await
}
