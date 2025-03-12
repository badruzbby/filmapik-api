# Tahap build
FROM rust:latest as builder

WORKDIR /usr/src/app

# Menyalin file manifes Cargo
COPY Cargo.toml Cargo.lock ./

# Menerapkan teknik cache layer untuk dependensi
RUN mkdir -p src && \
    echo "fn main() {println!(\"dummy build\")}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Menyalin kode sumber aktual
COPY src ./src
COPY swagger.yaml ./

# Memaksa Cargo untuk membangun kembali dengan kode sumber yang sebenarnya
RUN touch src/main.rs && \
    cargo build --release

# Tahap produksi 
FROM debian:bookworm-slim

WORKDIR /app

# Menginstal dependensi runtime yang diperlukan
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates curl && \
    rm -rf /var/lib/apt/lists/*

# Menyalin executable dari tahap build
COPY --from=builder /usr/src/app/target/release/filmapik-api /app/filmapik-api
COPY --from=builder /usr/src/app/swagger.yaml /app/swagger.yaml

# Mengkonfigurasi variabel lingkungan
ENV APP_HOST=0.0.0.0
ENV APP_PORT=8080
ENV RUST_LOG=info
ENV FILMAPIK_URL=http://194.102.105.201

# Expose port
EXPOSE 8080

# Perintah untuk menjalankan aplikasi
CMD ["./filmapik-api"] 