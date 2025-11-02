@echo off
chcp 866 >nul
title Neira Installation

echo Installing Neira dependencies...

REM Check if Rust is installed
where rustc >nul 2>nul
if %ERRORLEVEL% neq 0 (
    echo Installing Rust...
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
)

REM Install dependencies
cargo install --path .

echo Installation complete!
pause
