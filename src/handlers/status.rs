use actix_web::{web, HttpResponse};

async fn uptime() -> HttpResponse {
    // Implementation for uptime
    HttpResponse::Ok().body("Uptime: ...")
}

async fn collection() -> HttpResponse {
    // Implementation for collection
    HttpResponse::Ok().body("Collection: ...")
}

pub fn configure_status_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/status/uptime").to(uptime));
    cfg.service(web::resource("/status/collection").to(collection));
    // Add more configurations as needed for status routes
}
