@echo off
REM neira:meta
REM id: NEI-20240709-185300-launcher-wrapper
REM intent: feature
REM summary: |
REM   Добавлен бат-файл для запуска графического лаунчера из проводника двойным кликом.

set SCRIPT_DIR=%~dp0
set PS_SCRIPT=%SCRIPT_DIR%tools\launcher\neira_launcher.ps1

if not exist "%PS_SCRIPT%" (
    echo [Neira] Не найден файл %PS_SCRIPT%.
    pause
    exit /b 1
)

start "" powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%PS_SCRIPT%"
exit /b 0
