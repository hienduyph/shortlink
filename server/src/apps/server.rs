use actix_web::{middleware::Logger, web, App, HttpServer};

pub async fn serve() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let addr = "0.0.0.0:8000";
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::new(
                r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T ms"#,
            ))
            .service(web::scope("/").route("", web::get().to(crate::handlers::health_handler)))
            .service(
                web::scope("/health").route("", web::get().to(crate::handlers::health_handler)),
            )
            .service(web::scope("/login").route("", web::post().to(crate::handlers::login_handler)))
    })
    .bind(addr)?;

    println!("Listening in {}", addr);
    server.run().await
}
