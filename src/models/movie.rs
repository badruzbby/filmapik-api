use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Movie {
    pub id: String,
    pub title: String,
    pub poster: String,
    pub year: Option<i32>,
    pub rating: Option<f32>,
    pub quality: Option<String>,
    pub genres: Vec<String>,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationInfo {
    pub current_page: u32,
    pub per_page: u32,
    pub total_items: Option<u32>,
    pub total_pages: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MovieResponse {
    pub status: String,
    pub message: String,
    pub pagination: PaginationInfo,
    pub data: Vec<Movie>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Genre {
    pub id: String,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenreResponse {
    pub status: String,
    pub message: String,
    pub data: Vec<Genre>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MovieDetail {
    pub id: String,
    pub title: String,
    pub poster: String,
    pub rating: Option<f32>,
    pub rating_count: Option<u32>,
    pub description: Option<String>,
    pub views: Option<u32>,
    pub genres: Vec<String>,
    pub quality: Option<String>,
    pub year: Option<i32>,
    pub duration: Option<String>,
    pub country: Option<String>,
    pub release_date: Option<String>,
    pub language: Option<String>,
    pub director: Option<String>,
    pub actors: Vec<String>,
    pub url: String,
    pub watch_url: Option<String>,
    pub related_movies: Vec<Movie>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MovieDetailResponse {
    pub status: String,
    pub message: String,
    pub data: MovieDetail,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Country {
    pub id: String,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CountryResponse {
    pub status: String,
    pub message: String,
    pub data: Vec<Country>,
} 