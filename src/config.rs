use lazy_static::lazy_static;
use std::env;
use std::sync::Mutex;

lazy_static! {
    pub static ref APP_HOST: String = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    pub static ref APP_PORT: u16 = env::var("APP_PORT").map(|p| p.parse().unwrap_or(8080)).unwrap_or(8080);
    pub static ref FILMAPIK_URL: String = env::var("FILMAPIK_URL").unwrap_or_else(|_| "http://194.102.105.201".to_string());
}

// Fungsi untuk inisialisasi konfigurasi
pub fn init() {
    // Membaca variabel lingkungan dari file .env jika ada
    dotenv::dotenv().ok();
    
    // Memastikan logger sudah diinisialisasi
    if env::var("RUST_LOG").is_err() {
        // Menggunakan blok unsafe karena set_var adalah fungsi unsafe
        unsafe {
            std::env::set_var("RUST_LOG", "info");
        }
    }
    env_logger::init();
} 