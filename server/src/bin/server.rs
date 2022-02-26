#[tokio::main]
async fn main() -> std::io::Result<()> {
    shortlink_server::apps::server::serve().await
}
