use crate::errors::AppError;
use crate::scraper::FilmapikScraper;
use actix_web::{get, post, web, HttpResponse, Responder};
use log::info;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// Struktur respons untuk statistik cache
#[derive(Serialize)]
struct CacheStats {
    movies: u64,
    genres: u64,
    countries: u64,
    details: u64,
}

// Struktur respons umum untuk success
#[derive(Serialize)]
struct SuccessResponse {
    status: String,
    message: String,
}

// Struktur respons untuk statistik cache
#[derive(Serialize)]
struct CacheStatsResponse {
    status: String,
    message: String,
    data: CacheStats,
}

#[get("/cache/stats")]
async fn get_cache_stats() -> Result<impl Responder, AppError> {
    info!("Mendapatkan statistik cache");
    
    let scraper = FilmapikScraper::new();
    
    // Parse statistik cache dari string ke struct
    let stats_str = scraper.get_cache_stats();
    let parts: Vec<&str> = stats_str.split(", ").collect();
    
    let mut movies: u64 = 0;
    let mut genres: u64 = 0;
    let mut countries: u64 = 0;
    let mut details: u64 = 0;
    
    for part in parts {
        if part.contains("Movies:") {
            if let Some(val) = part.split(':').nth(1) {
                movies = val.trim().parse().unwrap_or(0);
            }
        } else if part.contains("Genres:") {
            if let Some(val) = part.split(':').nth(1) {
                genres = val.trim().parse().unwrap_or(0);
            }
        } else if part.contains("Countries:") {
            if let Some(val) = part.split(':').nth(1) {
                countries = val.trim().parse().unwrap_or(0);
            }
        } else if part.contains("Details:") {
            if let Some(val) = part.split(':').nth(1) {
                details = val.trim().parse().unwrap_or(0);
            }
        }
    }
    
    let cache_stats = CacheStats {
        movies,
        genres,
        countries,
        details,
    };
    
    let response = CacheStatsResponse {
        status: "success".to_string(),
        message: "Statistik cache berhasil didapatkan".to_string(),
        data: cache_stats,
    };
    
    Ok(HttpResponse::Ok().json(response))
}

#[post("/cache/clear")]
async fn clear_cache() -> Result<impl Responder, AppError> {
    info!("Menghapus semua cache");
    
    let scraper = FilmapikScraper::new();
    scraper.clear_cache().await;
    
    let response = SuccessResponse {
        status: "success".to_string(),
        message: "Cache berhasil dihapus".to_string(),
    };
    
    Ok(HttpResponse::Ok().json(response))
}

#[derive(Deserialize)]
struct CachePathParams {
    cache_type: String,
}

#[post("/cache/refresh/{cache_type}")]
async fn refresh_cache(path: web::Path<CachePathParams>) -> Result<impl Responder, AppError> {
    let cache_type = path.into_inner().cache_type;
    
    info!("Memperbarui cache untuk tipe: {}", cache_type);
    
    // Validasi tipe cache
    if cache_type != "genres" && cache_type != "countries" {
        return Err(AppError::ScrapingError(format!("Tipe cache tidak valid: {}", cache_type)));
    }
    
    let scraper = FilmapikScraper::new();
    scraper.refresh_cache(&cache_type).await?;
    
    let response = SuccessResponse {
        status: "success".to_string(),
        message: format!("Cache tipe {} berhasil diperbarui", cache_type),
    };
    
    Ok(HttpResponse::Ok().json(response))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_cache_stats)
       .service(clear_cache)
       .service(refresh_cache);
} 