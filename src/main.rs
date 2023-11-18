mod config;
mod handlers;
mod integrations;
// use actix_web::{App, HttpServer};
// use handlers::configure_handlers;
use integrations::load_integrations;
use config::from_file;

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
fn main() {
    let _config = from_file("config/service.yaml").expect("Failed to load configuration");
    let loaded_integrations = load_integrations();
    println!("Loaded Integrations: {:?}", loaded_integrations);
    // HttpServer::new(|| {
    //     App::new()
    //         .configure(configure_handlers)
    // })
    // .bind(format!("127.0.0.1:{}", config.server.port))?
    // .run()
    // .await
}
