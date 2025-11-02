@echo off
setlocal ENABLEEXTENSIONS

chcp 65001 >nul
title Neira Setup

echo [Neira] Verifying Rust toolchain...
rustc --version >nul 2>&1
if errorlevel 1 (
    echo [Neira] Rust is not installed. Please install it from https://www.rust-lang.org/tools/install
    echo [Neira] After installing Rust, run setup.bat again.
    pause
    exit /b 1
)

echo [Neira] Updating Rust toolchain...
rustup update
rustup component add rustfmt clippy

echo [Neira] Refreshing dependencies...
cargo update

echo [Neira] Running initial cargo check...
cargo check
if errorlevel 1 (
    echo [Neira] cargo check failed. Please resolve the issues above.
    pause
    exit /b 1
)

echo.
echo [Neira] Setup complete.
pause
endlocal
exit /b 0
