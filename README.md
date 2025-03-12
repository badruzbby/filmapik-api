# FilmApik API

REST API untuk mendapatkan informasi film dari FilmApik menggunakan Rust.

## Fitur

- Mendapatkan daftar film terbaru via endpoint `/api/movie/latest`
- Mendapatkan daftar film populer (rating tertinggi) via endpoint `/api/movie/popular`
- Mendapatkan daftar genre film via endpoint `/api/movie/genre`
- Mendapatkan film berdasarkan genre via endpoint `/api/movie/genre/{genre_id}`
- Mendapatkan daftar negara asal film via endpoint `/api/movie/country`
- Mendapatkan film berdasarkan negara via endpoint `/api/movie/country/{country_id}`
- Mendapatkan detail film via endpoint `/api/movie/{movie_id}`
- Menonton film via iframe pada endpoint `/api/movie/{movie_id}/watch`
- **Reverse Proxy** untuk bypass Content Security Policy pada iframe video
- **Pagination** untuk mengakses halaman film yang berbeda
- **Browser Headers** untuk scraping yang otentik dan menghindari pemblokiran
- **Caching System** untuk respons API yang lebih cepat dan mengurangi beban server
- Menggunakan scraping dari website FilmApik
- Struktur kode yang mengikuti best practice Rust

## Sistem Caching

API dilengkapi dengan mekanisme caching yang efisien untuk meningkatkan performa:

- **Film Terbaru dan Populer**: Di-cache selama 30 menit
- **Detail Film**: Di-cache selama 1 jam
- **Genre dan Negara**: Di-cache selama 24 jam (data yang jarang berubah)
- Kapasitas cache yang dapat dikonfigurasi (default: 1000 item)
- Pembersihan cache otomatis berdasarkan waktu

Manfaat caching:
- Mengurangi waktu respons API secara signifikan
- Mengurangi beban pada server FilmApik
- Menghindari pemblokiran IP karena terlalu banyak request
- Menghemat bandwidth dan sumber daya server

## Cara Menjalankan

### Prasyarat

- Rust dan Cargo harus terinstal di sistem Anda
- (Opsional) File `.env` untuk konfigurasi

### Langkah-langkah

1. Clone repositori ini
2. Jalankan aplikasi:

```bash
cargo run
```

Aplikasi akan berjalan pada http://127.0.0.1:8080 secara default.

### Konfigurasi

Anda dapat mengkonfigurasi aplikasi menggunakan variabel lingkungan atau file `.env`:

- `APP_HOST` - Host tempat server berjalan (default: 127.0.0.1)
- `APP_PORT` - Port tempat server berjalan (default: 8080)
- `FILMAPIK_URL` - URL dari website FilmApik (default: http://194.102.105.201)

## Endpoint API

### Get Latest Movies (dengan Pagination)

```
GET /api/movie/latest
GET /api/movie/latest?page=2
```

Parameter Query:
- `page`: Nomor halaman yang ingin diambil (default: 1)

**Response:**

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
      "id": "...",
      "title": "...",
      "poster": "...",
      "year": 2023,
      "rating": 7.5,
      "quality": "HD",
      "genres": ["Action", "Thriller"],
      "url": "..."
    },
    // ...film lainnya
  ]
}
```

### Get Popular Movies (dengan Pagination)

```
GET /api/movie/popular
GET /api/movie/popular?page=2
```

Parameter Query:
- `page`: Nomor halaman yang ingin diambil (default: 1)

**Response:**

```json
{
  "status": "success",
  "message": "Film populer (rating tertinggi) halaman 1 berhasil didapatkan",
  "pagination": {
    "current_page": 1,
    "per_page": 24,
    "total_items": null,
    "total_pages": null
  },
  "data": [
    {
      "id": "...",
      "title": "...",
      "poster": "...",
      "year": 2023,
      "rating": 10.0,
      "quality": "HD",
      "genres": ["Action", "Drama"],
      "url": "..."
    },
    // ...film lainnya
  ]
}
```

### Get Movie Genres

```
GET /api/movie/genre
```

**Response:**

```json
{
  "status": "success",
  "message": "Daftar genre film berhasil didapatkan",
  "data": [
    {
      "id": "action",
      "name": "Action",
      "url": "http://194.102.105.201/genre/action/"
    },
    {
      "id": "adventure",
      "name": "Adventure",
      "url": "http://194.102.105.201/genre/adventure/"
    },
    // ...genre lainnya
  ]
}
```

### Get Movies by Genre (dengan Pagination)

```
GET /api/movie/genre/action
GET /api/movie/genre/comedy
GET /api/movie/genre/action?page=2
```

Parameter Path:
- `genre_id`: ID genre film (contoh: "action", "comedy", "horror", dll)

Parameter Query:
- `page`: Nomor halaman yang ingin diambil (default: 1)

**Response:**

```json
{
  "status": "success",
  "message": "Film genre action halaman 1 berhasil didapatkan",
  "pagination": {
    "current_page": 1,
    "per_page": 24,
    "total_items": null,
    "total_pages": null
  },
  "data": [
    {
      "id": "...",
      "title": "...",
      "poster": "...",
      "year": 2023,
      "rating": 7.5,
      "quality": "HD",
      "genres": ["Action", "Thriller"],
      "url": "..."
    },
    // ...film lainnya
  ]
}
```

### Get Movie Countries

```
GET /api/movie/country
```

**Response:**

```json
{
  "status": "success",
  "message": "Daftar negara film berhasil didapatkan",
  "data": [
    {
      "id": "usa",
      "name": "USA",
      "url": "http://194.102.105.201/country/usa/"
    },
    {
      "id": "india",
      "name": "India",
      "url": "http://194.102.105.201/country/india/"
    },
    {
      "id": "china",
      "name": "China",
      "url": "http://194.102.105.201/country/china/"
    },
    {
      "id": "korea",
      "name": "Korea",
      "url": "http://194.102.105.201/country/korea/"
    },
    {
      "id": "japan",
      "name": "Japan",
      "url": "http://194.102.105.201/country/japan/"
    },
    // ... banyak negara lainnya
  ]
}
```

### Get Movies by Country (dengan Pagination)

```
GET /api/movie/country/usa
GET /api/movie/country/korea
GET /api/movie/country/india?page=2
```

Parameter Path:
- `country_id`: ID negara film (contoh: "usa", "korea", "india", dll)

Parameter Query:
- `page`: Nomor halaman yang ingin diambil (default: 1)

**Response:**

```json
{
  "status": "success",
  "message": "Film negara usa halaman 1 berhasil didapatkan",
  "pagination": {
    "current_page": 1,
    "per_page": 24,
    "total_items": null,
    "total_pages": null
  },
  "data": [
    {
      "id": "...",
      "title": "...",
      "poster": "...",
      "year": 2023,
      "rating": 7.5,
      "quality": "HD",
      "genres": ["Action", "Thriller"],
      "url": "..."
    },
    // ...film lainnya
  ]
}
```

### Get Movie Detail

```
GET /api/movie/paayum-oli-nee-yenakku-2023
GET /api/movie/guardians-of-the-galaxy-vol-3-2023
```

Parameter Path:
- `movie_id`: ID film (biasanya berupa judul film yang di-slugify, contoh: "paayum-oli-nee-yenakku-2023")

**Response:**

```json
{
  "status": "success",
  "message": "Detail film Paayum Oli Nee Yenakku berhasil didapatkan",
  "data": {
    "id": "paayum-oli-nee-yenakku-2023",
    "title": "Paayum Oli Nee Yenakku",
    "poster": "http://example.com/poster.jpg",
    "rating": 6.0,
    "rating_count": null,
    "description": "Deskripsi film...",
    "views": 10000,
    "genres": ["Action", "Romance", "Thriller"],
    "quality": "HD",
    "year": 2023,
    "duration": "118 min",
    "country": "India",
    "release_date": "23 Jun 2023",
    "language": "Tamil",
    "director": "Karthik Adwait",
    "actors": ["Aktor 1", "Aktor 2"],
    "url": "http://194.102.105.201/paayum-oli-nee-yenakku-2023/",
    "watch_url": "https://example.com/embed/video123",
    "related_movies": [
      {
        "id": "...",
        "title": "...",
        "poster": "...",
        "year": null,
        "rating": null,
        "quality": null,
        "genres": [],
        "url": "..."
      },
      // ...film terkait lainnya
    ]
  }
}
```

### Watch Movie (HTML dengan Iframe)

```
GET /api/movie/paayum-oli-nee-yenakku-2023/watch
```

Parameter Path:
- `movie_id`: ID film (biasanya berupa judul film yang di-slugify, contoh: "paayum-oli-nee-yenakku-2023")

**Response:**
- 200 OK dengan konten HTML yang berisi iframe untuk memutar video
- 404 Not Found jika URL video tidak tersedia

**Cara Kerja:**
- Endpoint ini menggunakan reverse proxy internal untuk bypass Content Security Policy (CSP)
- Video dimuat melalui iframe yang menunjuk ke endpoint proxy, bukan langsung ke sumber aslinya
- Hal ini memungkinkan video diputar tanpa kendala pembatasan iframe dari sumber asli

**Contoh Penggunaan:**

Anda dapat membuka URL ini langsung di browser untuk menonton film:
```
http://localhost:8080/api/movie/paayum-oli-nee-yenakku-2023/watch
```

Atau menyematkannya dalam halaman web dengan iframe:
```html
<iframe src="http://localhost:8080/api/movie/paayum-oli-nee-yenakku-2023/watch" 
        width="100%" height="500" frameborder="0" allowfullscreen></iframe>
```

### Proxy Video Content

```
GET /api/movie/paayum-oli-nee-yenakku-2023/watch/proxy
```

Parameter Path:
- `movie_id`: ID film (biasanya berupa judul film yang di-slugify, contoh: "paayum-oli-nee-yenakku-2023")

**Response:**
- Konten video dari sumber asli dengan header yang dimodifikasi untuk memungkinkan embedding
- 404 Not Found jika URL video tidak tersedia

**Catatan:** Endpoint ini biasanya tidak perlu diakses secara langsung karena digunakan secara internal oleh endpoint `/watch`.

## Teknik Scraping dan Proxy

API ini menggunakan teknik scraping dan proxy yang canggih dan etis:

1. **Header Browser** - Menggunakan header HTTP yang sama persis dengan browser asli:
   - User-Agent: `Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36 Edg/134.0.0.0`
   - Accept: `text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7`
   - Accept-Encoding: `gzip, deflate, br, zstd`
   - Accept-Language: `en-US,en;q=0.9`
   - Sec-Ch-UA, Sec-Fetch-* headers untuk identifikasi browser
   - Host, Referer, dan Connection yang tepat
   - Dan header lainnya untuk meningkatkan otentisitas

2. **Error Handling** - Mendeteksi status HTTP error dan memberikan pesan yang jelas
   
3. **Pagination** - Melakukan scraping halaman yang berbeda berdasarkan parameter

4. **Reverse Proxy** - Teknik bypass untuk Content Security Policy:
   - Memproxy konten video dari sumber asli
   - Memodifikasi header respons untuk mengizinkan embedding
   - Menggunakan header identik dengan Chrome/Edge browser asli
   - Mengimplementasikan Sec-Fetch-* headers untuk mensimulasikan perilaku iframe
   - Memfilter header yang dapat menyebabkan masalah pada respons proxy

## Struktur Proyek

```
filmapik-api/
├── Cargo.toml            # Konfigurasi proyek dan dependensi
├── src/
│   ├── main.rs           # Entrypoint aplikasi
│   ├── config.rs         # Konfigurasi aplikasi
│   ├── errors.rs         # Penanganan error
│   ├── api/              # Handler API
│   │   ├── mod.rs
│   │   └── movie.rs      # Handler untuk endpoint film
│   ├── models/           # Model data
│   │   ├── mod.rs
│   │   └── movie.rs      # Model untuk film
│   └── scraper/          # Modul untuk scraping
│       ├── mod.rs
│       └── filmapik.rs   # Scraper untuk FilmApik
└── .env                  # (Opsional) Variabel lingkungan
``` 