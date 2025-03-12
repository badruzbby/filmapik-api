# Deployment FilmApik API dengan Docker

Dokumen ini menjelaskan cara men-deploy FilmApik API menggunakan Docker di VPS Debian.

## Prasyarat

- VPS dengan OS Debian
- Docker dan Docker Compose terinstal
- Git (opsional, untuk mengambil kode sumber)

## Catatan Penting Versi

Aplikasi ini menggunakan format Cargo.lock versi 4, yang membutuhkan versi Rust terbaru. Dockerfile sudah dikonfigurasi untuk menggunakan `rust:latest` untuk mendukung format ini.

## Cara Menginstal Docker dan Docker Compose

```bash
# Update paket dan instal dependensi
sudo apt update
sudo apt install -y apt-transport-https ca-certificates curl gnupg lsb-release

# Tambahkan GPG key Docker
curl -fsSL https://download.docker.com/linux/debian/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg

# Tambahkan repositori Docker
echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/debian $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

# Update paket dan instal Docker
sudo apt update
sudo apt install -y docker-ce docker-ce-cli containerd.io

# Instal Docker Compose
sudo curl -L "https://github.com/docker/compose/releases/download/v2.21.0/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose

# Tambahkan user ke grup docker (opsional, untuk menjalankan docker tanpa sudo)
sudo usermod -aG docker $USER
```

## Langkah Deployment

1. Kloning repositori (jika menggunakan Git) atau salin seluruh kode ke server

```bash
git clone <repository-url> filmapik-api
cd filmapik-api
```

2. Konfigurasi variabel lingkungan (opsional)

Edit file `docker-compose.yml` jika perlu mengubah konfigurasi:
   - Ubah port yang di-expose jika 8080 sudah digunakan
   - Sesuaikan variabel lingkungan seperti `FILMAPIK_URL` jika diperlukan

3. Build dan jalankan dengan Docker Compose

```bash
sudo docker-compose up -d --build
```

4. Periksa log untuk memastikan aplikasi berjalan dengan baik

```bash
sudo docker-compose logs -f
```

## Konfigurasi Lanjutan

### Mengubah Port

Jika ingin mengubah port, edit file `docker-compose.yml` dan ubah bagian `ports`:

```yaml
ports:
  - "80:8080"  # Mengubah port dari 8080 menjadi 80
```

### Variabel Lingkungan

Berikut variabel lingkungan yang tersedia:

- `APP_HOST`: Host yang digunakan aplikasi (default: 0.0.0.0)
- `APP_PORT`: Port yang digunakan aplikasi (default: 8080)
- `RUST_LOG`: Level logging (default: info)
- `FILMAPIK_URL`: URL untuk filmapik

## Pemeliharaan

### Restart Container

```bash
sudo docker-compose restart
```

### Update Aplikasi

```bash
git pull  # Jika menggunakan Git
sudo docker-compose down
sudo docker-compose up -d --build
```

### Melihat Log

```bash
sudo docker-compose logs -f
```

## Troubleshooting

1. **Container gagal start**
   
   Periksa log untuk detail kesalahan:
   ```bash
   sudo docker-compose logs -f
   ```

2. **Tidak dapat terhubung ke API**
   
   Pastikan port sudah dibuka di firewall:
   ```bash
   sudo ufw allow 8080/tcp  # Jika menggunakan UFW
   ```

3. **Masalah dengan Rust atau Cargo**

   Jika Anda melihat error terkait Cargo.lock atau versi Rust, pastikan Anda menggunakan versi terbaru di Dockerfile:
   ```
   FROM rust:latest as builder
   ``` 