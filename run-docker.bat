@echo off
setlocal enabledelayedexpansion

echo FilmApik API Docker Helper
echo ------------------------

if "%1"=="prepare" (
    echo Mempersiapkan proyek untuk Docker build...
    echo Menjalankan cargo update untuk memastikan Cargo.lock ada...
    cargo update
    echo Persiapan selesai!
) else if "%1"=="build" (
    echo Building Docker image...
    echo Memastikan Cargo.lock ada...
    if not exist Cargo.lock (
        echo File Cargo.lock tidak ditemukan. Menjalankan cargo update...
        cargo update
    )
    docker-compose build --no-cache
    echo Build completed!
) else if "%1"=="build-nightly" (
    echo Building Docker image dengan Rust nightly...
    echo Memastikan Cargo.lock ada...
    if not exist Cargo.lock (
        echo File Cargo.lock tidak ditemukan. Menjalankan cargo update...
        cargo update
    )
    docker-compose -f docker-compose.nightly.yml build --no-cache
    echo Build completed!
) else if "%1"=="start" (
    echo Starting FilmApik API container...
    docker-compose up -d
    echo Container started! API tersedia di http://localhost:8080
    
    echo Menampilkan log...
    docker-compose logs -f
) else if "%1"=="start-nightly" (
    echo Starting FilmApik API container (nightly)...
    docker-compose -f docker-compose.nightly.yml up -d
    echo Container started! API tersedia di http://localhost:8080
    
    echo Menampilkan log...
    docker-compose -f docker-compose.nightly.yml logs -f
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
) else if "%1"=="stop-nightly" (
    echo Stopping FilmApik API container (nightly)...
    docker-compose -f docker-compose.nightly.yml down
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
) else if "%1"=="restart-nightly" (
    echo Restarting FilmApik API container (nightly)...
    docker-compose -f docker-compose.nightly.yml restart
    echo Container restarted!
    
    echo Menampilkan log...
    docker-compose -f docker-compose.nightly.yml logs -f
) else if "%1"=="logs" (
    echo Menampilkan log...
    docker-compose logs -f
) else if "%1"=="logs-nightly" (
    echo Menampilkan log (nightly)...
    docker-compose -f docker-compose.nightly.yml logs -f
) else if "%1"=="status" (
    echo Status container:
    docker-compose ps
) else if "%1"=="status-nightly" (
    echo Status container (nightly):
    docker-compose -f docker-compose.nightly.yml ps
) else (
    echo Cara penggunaan: run-docker.bat [PERINTAH]
    echo.
    echo Perintah yang tersedia:
    echo   prepare       - Menyiapkan proyek untuk build (menjalankan cargo update)
    echo   build         - Build ulang Docker image (gunakan jika ada perubahan kode)
    echo   build-nightly - Build ulang Docker image dengan Rust nightly (mendukung edition 2024)
    echo   start         - Mulai container FilmApik API (alias: up)
    echo   start-nightly - Mulai container dengan Rust nightly
    echo   stop          - Hentikan container FilmApik API (alias: down)
    echo   stop-nightly  - Hentikan container nightly
    echo   restart       - Restart container FilmApik API
    echo   restart-nightly - Restart container nightly
    echo   logs          - Tampilkan log container
    echo   logs-nightly  - Tampilkan log container nightly
    echo   status        - Tampilkan status container
    echo   status-nightly - Tampilkan status container nightly
    echo.
    echo Contoh: run-docker.bat start
    echo.
    echo Pemecahan Masalah:
    echo   Jika menemui error 'Cargo.lock not found', jalankan 'run-docker.bat prepare' terlebih dahulu
    echo   Jika menemui error 'edition2024 is required', gunakan 'run-docker.bat build-nightly'
)

endlocal 