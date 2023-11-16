pub mod status;

use actix_web::web;

pub fn configure_handlers(cfg: &mut web::ServiceConfig) {
    status::configure_status_routes(cfg);
    // Add more configurations as needed for other handlers
}
