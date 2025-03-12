#!/bin/bash

# Warna untuk output
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}FilmApik API Docker Helper${NC}"
echo "------------------------"

if [ "$1" == "build" ]; then
    echo -e "${GREEN}Building Docker image...${NC}"
    docker-compose build --no-cache
    echo -e "${GREEN}Build completed!${NC}"

elif [ "$1" == "start" ] || [ "$1" == "up" ]; then
    echo -e "${GREEN}Starting FilmApik API container...${NC}"
    docker-compose up -d
    echo -e "${GREEN}Container started! API tersedia di http://localhost:8080${NC}"
    
    # Tampilkan log
    echo -e "${BLUE}Menampilkan log...${NC}"
    docker-compose logs -f

elif [ "$1" == "stop" ] || [ "$1" == "down" ]; then
    echo -e "${RED}Stopping FilmApik API container...${NC}"
    docker-compose down
    echo -e "${RED}Container stopped!${NC}"

elif [ "$1" == "restart" ]; then
    echo -e "${RED}Restarting FilmApik API container...${NC}"
    docker-compose restart
    echo -e "${GREEN}Container restarted!${NC}"
    
    # Tampilkan log
    echo -e "${BLUE}Menampilkan log...${NC}"
    docker-compose logs -f

elif [ "$1" == "logs" ]; then
    echo -e "${BLUE}Menampilkan log...${NC}"
    docker-compose logs -f

elif [ "$1" == "status" ]; then
    echo -e "${BLUE}Status container:${NC}"
    docker-compose ps

else
    echo "Cara penggunaan: ./run-docker.sh [PERINTAH]"
    echo ""
    echo "Perintah yang tersedia:"
    echo "  build   - Build ulang Docker image (gunakan jika ada perubahan kode)"
    echo "  start   - Mulai container FilmApik API (alias: up)"
    echo "  stop    - Hentikan container FilmApik API (alias: down)"
    echo "  restart - Restart container FilmApik API"
    echo "  logs    - Tampilkan log container"
    echo "  status  - Tampilkan status container"
    echo ""
    echo "Contoh: ./run-docker.sh start"
fi 