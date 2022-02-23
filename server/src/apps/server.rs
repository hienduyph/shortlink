use actix_web::{web, App, HttpServer};

pub async fn serve() -> std::io::Result<()> {
    let addr = "0.0.0.0:8000";
    let server = HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/").route("", web::get().to(crate::handlers::health::health_handle)),
            )
            .service(
                web::scope("/health")
                    .route("", web::get().to(crate::handlers::health::health_handle)),
            )
    })
    .bind(addr)?;

    println!("Listening in {}", addr);
    server.run().await
}
