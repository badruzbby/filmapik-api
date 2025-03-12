use actix_web::web;

pub mod movie;
pub mod country;
pub mod cache;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(movie::configure)
            .configure(country::configure)
            .configure(cache::configure)
    );
} 