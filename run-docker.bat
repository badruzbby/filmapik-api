@echo off
setlocal enabledelayedexpansion

echo FilmApik API Docker Helper
echo ------------------------

if "%1"=="build" (
    echo Building Docker image...
    docker-compose build --no-cache
    echo Build completed!
) else if "%1"=="start" (
    echo Starting FilmApik API container...
    docker-compose up -d
    echo Container started! API tersedia di http://localhost:8080
    
    echo Menampilkan log...
    docker-compose logs -f
) else if "%1"=="up" (
    echo Starting FilmApik API container...
    docker-compose up -d
    echo Container started! API tersedia di http://localhost:8080
    
    echo Menampilkan log...
    docker-compose logs -f
) else if "%1"=="stop" (
    echo Stopping FilmApik API container...
    docker-compose down
    echo Container stopped!
) else if "%1"=="down" (
    echo Stopping FilmApik API container...
    docker-compose down
    echo Container stopped!
) else if "%1"=="restart" (
    echo Restarting FilmApik API container...
    docker-compose restart
    echo Container restarted!
    
    echo Menampilkan log...
    docker-compose logs -f
) else if "%1"=="logs" (
    echo Menampilkan log...
    docker-compose logs -f
) else if "%1"=="status" (
    echo Status container:
    docker-compose ps
) else (
    echo Cara penggunaan: run-docker.bat [PERINTAH]
    echo.
    echo Perintah yang tersedia:
    echo   build   - Build ulang Docker image (gunakan jika ada perubahan kode)
    echo   start   - Mulai container FilmApik API (alias: up)
    echo   stop    - Hentikan container FilmApik API (alias: down)
    echo   restart - Restart container FilmApik API
    echo   logs    - Tampilkan log container
    echo   status  - Tampilkan status container
    echo.
    echo Contoh: run-docker.bat start
)

endlocal 