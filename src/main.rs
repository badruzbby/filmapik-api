mod api;
mod config;
mod errors;
mod models;
mod scraper;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Inisialisasi konfigurasi
    config::init();
    
    let host = &*config::APP_HOST;
    let port = *config::APP_PORT;
    
    info!("Memulai server FilmApik API pada {}:{}", host, port);
    
    HttpServer::new(|| {
        // Konfigurasi CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .service(web::scope("").configure(api::configure))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
