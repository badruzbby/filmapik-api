use crate::errors::AppError;
use crate::models::movie::{MovieResponse, PaginationInfo, GenreResponse, MovieDetailResponse, CountryResponse};
use crate::scraper::FilmapikScraper;
use actix_web::{get, web, HttpResponse, Responder, http};
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

#[get("/movie/latest")]
async fn get_latest_movies(query: web::Query<PaginationParams>) -> Result<impl Responder, AppError> {
    let page = query.page;
    
    info!("Mendapatkan daftar film terbaru halaman {}", page);
    
    let scraper = FilmapikScraper::new();
    let movies = scraper.get_latest_movies(page).await?;
    
    // Buat informasi pagination
    let pagination = PaginationInfo {
        current_page: page,
        per_page: PER_PAGE,
        total_items: None, // Sulit untuk mendapatkan total item dari scraping
        total_pages: None, // Sulit untuk mendapatkan total halaman dari scraping
    };
    
    let response = MovieResponse {
        status: "success".to_string(),
        message: format!("Film terbaru halaman {} berhasil didapatkan", page),
        pagination,
        data: movies,
    };
    
    Ok(HttpResponse::Ok().json(response))
}

#[get("/movie/genre")]
async fn get_genres() -> Result<impl Responder, AppError> {
    info!("Mendapatkan daftar genre film");
    
    let scraper = FilmapikScraper::new();
    let genres = scraper.get_genres().await?;
    
    let response = GenreResponse {
        status: "success".to_string(),
        message: "Daftar genre film berhasil didapatkan".to_string(),
        data: genres,
    };
    
    Ok(HttpResponse::Ok().json(response))
}

#[get("/movie/genre/{genre_id}")]
async fn get_movies_by_genre(
    path: web::Path<String>,
    query: web::Query<PaginationParams>,
) -> Result<impl Responder, AppError> {
    let genre_id = path.into_inner();
    let page = query.page;
    
    info!("Mendapatkan daftar film genre {} halaman {}", genre_id, page);
    
    // Validasi genre_id
    if genre_id.is_empty() {
        return Err(AppError::ScrapingError("Genre ID tidak valid".to_string()));
    }
    
    let scraper = FilmapikScraper::new();
    let movies = scraper.get_movies_by_genre(&genre_id, page).await?;
    
    // Buat informasi pagination
    let pagination = PaginationInfo {
        current_page: page,
        per_page: PER_PAGE,
        total_items: None,
        total_pages: None,
    };
    
    let response = MovieResponse {
        status: "success".to_string(),
        message: format!("Film genre {} halaman {} berhasil didapatkan", genre_id, page),
        pagination,
        data: movies,
    };
    
    Ok(HttpResponse::Ok().json(response))
}

#[get("/movie/popular")]
async fn get_popular_movies(query: web::Query<PaginationParams>) -> Result<impl Responder, AppError> {
    let page = query.page;
    
    info!("Mendapatkan daftar film populer (rating tertinggi) halaman {}", page);
    
    let scraper = FilmapikScraper::new();
    let movies = scraper.get_popular_movies(page).await?;
    
    // Buat informasi pagination
    let pagination = PaginationInfo {
        current_page: page,
        per_page: PER_PAGE,
        total_items: None, // Sulit untuk mendapatkan total item dari scraping
        total_pages: None, // Sulit untuk mendapatkan total halaman dari scraping
    };
    
    let response = MovieResponse {
        status: "success".to_string(),
        message: format!("Film populer (rating tertinggi) halaman {} berhasil didapatkan", page),
        pagination,
        data: movies,
    };
    
    Ok(HttpResponse::Ok().json(response))
}

#[get("/movie/country")]
async fn get_countries() -> Result<impl Responder, AppError> {
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

#[get("/movie/{movie_id}")]
async fn get_movie_detail(path: web::Path<String>) -> Result<impl Responder, AppError> {
    let movie_id = path.into_inner();
    
    info!("Mendapatkan detail film dengan ID: {}", movie_id);
    
    // Validasi movie_id
    if movie_id.is_empty() {
        return Err(AppError::ScrapingError("Movie ID tidak valid".to_string()));
    }
    
    let scraper = FilmapikScraper::new();
    let movie_detail = scraper.get_movie_detail(&movie_id).await?;
    
    let response = MovieDetailResponse {
        status: "success".to_string(),
        message: format!("Detail film {} berhasil didapatkan", movie_detail.title),
        data: movie_detail,
    };
    
    Ok(HttpResponse::Ok().json(response))
}

#[get("/movie/{movie_id}/watch")]
async fn get_movie_watch_url(path: web::Path<String>) -> Result<impl Responder, AppError> {
    let movie_id = path.into_inner();
    
    info!("Menampilkan halaman iframe untuk film dengan ID: {}", movie_id);
    
    // Validasi movie_id
    if movie_id.is_empty() {
        return Err(AppError::ScrapingError("Movie ID tidak valid".to_string()));
    }
    
    let scraper = FilmapikScraper::new();
    let movie_detail = scraper.get_movie_detail(&movie_id).await?;
    
    // Cek apakah watch_url tersedia
    if let Some(watch_url) = movie_detail.watch_url {
        info!("Membuat iframe untuk URL video: {}", watch_url);
        
        // Buat URL proxy untuk bypass CSP
        let proxy_url = format!("/api/movie/{}/watch/proxy", movie_id);
        
        // Buat HTML dengan iframe untuk menampilkan video melalui proxy
        let html = format!(r#"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} - FilmApik API</title>
    <style>
        body, html {{
            margin: 0;
            padding: 0;
            width: 100%;
            height: 100%;
            overflow: hidden;
            background-color: #000;
            font-family: Arial, sans-serif;
        }}
        .container {{
            display: flex;
            flex-direction: column;
            width: 100%;
            height: 100%;
        }}
        .video-container {{
            flex: 1;
            position: relative;
            width: 100%;
            height: 100%;
        }}
        iframe {{
            position: absolute;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            border: none;
        }}
        .info-bar {{
            padding: 10px;
            background-color: #222;
            color: white;
            text-align: center;
            font-size: 14px;
        }}
        .info-bar a {{
            color: #4CAF50;
            text-decoration: none;
        }}
        .info-bar a:hover {{
            text-decoration: underline;
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="video-container">
            <iframe src="{}" allowfullscreen></iframe>
        </div>
        <div class="info-bar">
            Menonton: <strong>{}</strong> | FilmApik API
        </div>
    </div>
</body>
</html>"#, movie_detail.title, proxy_url, movie_detail.title);
        
        // Kembalikan respons HTML
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html))
    } else {
        Err(AppError::ScrapingError("URL video tidak ditemukan".to_string()))
    }
}

#[get("/movie/{movie_id}/watch/proxy")]
async fn proxy_video_content(path: web::Path<String>) -> Result<impl Responder, AppError> {
    let movie_id = path.into_inner();
    
    info!("Memproxy konten video untuk film dengan ID: {}", movie_id);
    
    // Validasi movie_id
    if movie_id.is_empty() {
        return Err(AppError::ScrapingError("Movie ID tidak valid".to_string()));
    }
    
    let scraper = FilmapikScraper::new();
    let movie_detail = scraper.get_movie_detail(&movie_id).await?;
    
    // Cek apakah watch_url tersedia
    if let Some(watch_url) = movie_detail.watch_url {
        info!("Mengambil konten dari URL: {}", watch_url);
        
        // Membuat client HTTP untuk melakukan proxy request
        let client = reqwest::Client::new();
        
        // Mengirim permintaan ke URL asli dengan header yang lebih lengkap
        let response = client.get(&watch_url)
            .header("Accept-Language", "en-US,en;q=0.9")
            .header("Connection", "keep-alive")
            .header("Host", "filmdewasa.org")
            .header("Referer", "http://194.102.105.201/")
            .header("sec-ch-ua", "\"Chromium\";v=\"134\", \"Not:A-Brand\";v=\"24\", \"Microsoft Edge\";v=\"134\"")
            .header("sec-ch-ua-mobile", "?0")
            .header("sec-ch-ua-platform", "\"Windows\"")
            .header("Sec-Fetch-Dest", "iframe")
            .header("Sec-Fetch-Mode", "navigate")
            .header("Sec-Fetch-Site", "cross-site")
            .header("Sec-Fetch-Storage-Access", "active")
            .header("Upgrade-Insecure-Requests", "1")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36 Edg/134.0.0.0")
            .send()
            .await
            .map_err(|e| AppError::ScrapingError(format!("Gagal memproxy konten: {}", e)))?;
        
        // Membuat response builder
        let mut builder = HttpResponse::Ok();
        
        // Salin header yang relevan
        for (key, value) in response.headers().iter() {
            // Filter header yang tidak ingin diikutkan
            let key_str = key.as_str();
            if !key_str.eq_ignore_ascii_case("content-length") &&
               !key_str.eq_ignore_ascii_case("transfer-encoding") &&
               !key_str.eq_ignore_ascii_case("connection") &&
               !key_str.eq_ignore_ascii_case("content-encoding") &&
               !key_str.eq_ignore_ascii_case("content-security-policy") &&
               !key_str.eq_ignore_ascii_case("x-frame-options") {
                // Konversi header dari reqwest ke actix-web
                if let Ok(header_name) = http::header::HeaderName::from_bytes(key.as_str().as_bytes()) {
                    if let Ok(header_value) = http::header::HeaderValue::from_str(value.to_str().unwrap_or_default()) {
                        builder.append_header((header_name, header_value));
                    }
                }
            }
        }
        
        // Setel header untuk mengizinkan iframe
        builder.append_header((http::header::CONTENT_SECURITY_POLICY, "frame-ancestors *"));
        
        // Setel content-type jika tersedia
        if let Some(content_type) = response.headers().get("content-type") {
            if let Ok(content_type_str) = content_type.to_str() {
                builder.content_type(content_type_str);
            }
        }
        
        // Ambil body respons
        let body_bytes = response.bytes().await
            .map_err(|e| AppError::ScrapingError(format!("Gagal membaca body: {}", e)))?;
        
        // Kembalikan respons proxy
        Ok(builder.body(body_bytes))
    } else {
        Err(AppError::ScrapingError("URL video tidak ditemukan".to_string()))
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_latest_movies)
       .service(get_genres)
       .service(get_movies_by_genre)
       .service(get_popular_movies)
       .service(get_countries)
       .service(get_movie_detail)
       .service(get_movie_watch_url)
       .service(proxy_video_content);
} 