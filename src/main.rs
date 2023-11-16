mod config;
mod handlers;
use actix_web::{App, HttpServer};
use handlers::configure_handlers;
use config::config::from_file;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = from_file("config/service.yaml").expect("Failed to load configuration");
    HttpServer::new(|| {
        App::new()
            .configure(configure_handlers)
    })
    .bind(format!("127.0.0.1:{}", config.server.port))?
    .run()
    .await
}
