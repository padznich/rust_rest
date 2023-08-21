use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;


pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health-check", web::get().to(health_check_handler));
}


pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("OK")
}


#[actix_rt::main]
async fn main() -> io::Result<()> {
    // Construct app and configure routes
    let app = move || App::new().configure(general_routes);

    // Start HTTP server
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
