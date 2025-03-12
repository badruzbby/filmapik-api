#!/bin/bash

# Warna untuk output
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}FilmApik API Docker Helper${NC}"
echo "------------------------"

if [ "$1" == "prepare" ]; then
    echo -e "${GREEN}Mempersiapkan proyek untuk Docker build...${NC}"
    echo -e "Menjalankan cargo update untuk memastikan Cargo.lock ada..."
    cargo update
    echo -e "${GREEN}Persiapan selesai!${NC}"

elif [ "$1" == "clean-lock" ]; then
    echo -e "${RED}Menghapus Cargo.lock untuk mengatasi masalah versi...${NC}"
    if [ -f "Cargo.lock" ]; then
        rm Cargo.lock
        echo -e "${GREEN}File Cargo.lock berhasil dihapus.${NC}"
    else
        echo -e "${BLUE}File Cargo.lock tidak ditemukan.${NC}"
    fi
    echo -e "${GREEN}Selesai.${NC}"

elif [ "$1" == "build" ]; then
    echo -e "${GREEN}Building Docker image...${NC}"
    echo -e "Memastikan Cargo.lock ada..."
    if [ ! -f "Cargo.lock" ]; then
        echo -e "${BLUE}File Cargo.lock tidak ditemukan. Menjalankan cargo update...${NC}"
        cargo update
    fi
    docker-compose build --no-cache
    echo -e "${GREEN}Build completed!${NC}"

elif [ "$1" == "build-nightly" ]; then
    echo -e "${GREEN}Building Docker image dengan Rust nightly...${NC}"
    echo -e "Memastikan Cargo.lock ada..."
    if [ ! -f "Cargo.lock" ]; then
        echo -e "${BLUE}File Cargo.lock tidak ditemukan. Menjalankan cargo update...${NC}"
        cargo update
    fi
    docker-compose -f docker-compose.nightly.yml build --no-cache
    echo -e "${GREEN}Build completed!${NC}"

elif [ "$1" == "start" ] || [ "$1" == "up" ]; then
    echo -e "${GREEN}Starting FilmApik API container...${NC}"
    docker-compose up -d
    echo -e "${GREEN}Container started! API tersedia di http://localhost:8080${NC}"
    
    # Tampilkan log
    echo -e "${BLUE}Menampilkan log...${NC}"
    docker-compose logs -f

elif [ "$1" == "start-nightly" ]; then
    echo -e "${GREEN}Starting FilmApik API container (nightly)...${NC}"
    docker-compose -f docker-compose.nightly.yml up -d
    echo -e "${GREEN}Container started! API tersedia di http://localhost:8080${NC}"
    
    # Tampilkan log
    echo -e "${BLUE}Menampilkan log...${NC}"
    docker-compose -f docker-compose.nightly.yml logs -f

elif [ "$1" == "stop" ] || [ "$1" == "down" ]; then
    echo -e "${RED}Stopping FilmApik API container...${NC}"
    docker-compose down
    echo -e "${RED}Container stopped!${NC}"

elif [ "$1" == "stop-nightly" ]; then
    echo -e "${RED}Stopping FilmApik API container (nightly)...${NC}"
    docker-compose -f docker-compose.nightly.yml down
    echo -e "${RED}Container stopped!${NC}"

elif [ "$1" == "restart" ]; then
    echo -e "${RED}Restarting FilmApik API container...${NC}"
    docker-compose restart
    echo -e "${GREEN}Container restarted!${NC}"
    
    # Tampilkan log
    echo -e "${BLUE}Menampilkan log...${NC}"
    docker-compose logs -f

elif [ "$1" == "restart-nightly" ]; then
    echo -e "${RED}Restarting FilmApik API container (nightly)...${NC}"
    docker-compose -f docker-compose.nightly.yml restart
    echo -e "${GREEN}Container restarted!${NC}"
    
    # Tampilkan log
    echo -e "${BLUE}Menampilkan log...${NC}"
    docker-compose -f docker-compose.nightly.yml logs -f

elif [ "$1" == "logs" ]; then
    echo -e "${BLUE}Menampilkan log...${NC}"
    docker-compose logs -f

elif [ "$1" == "logs-nightly" ]; then
    echo -e "${BLUE}Menampilkan log (nightly)...${NC}"
    docker-compose -f docker-compose.nightly.yml logs -f

elif [ "$1" == "status" ]; then
    echo -e "${BLUE}Status container:${NC}"
    docker-compose ps

elif [ "$1" == "status-nightly" ]; then
    echo -e "${BLUE}Status container (nightly):${NC}"
    docker-compose -f docker-compose.nightly.yml ps

else
    echo "Cara penggunaan: ./run-docker.sh [PERINTAH]"
    echo ""
    echo "Perintah yang tersedia:"
    echo "  prepare       - Menyiapkan proyek untuk build (menjalankan cargo update)"
    echo "  clean-lock    - Menghapus Cargo.lock untuk mengatasi masalah versi"
    echo "  build         - Build ulang Docker image (gunakan jika ada perubahan kode)"
    echo "  build-nightly - Build ulang Docker image dengan Rust nightly (mendukung edition 2024)"
    echo "  start         - Mulai container FilmApik API (alias: up)"
    echo "  start-nightly - Mulai container dengan Rust nightly"
    echo "  stop          - Hentikan container FilmApik API (alias: down)"
    echo "  stop-nightly  - Hentikan container nightly"
    echo "  restart       - Restart container FilmApik API"
    echo "  restart-nightly - Restart container nightly"
    echo "  logs          - Tampilkan log container"
    echo "  logs-nightly  - Tampilkan log container nightly"
    echo "  status        - Tampilkan status container"
    echo "  status-nightly - Tampilkan status container nightly"
    echo ""
    echo "Contoh: ./run-docker.sh start"
    echo ""
    echo "Pemecahan Masalah:"
    echo "  Jika menemui error 'Cargo.lock not found', jalankan './run-docker.sh prepare' terlebih dahulu"
    echo "  Jika menemui error 'edition2024 is required', gunakan './run-docker.sh build-nightly'"
    echo "  Jika menemui error 'lock file version 4', jalankan './run-docker.sh clean-lock' untuk membersihkan Cargo.lock"
fi 