@echo off
setlocal ENABLEEXTENSIONS

chcp 65001 >nul
title Neira Build

echo [Neira] Checking prerequisites...
if not exist "Cargo.lock" (
    echo [Neira] Workspace not initialized. Running setup...
    call setup.bat
    if errorlevel 1 (
        echo [Neira] Setup failed. Cannot continue.
        pause
        exit /b 1
    )
)

echo [Neira] Cleaning previous build artifacts...
cargo clean

echo [Neira] Building release binary...
set RUST_BACKTRACE=1
cargo build --release --verbose
if errorlevel 1 (
    echo.
    echo [Neira] Build failed. Please review the compiler output above.
    pause
    exit /b 1
)

if exist "target\release\neira.exe" (
    echo.
    echo [Neira] Build succeeded. Binary is ready.
    pause
    exit /b 0
) else (
    echo.
    echo [Neira] Build finished but binary is missing. Check build logs.
    pause
    exit /b 1
)

endlocal
