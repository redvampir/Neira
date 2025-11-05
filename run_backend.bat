@echo off
setlocal ENABLEEXTENSIONS

rem Use UTF-8 code page for readable output
chcp 65001 >nul
title Neira Backend (Consciousness)

echo [Neira] Preparing workspace...
if not exist "logs" mkdir logs
if not exist "data" mkdir data

if not exist "spinal_cord\target\release\backend.exe" (
    echo [Neira] Backend binary not found. Building...
    cd spinal_cord
    cargo build --release --bin backend
    cd ..
    if errorlevel 1 (
        echo [Neira] Build failed. See messages above.
        pause
        exit /b 1
    )
)

set "RUST_LOG=info"
set "NEIRA_DATA_DIR=%~dp0data"
set "NEIRA_SUCCESS_THRESHOLD=0.8"
set "NEIRA_BIND_ADDR=0.0.0.0:9090"
set "ORGANS_BUILDER_ENABLED=true"
set "FACTORY_ADAPTER_ENABLED=true"

echo [Neira] Starting Consciousness Backend on port 9090...
start "Neira Backend" /MIN cmd /c ^
    "set RUST_LOG=%RUST_LOG%&& set NEIRA_DATA_DIR=%NEIRA_DATA_DIR%&& set NEIRA_SUCCESS_THRESHOLD=%NEIRA_SUCCESS_THRESHOLD%&& set NEIRA_BIND_ADDR=%NEIRA_BIND_ADDR%&& set ORGANS_BUILDER_ENABLED=%ORGANS_BUILDER_ENABLED%&& set FACTORY_ADAPTER_ENABLED=%FACTORY_ADAPTER_ENABLED%&& spinal_cord\target\release\backend.exe > logs\backend.log 2>&1"

timeout /t 5 /nobreak >nul

echo [Neira] Backend is running. Logs: logs\backend.log
set "NEIRA_URL=http://localhost:9090/static/consciousness/"
echo [Neira] Opening Consciousness Dashboard at %NEIRA_URL%
start "" "%NEIRA_URL%"

echo.
echo [Neira] Backend started successfully!
echo   - Dashboard: http://localhost:9090/static/consciousness/
echo   - API: http://localhost:9090/api/neira/consciousness/stats
echo   - Logs: logs\backend.log
echo.
echo [Neira] Press any key to stop the server...
pause >nul

echo [Neira] Stopping backend...
taskkill /IM backend.exe /F >nul 2>&1

echo [Neira] Backend stopped. Goodbye.
endlocal
