use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};

use crate::{
    entity::user, infra::db_conn, postgres_impl::user::{UserQueryPostgresImpl, UserModifierPostgresImpl},
    services::security::SecurityService,
};

pub async fn serve() -> std::io::Result<()> {
    // init infra
    let db = Arc::new(db_conn().await);

    // repo
    let user_query_repo: Arc<dyn user::UserQueryRepo> = Arc::new(UserQueryPostgresImpl::new(db.clone()));
    let user_modifier_repo: Arc<dyn user::UserModifierRepo> = Arc::new(UserModifierPostgresImpl::new(db.clone()));

    // init the core svc
    let security_service = Arc::new(SecurityService::new(
        user_query_repo.clone(),
        user_modifier_repo.clone(),
    ));

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // setup system

    let addr = "0.0.0.0:8000";
    let server = HttpServer::new(move || {
        let cors = Cors::permissive()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::from(security_service.clone()))
            .wrap(Logger::new(
                r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T ms"#,
            ))
            .service(crate::handlers::health_handler)
            .service(crate::handlers::login_handler)
            .service(crate::handlers::register_handler)
    })
    .bind(addr)?;

    log::info!("Listening in {}", addr);
    server.run().await
}
