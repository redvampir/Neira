@echo off
setlocal ENABLEEXTENSIONS

rem Use UTF-8 code page for readable output
chcp 65001 >nul
title Neira Launcher

echo [Neira] Preparing workspace...
if not exist "logs" mkdir logs
if not exist "data" mkdir data

if not exist "target\release\neira.exe" (
    echo [Neira] Release binary not found. Building...
    call build.bat
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

echo [Neira] Starting server...
start "Neira Server" /MIN cmd /c ^
    "set RUST_LOG=%RUST_LOG%&& set NEIRA_DATA_DIR=%NEIRA_DATA_DIR%&& set NEIRA_SUCCESS_THRESHOLD=%NEIRA_SUCCESS_THRESHOLD%&& set NEIRA_BIND_ADDR=%NEIRA_BIND_ADDR%&& target\release\neira.exe > logs\neira.log 2>&1"

timeout /t 3 /nobreak >nul

echo [Neira] Server is running. Logs: logs\neira.log
set "NEIRA_URL=http://localhost:9090/"
echo [Neira] Opening web UI at %NEIRA_URL%
start "" "%NEIRA_URL%"

echo.
echo [Neira] Press any key to stop the server...
pause >nul

echo [Neira] Stopping server...
taskkill /IM neira.exe /F >nul 2>&1

echo [Neira] Server stopped. Goodbye.
endlocal
