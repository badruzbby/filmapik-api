use crate::config::FILMAPIK_URL;
use crate::errors::AppError;
use crate::models::movie::{Movie, Genre, MovieDetail, Country};
use anyhow::Result;
use log::{error, info};
use reqwest::{Client, header};
use scraper::{Html, Selector};
use chrono::NaiveDate;
use std::time::Duration;
use moka::future::Cache;
use std::sync::Arc;

// Mendefinisikan TTL (time-to-live) untuk berbagai jenis cache
const CACHE_TTL_MOVIES: u64 = 1800; // 30 menit untuk daftar film
const CACHE_TTL_GENRES: u64 = 86400; // 24 jam untuk genre (jarang berubah)
const CACHE_TTL_COUNTRIES: u64 = 86400; // 24 jam untuk negara (jarang berubah)
const CACHE_TTL_DETAILS: u64 = 3600; // 1 jam untuk detail film
const CACHE_MAX_CAPACITY: u64 = 1000; // Maksimum item dalam cache

// Definisikan key untuk cache
type CacheKey = String;

#[derive(Debug, Clone)]
pub struct FilmapikScraper {
    client: Client,
    // Cache untuk berbagai jenis data
    movies_cache: Arc<Cache<CacheKey, Vec<Movie>>>,
    genres_cache: Arc<Cache<CacheKey, Vec<Genre>>>,
    countries_cache: Arc<Cache<CacheKey, Vec<Country>>>,
    movie_detail_cache: Arc<Cache<CacheKey, MovieDetail>>,
}

impl FilmapikScraper {
    pub fn new() -> Self {
        // Membuat header map yang menyerupai browser
        let mut headers = header::HeaderMap::new();
        
        // User-Agent untuk Chrome di Windows
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        );
        
        // Accept header
        headers.insert(
            header::ACCEPT,
            header::HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7")
        );
        
        // Accept language (Bahasa Indonesia dan English)
        headers.insert(
            header::ACCEPT_LANGUAGE,
            header::HeaderValue::from_static("id,en-US;q=0.9,en;q=0.8")
        );
        
        // Cache control
        headers.insert(
            header::CACHE_CONTROL,
            header::HeaderValue::from_static("max-age=0")
        );
        
        // Connection type
        headers.insert(
            header::CONNECTION,
            header::HeaderValue::from_static("keep-alive")
        );
        
        // DNT (Do Not Track)
        headers.insert(
            header::HeaderName::from_static("dnt"),
            header::HeaderValue::from_static("1")
        );
        
        // Upgrade-Insecure-Requests
        headers.insert(
            header::HeaderName::from_static("upgrade-insecure-requests"),
            header::HeaderValue::from_static("1")
        );
        
        // Sec-Fetch-* headers
        headers.insert(
            header::HeaderName::from_static("sec-fetch-dest"),
            header::HeaderValue::from_static("document")
        );
        
        headers.insert(
            header::HeaderName::from_static("sec-fetch-mode"),
            header::HeaderValue::from_static("navigate")
        );
        
        headers.insert(
            header::HeaderName::from_static("sec-fetch-site"),
            header::HeaderValue::from_static("none")
        );
        
        headers.insert(
            header::HeaderName::from_static("sec-fetch-user"),
            header::HeaderValue::from_static("?1")
        );
        
        // Buat client dengan headers yang sudah dikonfigurasi
        let client = Client::builder()
            .default_headers(headers)
            .build()
            .unwrap_or_else(|_| Client::new());
        
        // Inisialisasi cache dengan TTL dan kapasitas
        let movies_cache = Cache::builder()
            .max_capacity(CACHE_MAX_CAPACITY)
            .time_to_live(Duration::from_secs(CACHE_TTL_MOVIES))
            .build();
            
        let genres_cache = Cache::builder()
            .max_capacity(CACHE_MAX_CAPACITY / 10) // Tidak perlu terlalu banyak untuk genre
            .time_to_live(Duration::from_secs(CACHE_TTL_GENRES))
            .build();
            
        let countries_cache = Cache::builder()
            .max_capacity(CACHE_MAX_CAPACITY / 10) // Tidak perlu terlalu banyak untuk negara
            .time_to_live(Duration::from_secs(CACHE_TTL_COUNTRIES))
            .build();
            
        let movie_detail_cache = Cache::builder()
            .max_capacity(CACHE_MAX_CAPACITY)
            .time_to_live(Duration::from_secs(CACHE_TTL_DETAILS))
            .build();
        
        Self { 
            client,
            movies_cache: Arc::new(movies_cache),
            genres_cache: Arc::new(genres_cache),
            countries_cache: Arc::new(countries_cache),
            movie_detail_cache: Arc::new(movie_detail_cache),
        }
    }

    pub async fn get_latest_movies(&self, page: u32) -> Result<Vec<Movie>, AppError> {
        info!("Mengambil daftar film terbaru dari FilmApik - Halaman {}", page);
        
        // Buat cache key berdasarkan tipe request dan halaman
        let cache_key = format!("latest_movies_page_{}", page);
        
        // Coba ambil dari cache terlebih dahulu
        if let Some(cached_movies) = self.movies_cache.get(&cache_key).await {
            info!("Menggunakan cache untuk film terbaru halaman {}", page);
            return Ok(cached_movies);
        }
        
        // Jika tidak ada di cache, ambil dari website
        // Ambil HTML dari website dengan parameter page
        let url = if page > 1 {
            format!("{}/page/{}/", *FILMAPIK_URL, page)
        } else {
            format!("{}/", *FILMAPIK_URL)
        };
        
        // Tambahkan referer untuk setiap request yang berbeda
        // Referer biasanya URL sebelumnya (halaman sebelumnya atau homepage)
        let referer = if page > 1 {
            format!("{}/page/{}/", *FILMAPIK_URL, page - 1)
        } else {
            (*FILMAPIK_URL).to_string()
        };
        
        // Buat request dengan referer yang dinamis
        let response = self.client.get(&url)
            .header(header::REFERER, referer)
            .send()
            .await
            .map_err(|e| {
                error!("Gagal melakukan request ke {}: {}", url, e);
                AppError::HttpError(e)
            })?;
        
        let status = response.status();
        if !status.is_success() {
            error!("Server mengembalikan status error: {}", status);
            return Err(AppError::ScrapingError(format!("Status HTTP error: {}", status)));
        }
        
        let html = response.text().await.map_err(|e| {
            error!("Gagal membaca response HTML: {}", e);
            AppError::HttpError(e)
        })?;
        
        // Parse data film
        let movies = self.parse_latest_movies(&html)?;
        
        // Simpan ke cache untuk penggunaan berikutnya
        self.movies_cache.insert(cache_key, movies.clone()).await;
        
        Ok(movies)
    }

    fn parse_latest_movies(&self, html: &str) -> Result<Vec<Movie>, AppError> {
        let document = Html::parse_document(html);
        
        // Selector untuk area film terbaru
        let latest_movies_selector = Selector::parse("#gmr-main-load article.item-infinite").map_err(|e| {
            error!("Gagal membuat selector: {}", e);
            AppError::ScrapingError(e.to_string())
        })?;
        
        let mut movies = Vec::new();
        
        for movie_element in document.select(&latest_movies_selector) {
            // Selectors untuk elemen film
            let title_selector = Selector::parse(".entry-title a").map_err(|e| {
                AppError::ScrapingError(e.to_string())
            })?;
            
            let poster_selector = Selector::parse(".content-thumbnail img").map_err(|e| {
                AppError::ScrapingError(e.to_string())
            })?;
            
            let quality_selector = Selector::parse(".gmr-quality-item a").map_err(|e| {
                AppError::ScrapingError(e.to_string())
            })?;
            
            let rating_selector = Selector::parse(".gmr-rating-item").map_err(|e| {
                AppError::ScrapingError(e.to_string())
            })?;
            
            let genre_selector = Selector::parse(".gmr-movie-on a").map_err(|e| {
                AppError::ScrapingError(e.to_string())
            })?;
            
            // Extract data
            if let Some(title_element) = movie_element.select(&title_selector).next() {
                let title = title_element.text().next().unwrap_or_default().trim().to_string();
                let url = title_element.value().attr("href").unwrap_or_default().to_string();
                let id = url.split('/').filter(|s| !s.is_empty()).last().unwrap_or_default().to_string();
                
                let poster = movie_element
                    .select(&poster_selector)
                    .next()
                    .and_then(|el| el.value().attr("src"))
                    .unwrap_or_default()
                    .to_string();
                
                // Ambil rating (opsional)
                let rating = movie_element
                    .select(&rating_selector)
                    .next()
                    .and_then(|el| el.text().next())
                    .and_then(|s| {
                        // Filter string untuk mendapatkan angka rating
                        let s = s.trim();
                        let s = s.trim_start_matches(|c| c == ' ' || c == '\t');
                        s.parse::<f32>().ok()
                    });
                
                // Ambil kualitas (opsional)
                let quality = movie_element
                    .select(&quality_selector)
                    .next()
                    .and_then(|el| el.text().next())
                    .map(|s| s.trim().to_string());
                
                // Ekstrak tahun dari judul film
                let year = if let Some(year_str) = title.split('(').nth(1) {
                    year_str
                        .trim_end_matches(')')
                        .parse::<i32>()
                        .ok()
                } else {
                    None
                };
                
                // Ambil genre
                let mut genres = Vec::new();
                for genre_element in movie_element.select(&genre_selector) {
                    if let Some(genre_text) = genre_element.text().next() {
                        genres.push(genre_text.trim().to_string());
                    }
                }
                
                let movie = Movie {
                    id,
                    title,
                    poster,
                    year,
                    rating,
                    quality,
                    genres,
                    url,
                };
                
                movies.push(movie);
            }
        }
        
        Ok(movies)
    }

    pub async fn get_genres(&self) -> Result<Vec<Genre>, AppError> {
        info!("Mendapatkan daftar genre dari FilmApik");
        
        // Buat cache key untuk genres
        let cache_key = "all_genres".to_string();
        
        // Coba ambil dari cache terlebih dahulu
        if let Some(cached_genres) = self.genres_cache.get(&cache_key).await {
            info!("Menggunakan cache untuk daftar genre");
            return Ok(cached_genres);
        }
        
        // Daftar genre yang tersedia di FilmApik (secara statis)
        let genres = vec![
            Genre {
                id: "action".to_string(),
                name: "Action".to_string(),
                url: format!("{}/genre/action/", *FILMAPIK_URL),
            },
            Genre {
                id: "adventure".to_string(),
                name: "Adventure".to_string(),
                url: format!("{}/genre/adventure/", *FILMAPIK_URL),
            },
            Genre {
                id: "animation".to_string(),
                name: "Animation".to_string(),
                url: format!("{}/genre/animation/", *FILMAPIK_URL),
            },
            Genre {
                id: "comedy".to_string(),
                name: "Comedy".to_string(),
                url: format!("{}/genre/comedy/", *FILMAPIK_URL),
            },
            Genre {
                id: "crime".to_string(),
                name: "Crime".to_string(),
                url: format!("{}/genre/crime/", *FILMAPIK_URL),
            },
            Genre {
                id: "documentary".to_string(),
                name: "Documentary".to_string(),
                url: format!("{}/genre/documentary/", *FILMAPIK_URL),
            },
            Genre {
                id: "drama".to_string(),
                name: "Drama".to_string(),
                url: format!("{}/genre/drama/", *FILMAPIK_URL),
            },
            Genre {
                id: "family".to_string(),
                name: "Family".to_string(),
                url: format!("{}/genre/family/", *FILMAPIK_URL),
            },
            Genre {
                id: "fantasy".to_string(),
                name: "Fantasy".to_string(),
                url: format!("{}/genre/fantasy/", *FILMAPIK_URL),
            },
            Genre {
                id: "history".to_string(),
                name: "History".to_string(),
                url: format!("{}/genre/history/", *FILMAPIK_URL),
            },
            Genre {
                id: "horror".to_string(),
                name: "Horror".to_string(),
                url: format!("{}/genre/horror/", *FILMAPIK_URL),
            },
            Genre {
                id: "music".to_string(),
                name: "Music".to_string(),
                url: format!("{}/genre/music/", *FILMAPIK_URL),
            },
            Genre {
                id: "mystery".to_string(),
                name: "Mystery".to_string(),
                url: format!("{}/genre/mystery/", *FILMAPIK_URL),
            },
            Genre {
                id: "romance".to_string(),
                name: "Romance".to_string(),
                url: format!("{}/genre/romance/", *FILMAPIK_URL),
            },
            Genre {
                id: "science-fiction".to_string(),
                name: "Science Fiction".to_string(),
                url: format!("{}/genre/science-fiction/", *FILMAPIK_URL),
            },
            Genre {
                id: "thriller".to_string(),
                name: "Thriller".to_string(),
                url: format!("{}/genre/thriller/", *FILMAPIK_URL),
            },
            Genre {
                id: "war".to_string(),
                name: "War".to_string(),
                url: format!("{}/genre/war/", *FILMAPIK_URL),
            },
        ];
        
        // Simpan ke cache untuk penggunaan berikutnya
        self.genres_cache.insert(cache_key, genres.clone()).await;
        
        Ok(genres)
    }

    pub async fn get_movies_by_genre(&self, genre_id: &str, page: u32) -> Result<Vec<Movie>, AppError> {
        info!("Mengambil daftar film genre {} halaman {}", genre_id, page);
        
        // Buat cache key berdasarkan genre dan halaman
        let cache_key = format!("genre_{}_page_{}", genre_id, page);
        
        // Coba ambil dari cache terlebih dahulu
        if let Some(cached_movies) = self.movies_cache.get(&cache_key).await {
            info!("Menggunakan cache untuk film genre {} halaman {}", genre_id, page);
            return Ok(cached_movies);
        }
        
        // URL untuk halaman genre dengan pagination
        let url = if page > 1 {
            format!("{}/genre/{}/page/{}/", *FILMAPIK_URL, genre_id, page)
        } else {
            format!("{}/genre/{}/", *FILMAPIK_URL, genre_id)
        };
        
        // Tambahkan referer untuk setiap request
        let referer = if page > 1 {
            format!("{}/genre/{}/page/{}/", *FILMAPIK_URL, genre_id, page - 1)
        } else {
            format!("{}/", *FILMAPIK_URL)
        };
        
        // Membuat request dengan referer yang dinamis
        let response = self.client.get(&url)
            .header(header::REFERER, referer)
            .send()
            .await
            .map_err(|e| {
                error!("Gagal melakukan request ke {}: {}", url, e);
                AppError::HttpError(e)
            })?;
        
        let status = response.status();
        if !status.is_success() {
            error!("Server mengembalikan status error: {}", status);
            return Err(AppError::ScrapingError(format!("Status HTTP error: {}", status)));
        }
        
        let html = response.text().await.map_err(|e| {
            error!("Gagal membaca response HTML: {}", e);
            AppError::HttpError(e)
        })?;
        
        // Gunakan fungsi parse_latest_movies yang sudah ada karena struktur HTML-nya sama
        let movies = self.parse_latest_movies(&html)?;
        
        // Simpan ke cache untuk penggunaan berikutnya
        self.movies_cache.insert(cache_key, movies.clone()).await;
        
        Ok(movies)
    }

    pub async fn get_popular_movies(&self, page: u32) -> Result<Vec<Movie>, AppError> {
        info!("Mengambil daftar film populer (rating tertinggi) dari FilmApik - Halaman {}", page);
        
        // Buat cache key berdasarkan tipe request dan halaman
        let cache_key = format!("popular_movies_page_{}", page);
        
        // Coba ambil dari cache terlebih dahulu
        if let Some(cached_movies) = self.movies_cache.get(&cache_key).await {
            info!("Menggunakan cache untuk film populer halaman {}", page);
            return Ok(cached_movies);
        }
        
        // URL untuk halaman best-rating dengan pagination
        let url = if page > 1 {
            format!("{}/best-rating/page/{}/", *FILMAPIK_URL, page)
        } else {
            format!("{}/best-rating/", *FILMAPIK_URL)
        };
        
        // Tambahkan referer untuk setiap request yang berbeda
        let referer = if page > 1 {
            format!("{}/best-rating/page/{}/", *FILMAPIK_URL, page - 1)
        } else {
            format!("{}/", *FILMAPIK_URL)
        };
        
        // Buat request dengan referer yang dinamis
        let response = self.client.get(&url)
            .header(header::REFERER, referer)
            .send()
            .await
            .map_err(|e| {
                error!("Gagal melakukan request ke {}: {}", url, e);
                AppError::HttpError(e)
            })?;
        
        let status = response.status();
        if !status.is_success() {
            error!("Server mengembalikan status error: {}", status);
            return Err(AppError::ScrapingError(format!("Status HTTP error: {}", status)));
        }
        
        let html = response.text().await.map_err(|e| {
            error!("Gagal membaca response HTML: {}", e);
            AppError::HttpError(e)
        })?;
        
        // Gunakan fungsi parse_latest_movies yang sudah ada karena struktur HTML-nya sama
        let movies = self.parse_latest_movies(&html)?;
        
        // Simpan ke cache untuk penggunaan berikutnya
        self.movies_cache.insert(cache_key, movies.clone()).await;
        
        Ok(movies)
    }

    pub async fn get_movie_detail(&self, movie_id: &str) -> Result<MovieDetail, AppError> {
        info!("Mengambil detail film dengan ID: {}", movie_id);
        
        // Buat cache key berdasarkan movie_id
        let cache_key = format!("movie_detail_{}", movie_id);
        
        // Coba ambil dari cache terlebih dahulu
        if let Some(cached_detail) = self.movie_detail_cache.get(&cache_key).await {
            info!("Menggunakan cache untuk detail film {}", movie_id);
            return Ok(cached_detail);
        }
        
        // Buat URL untuk halaman detail film
        let url = format!("{}/{}/", *FILMAPIK_URL, movie_id);
        
        // Buat referrer
        let referer = format!("{}/", *FILMAPIK_URL);
        
        // Lakukan request
        let response = self.client.get(&url)
            .header(header::REFERER, referer)
            .send()
            .await
            .map_err(|e| {
                error!("Gagal melakukan request ke {}: {}", url, e);
                AppError::HttpError(e)
            })?;
        
        let status = response.status();
        if !status.is_success() {
            error!("Server mengembalikan status error: {}", status);
            return Err(AppError::ScrapingError(format!("Status HTTP error: {}", status)));
        }
        
        let html = response.text().await.map_err(|e| {
            error!("Gagal membaca response HTML: {}", e);
            AppError::HttpError(e)
        })?;
        
        // Parse detail film
        let movie_detail = self.parse_movie_detail(&html, movie_id, &url)?;
        
        // Simpan ke cache untuk penggunaan berikutnya
        self.movie_detail_cache.insert(cache_key, movie_detail.clone()).await;
        
        Ok(movie_detail)
    }

    fn parse_movie_detail(&self, html: &str, movie_id: &str, url: &str) -> Result<MovieDetail, AppError> {
        let document = Html::parse_document(html);
        
        // Selector untuk berbagai elemen
        let title_selector = Selector::parse(".entry-title").map_err(|e| {
            AppError::ScrapingError(e.to_string())
        })?;
        
        let poster_selector = Selector::parse(".gmr-movie-data img").map_err(|e| {
            AppError::ScrapingError(e.to_string())
        })?;
        
        let desc_selector = Selector::parse(".gmr-movie-content p").map_err(|e| {
            AppError::ScrapingError(e.to_string())
        })?;
        
        // Perbaikan selector untuk rating
        let rating_selector = Selector::parse("div.gmr-meta-rating > span[itemprop=\"ratingValue\"]").map_err(|e| {
            AppError::ScrapingError(e.to_string())
        })?;
        
        let meta_info_selector = Selector::parse(".gmr-moviedata").map_err(|e| {
            AppError::ScrapingError(e.to_string())
        })?;
        
        let genre_selector = Selector::parse(".gmr-movie-on a").map_err(|e| {
            AppError::ScrapingError(e.to_string())
        })?;
        
        // Selector untuk iframe video
        let iframe_selector = Selector::parse("iframe").map_err(|e| {
            AppError::ScrapingError(e.to_string())
        })?;
        
        // Selectors untuk film terkait/rekomendasi
        let related_selector = Selector::parse(".gmr-related-movie .row .item-related").map_err(|e| {
            AppError::ScrapingError(e.to_string())
        })?;
        
        // Extract title
        let title = document
            .select(&title_selector)
            .next()
            .and_then(|el| el.text().next())
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "Unknown Title".to_string());
        
        // Extract poster
        let poster = document
            .select(&poster_selector)
            .next()
            .and_then(|el| el.value().attr("src"))
            .map(|s| s.to_string())
            .unwrap_or_default();
        
        // Extract description
        let description = document
            .select(&desc_selector)
            .next()
            .map(|el| {
                let desc_text = el.text().collect::<Vec<_>>().join(" ");
                desc_text.trim().to_string()
            });
        
        // Extract rating (dengan selector yang sudah diperbaiki)
        let rating = document
            .select(&rating_selector)
            .next()
            .and_then(|el| el.text().next())
            .and_then(|s| {
                s.trim().parse::<f32>().ok()
            });
        
        // Extract watch URL dari iframe
        let watch_url = document
            .select(&iframe_selector)
            .next()
            .and_then(|el| el.value().attr("src"))
            .map(|s| s.to_string());
        
        // Default values
        let mut rating_count: Option<u32> = None;
        let mut quality: Option<String> = None;
        let mut year: Option<i32> = None;
        let mut duration: Option<String> = None;
        let mut country: Option<String> = None;
        let mut release_date: Option<String> = None;
        let mut language: Option<String> = None;
        let mut director: Option<String> = None;
        let mut views: Option<u32> = None;
        
        // Extract metadata information
        for meta_element in document.select(&meta_info_selector) {
            let meta_text = meta_element.text().collect::<Vec<_>>().join(" ");
            let meta_text = meta_text.trim();
            
            if meta_text.starts_with("Diposting") && meta_text.contains("dilihat") {
                // Extract views
                if let Some(views_str) = meta_text.split("dilihat").nth(1) {
                    views = views_str
                        .trim()
                        .replace(",", "")
                        .parse::<u32>()
                        .ok();
                }
            } else if meta_text.starts_with("Kualitas:") {
                quality = meta_text.split(':').nth(1).map(|s| s.trim().to_string());
            } else if meta_text.starts_with("Tahun:") {
                year = meta_text
                    .split(':')
                    .nth(1)
                    .and_then(|s| s.trim().parse::<i32>().ok());
            } else if meta_text.starts_with("Durasi:") {
                duration = meta_text.split(':').nth(1).map(|s| s.trim().to_string());
            } else if meta_text.starts_with("Negara:") {
                country = meta_text.split(':').nth(1).map(|s| s.trim().to_string());
            } else if meta_text.starts_with("Rilis:") {
                release_date = meta_text.split(':').nth(1).map(|s| s.trim().to_string());
            } else if meta_text.starts_with("Bahasa:") {
                language = meta_text.split(':').nth(1).map(|s| s.trim().to_string());
            } else if meta_text.starts_with("Direksi:") {
                director = meta_text.split(':').nth(1).map(|s| s.trim().to_string());
            }
        }
        
        // Extract genres
        let mut genres = Vec::new();
        for genre_element in document.select(&genre_selector) {
            if let Some(genre_text) = genre_element.text().next() {
                let genre_text = genre_text.trim();
                if !genre_text.is_empty() {
                    genres.push(genre_text.to_string());
                }
            }
        }
        
        // Extract actors
        let mut actors = Vec::new();
        let actors_selector = Selector::parse(".gmr-castcrew li").ok();
        if let Some(selector) = actors_selector {
            for actor_element in document.select(&selector) {
                if let Some(actor_text) = actor_element.text().next() {
                    let actor_text = actor_text.trim();
                    if !actor_text.is_empty() {
                        actors.push(actor_text.to_string());
                    }
                }
            }
        }
        
        // Extract related movies
        let mut related_movies = Vec::new();
        for related_element in document.select(&related_selector) {
            let title_selector = Selector::parse("h3 a").ok();
            let poster_selector = Selector::parse("img").ok();
            
            if let (Some(title_sel), Some(poster_sel)) = (title_selector, poster_selector) {
                let related_title = related_element
                    .select(&title_sel)
                    .next()
                    .and_then(|el| el.text().next())
                    .map(|s| s.trim().to_string());
                
                let related_url = related_element
                    .select(&title_sel)
                    .next()
                    .and_then(|el| el.value().attr("href"))
                    .map(|s| s.to_string());
                
                let related_poster = related_element
                    .select(&poster_sel)
                    .next()
                    .and_then(|el| el.value().attr("src"))
                    .map(|s| s.to_string());
                
                if let (Some(title), Some(url)) = (related_title, related_url) {
                    let id = url.split('/').filter(|s| !s.is_empty()).last().unwrap_or_default().to_string();
                    let movie = Movie {
                        id,
                        title,
                        poster: related_poster.unwrap_or_default(),
                        year: None,
                        rating: None,
                        quality: None,
                        genres: Vec::new(),
                        url,
                    };
                    related_movies.push(movie);
                }
            }
        }
        
        let movie_detail = MovieDetail {
            id: movie_id.to_string(),
            title,
            poster,
            rating,
            rating_count,
            description,
            views,
            genres,
            quality,
            year,
            duration,
            country,
            release_date,
            language,
            director,
            actors,
            url: url.to_string(),
            watch_url,
            related_movies,
        };
        
        Ok(movie_detail)
    }

    pub async fn get_countries(&self) -> Result<Vec<Country>, AppError> {
        info!("Mendapatkan daftar negara dari FilmApik");
        
        // Buat cache key untuk countries
        let cache_key = "all_countries".to_string();
        
        // Coba ambil dari cache terlebih dahulu
        if let Some(cached_countries) = self.countries_cache.get(&cache_key).await {
            info!("Menggunakan cache untuk daftar negara");
            return Ok(cached_countries);
        }
        
        // Daftar negara yang tersedia di FilmApik (secara statis)
        // Data diambil dari film-terbaru.html dan ditambahkan negara lainnya yang populer
        let countries = vec![
            Country {
                id: "usa".to_string(),
                name: "USA".to_string(),
                url: format!("{}/country/usa/", *FILMAPIK_URL),
            },
            Country {
                id: "india".to_string(),
                name: "India".to_string(),
                url: format!("{}/country/india/", *FILMAPIK_URL),
            },
            Country {
                id: "china".to_string(),
                name: "China".to_string(),
                url: format!("{}/country/china/", *FILMAPIK_URL),
            },
            Country {
                id: "korea".to_string(),
                name: "Korea".to_string(),
                url: format!("{}/country/korea/", *FILMAPIK_URL),
            },
            Country {
                id: "philippines".to_string(),
                name: "Philippines".to_string(),
                url: format!("{}/country/philippines/", *FILMAPIK_URL),
            },
            // Tambahan negara-negara lainnya
            Country {
                id: "japan".to_string(),
                name: "Japan".to_string(),
                url: format!("{}/country/japan/", *FILMAPIK_URL),
            },
            Country {
                id: "thailand".to_string(),
                name: "Thailand".to_string(),
                url: format!("{}/country/thailand/", *FILMAPIK_URL),
            },
            Country {
                id: "indonesia".to_string(),
                name: "Indonesia".to_string(),
                url: format!("{}/country/indonesia/", *FILMAPIK_URL),
            },
            Country {
                id: "malaysia".to_string(),
                name: "Malaysia".to_string(),
                url: format!("{}/country/malaysia/", *FILMAPIK_URL),
            },
            Country {
                id: "france".to_string(),
                name: "France".to_string(),
                url: format!("{}/country/france/", *FILMAPIK_URL),
            },
            Country {
                id: "germany".to_string(),
                name: "Germany".to_string(),
                url: format!("{}/country/germany/", *FILMAPIK_URL),
            },
            Country {
                id: "uk".to_string(),
                name: "United Kingdom".to_string(),
                url: format!("{}/country/uk/", *FILMAPIK_URL),
            },
            Country {
                id: "italy".to_string(),
                name: "Italy".to_string(),
                url: format!("{}/country/italy/", *FILMAPIK_URL),
            },
            Country {
                id: "spain".to_string(),
                name: "Spain".to_string(),
                url: format!("{}/country/spain/", *FILMAPIK_URL),
            },
            Country {
                id: "russia".to_string(),
                name: "Russia".to_string(),
                url: format!("{}/country/russia/", *FILMAPIK_URL),
            },
            Country {
                id: "australia".to_string(),
                name: "Australia".to_string(),
                url: format!("{}/country/australia/", *FILMAPIK_URL),
            },
            Country {
                id: "canada".to_string(),
                name: "Canada".to_string(),
                url: format!("{}/country/canada/", *FILMAPIK_URL),
            },
            Country {
                id: "brazil".to_string(),
                name: "Brazil".to_string(),
                url: format!("{}/country/brazil/", *FILMAPIK_URL),
            },
            Country {
                id: "mexico".to_string(),
                name: "Mexico".to_string(),
                url: format!("{}/country/mexico/", *FILMAPIK_URL),
            },
            Country {
                id: "taiwan".to_string(),
                name: "Taiwan".to_string(),
                url: format!("{}/country/taiwan/", *FILMAPIK_URL),
            },
            Country {
                id: "hongkong".to_string(),
                name: "Hong Kong".to_string(),
                url: format!("{}/country/hongkong/", *FILMAPIK_URL),
            },
            Country {
                id: "vietnam".to_string(),
                name: "Vietnam".to_string(),
                url: format!("{}/country/vietnam/", *FILMAPIK_URL),
            },
            Country {
                id: "turkey".to_string(),
                name: "Turkey".to_string(),
                url: format!("{}/country/turkey/", *FILMAPIK_URL),
            },
            Country {
                id: "singapore".to_string(),
                name: "Singapore".to_string(),
                url: format!("{}/country/singapore/", *FILMAPIK_URL),
            },
        ];
        
        // Simpan ke cache untuk penggunaan berikutnya
        self.countries_cache.insert(cache_key, countries.clone()).await;
        
        Ok(countries)
    }

    pub async fn get_movies_by_country(&self, country_id: &str, page: u32) -> Result<Vec<Movie>, AppError> {
        info!("Mengambil daftar film negara {} halaman {}", country_id, page);
        
        // Buat cache key berdasarkan country dan halaman
        let cache_key = format!("country_{}_page_{}", country_id, page);
        
        // Coba ambil dari cache terlebih dahulu
        if let Some(cached_movies) = self.movies_cache.get(&cache_key).await {
            info!("Menggunakan cache untuk film negara {} halaman {}", country_id, page);
            return Ok(cached_movies);
        }
        
        // URL untuk halaman negara dengan pagination
        let url = if page > 1 {
            format!("{}/country/{}/page/{}/", *FILMAPIK_URL, country_id, page)
        } else {
            format!("{}/country/{}/", *FILMAPIK_URL, country_id)
        };
        
        // Tambahkan referer untuk setiap request
        let referer = if page > 1 {
            format!("{}/country/{}/page/{}/", *FILMAPIK_URL, country_id, page - 1)
        } else {
            format!("{}/", *FILMAPIK_URL)
        };
        
        // Membuat request dengan referer yang dinamis
        let response = self.client.get(&url)
            .header(header::REFERER, referer)
            .send()
            .await
            .map_err(|e| {
                error!("Gagal melakukan request ke {}: {}", url, e);
                AppError::HttpError(e)
            })?;
        
        let status = response.status();
        if !status.is_success() {
            error!("Server mengembalikan status error: {}", status);
            return Err(AppError::ScrapingError(format!("Status HTTP error: {}", status)));
        }
        
        let html = response.text().await.map_err(|e| {
            error!("Gagal membaca response HTML: {}", e);
            AppError::HttpError(e)
        })?;
        
        // Gunakan fungsi parse_latest_movies yang sudah ada karena struktur HTML-nya sama
        let movies = self.parse_latest_movies(&html)?;
        
        // Simpan ke cache untuk penggunaan berikutnya
        self.movies_cache.insert(cache_key, movies.clone()).await;
        
        Ok(movies)
    }
    
    // Method untuk menghapus cache secara manual jika diperlukan
    pub async fn clear_cache(&self) {
        info!("Menghapus semua cache");
        self.movies_cache.invalidate_all();
        self.genres_cache.invalidate_all();
        self.countries_cache.invalidate_all();
        self.movie_detail_cache.invalidate_all();
    }
    
    // Method untuk memperbarui cache tertentu secara manual
    pub async fn refresh_cache(&self, cache_type: &str) -> Result<(), AppError> {
        match cache_type {
            "genres" => {
                info!("Memperbarui cache untuk genre");
                let genres = self.get_genres().await?;
                self.genres_cache.insert("all_genres".to_string(), genres).await;
            },
            "countries" => {
                info!("Memperbarui cache untuk negara");
                let countries = self.get_countries().await?;
                self.countries_cache.insert("all_countries".to_string(), countries).await;
            },
            _ => {
                return Err(AppError::ScrapingError(format!("Tipe cache tidak dikenali: {}", cache_type)));
            }
        }
        
        Ok(())
    }
    
    // Method untuk mendapatkan statistik cache
    pub fn get_cache_stats(&self) -> String {
        let movies_stats = self.movies_cache.entry_count();
        let genres_stats = self.genres_cache.entry_count();
        let countries_stats = self.countries_cache.entry_count();
        let details_stats = self.movie_detail_cache.entry_count();
        
        format!(
            "Cache Stats: Movies: {}, Genres: {}, Countries: {}, Details: {}",
            movies_stats, genres_stats, countries_stats, details_stats
        )
    }
} 