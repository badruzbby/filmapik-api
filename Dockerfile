FROM rust:1.81-slim as builder

WORKDIR /app

# Copy manifest files
COPY Cargo.toml ./

# Buat Cargo.lock baru di dalam container (jangan gunakan yang dari host)
RUN touch Cargo.lock

# Install dependensi untuk build di Debian
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev build-essential && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Create a dummy main.rs to build dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    # Buat Cargo.lock yang kompatibel dengan versi Cargo di container
    cargo update && \
    cargo build --release && \
    rm -rf src

# Copy actual source code, kecuali Cargo.lock (gunakan .dockerignore)
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

WORKDIR /app

# Install OpenSSL, CA certificates, dan curl (untuk healthcheck)
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder stage
COPY --from=builder /app/target/release/filmapik-api /app/filmapik-api

# Expose the port the API listens on
EXPOSE 8080

# Run the API
CMD ["/app/filmapik-api"] 