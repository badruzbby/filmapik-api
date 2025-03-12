use crate::errors::AppError;
use crate::models::movie::{CountryResponse, MovieResponse, PaginationInfo};
use crate::scraper::FilmapikScraper;
use actix_web::{get, web, HttpResponse, Responder};
use log::info;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    page: u32,
}

fn default_page() -> u32 {
    1
}

// Jumlah film per halaman, biasanya 20 atau 24 film di FilmApik
const PER_PAGE: u32 = 24;

#[get("/movie/country")]
pub async fn get_countries() -> Result<impl Responder, AppError> {
    info!("Mendapatkan daftar negara film");
    
    let scraper = FilmapikScraper::new();
    let countries = scraper.get_countries().await?;
    
    let response = CountryResponse {
        status: "success".to_string(),
        message: "Daftar negara film berhasil didapatkan".to_string(),
        data: countries,
    };
    
    Ok(HttpResponse::Ok().json(response))
}

#[get("/movie/country/{country_id}")]
pub async fn get_movies_by_country(
    path: web::Path<String>,
    query: web::Query<PaginationParams>,
) -> Result<impl Responder, AppError> {
    let country_id = path.into_inner();
    let page = query.page;
    
    info!("Mendapatkan daftar film negara {} halaman {}", country_id, page);
    
    // Validasi country_id
    if country_id.is_empty() {
        return Err(AppError::ScrapingError("Country ID tidak valid".to_string()));
    }
    
    let scraper = FilmapikScraper::new();
    let movies = scraper.get_movies_by_country(&country_id, page).await?;
    
    // Buat informasi pagination
    let pagination = PaginationInfo {
        current_page: page,
        per_page: PER_PAGE,
        total_items: None,
        total_pages: None,
    };
    
    let response = MovieResponse {
        status: "success".to_string(),
        message: format!("Film negara {} halaman {} berhasil didapatkan", country_id, page),
        pagination,
        data: movies,
    };
    
    Ok(HttpResponse::Ok().json(response))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_countries)
       .service(get_movies_by_country);
} 