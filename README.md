# FilmApik API

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

> API tidak resmi untuk mengakses konten dari FilmApik

## 📖 Deskripsi

FilmApik API adalah sebuah REST API yang dibangun dengan Rust dan Actix Web untuk mengakses konten film dan serial TV dari situs FilmApik. API ini menggunakan teknik web scraping untuk mengambil data dari situs aslinya dan menyajikannya dalam format JSON yang mudah dikonsumsi oleh aplikasi lain.

## ✨ Fitur

- Pencarian film 
- Informasi detail film/serial termasuk sinopsis, tahun rilis, genre, dan lainnya
- Akses ke tautan streaming
- Daftar film/serial terbaru, populer, dan berdasarkan kategori
- Caching untuk meningkatkan performa
- Dokumentasi API dengan Swagger

## 🛠️ Teknologi yang Digunakan

- [Rust](https://www.rust-lang.org/) - Bahasa pemrograman
- [Actix Web](https://actix.rs/) - Framework web
- [Serde](https://serde.rs/) - Serialisasi/deserialisasi JSON
- [Reqwest](https://docs.rs/reqwest/) - HTTP client
- [Scraper](https://docs.rs/scraper/) - HTML parsing dan scraping
- [Moka](https://docs.rs/moka/) - In-memory caching

## 🚀 Memulai

### Prasyarat

- [Rust](https://www.rust-lang.org/tools/install) (versi terbaru)
- Cargo (terinstal bersama dengan Rust)

### Instalasi dan Menjalankan Secara Lokal

1. Kloning repositori
   ```bash
   git clone <repository-url> filmapik-api
   cd filmapik-api
   ```

2. Build dan jalankan aplikasi
   ```bash
   cargo run --release
   ```

3. API akan tersedia di `http://127.0.0.1:8080`

### Variabel Lingkungan

Aplikasi ini mendukung konfigurasi melalui variabel lingkungan:

- `APP_HOST`: Host yang digunakan aplikasi (default: 127.0.0.1)
- `APP_PORT`: Port yang digunakan aplikasi (default: 8080)
- `RUST_LOG`: Level logging (default: info)
- `FILMAPIK_URL`: URL untuk situs FilmApik (default: http://194.102.105.201)

Anda dapat mengatur variabel lingkungan dengan membuat file `.env` di direktori root atau mengaturnya saat menjalankan aplikasi:

```bash
APP_HOST=0.0.0.0 APP_PORT=9000 cargo run --release
```

## 🐳 Deployment dengan Docker

Lihat [README-docker.md](README-docker.md) untuk petunjuk lengkap tentang cara men-deploy aplikasi menggunakan Docker.

### Instalasi Cepat dengan Docker

```bash
# Build dan jalankan container
docker-compose up -d --build

# API akan tersedia di http://localhost:8080
```

## 📚 Dokumentasi API

Dokumentasi API tersedia dalam format Swagger dan dapat diakses melalui swagger.yaml di root proyek atau melalui endpoint:

```
http://localhost:8080/docs
```

### Endpoint Utama

Berikut adalah endpoint utama yang tersedia di API:

#### Film / Movie
- `GET /api/movie/latest` - Mendapatkan daftar film terbaru
- `GET /api/movie/popular` - Mendapatkan daftar film populer
- `GET /api/movie/genre` - Mendapatkan daftar genre film
- `GET /api/movie/genre/{genre}` - Mendapatkan daftar film berdasarkan genre
- `GET /api/movie/detail/{id}` - Mendapatkan detail film berdasarkan ID
- `GET /api/movie/watch/{id}` - Mendapatkan URL untuk menonton film

#### Negara / Country
- `GET /api/country` - Mendapatkan daftar negara

#### Cache
- `GET /api/cache/status` - Mendapatkan status cache
- `POST /api/cache/clear` - Membersihkan cache

Semua endpoint mendukung parameter paginasi `?page=1` (default: 1).

### Contoh Respons

```json
{
  "status": "success",
  "message": "Film terbaru halaman 1 berhasil didapatkan",
  "pagination": {
    "current_page": 1,
    "per_page": 24,
    "total_items": null,
    "total_pages": null
  },
  "data": [
    {
      "id": "example-id",
      "title": "Judul Film",
      "poster": "https://example.com/poster.jpg",
      "year": "2023",
      "type": "Movie"
    }
  ]
}
```

## 🧪 Pengujian

Untuk menjalankan test:

```bash
cargo test
```

## 📝 Lisensi

Proyek ini dilisensikan di bawah lisensi MIT - lihat file [LICENSE](LICENSE) untuk detail lebih lanjut.

## ⚠️ Disclaimer

API ini tidak terafiliasi dengan FilmApik secara resmi. Proyek ini dibuat untuk tujuan pendidikan dan pribadi. Harap gunakan dengan bijak dan sesuai dengan peraturan dan regulasi yang berlaku.

## 👨‍💻 Kontribusi

Kontribusi selalu diterima! Silakan buat issue atau pull request untuk perbaikan atau fitur baru.

1. Fork repositori
2. Buat branch fitur (`git checkout -b feature/amazing-feature`)
3. Commit perubahan Anda (`git commit -m 'Add some amazing feature'`)
4. Push ke branch (`git push origin feature/amazing-feature`)
5. Buka Pull Request 