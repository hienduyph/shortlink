#[actix_web::main]
async fn main() -> std::io::Result<()> {
    shortlink::apps::server::serve().await
}
