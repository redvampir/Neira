# Startup script для Embeddings Service
# Автоматическая проверка окружения и запуск

$ErrorActionPreference = "Stop"

Write-Host "🧠 Запуск Neira Embeddings Service..." -ForegroundColor Cyan

# Путь к сервису
$SERVICE_DIR = "$PSScriptRoot\..\sensory_organs\embeddings_service"
Set-Location $SERVICE_DIR

# Проверка Python
Write-Host "Проверка Python..." -ForegroundColor Yellow
try {
    $pythonVersion = python --version 2>&1
    Write-Host "✅ $pythonVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ Python не найден! Установите Python 3.10+" -ForegroundColor Red
    exit 1
}

# Проверка/создание venv
if (-not (Test-Path "venv")) {
    Write-Host "Создание виртуального окружения..." -ForegroundColor Yellow
    python -m venv venv
    Write-Host "✅ venv создан" -ForegroundColor Green
}

# Активация venv
Write-Host "Активация venv..." -ForegroundColor Yellow
& ".\venv\Scripts\Activate.ps1"

# Проверка зависимостей
Write-Host "Проверка зависимостей..." -ForegroundColor Yellow
$pipList = pip list
if ($pipList -notmatch "sentence-transformers") {
    Write-Host "Установка зависимостей (первый запуск, займёт ~2-3 минуты)..." -ForegroundColor Yellow
    pip install -r requirements.txt
    Write-Host "✅ Зависимости установлены" -ForegroundColor Green
} else {
    Write-Host "✅ Зависимости уже установлены" -ForegroundColor Green
}

# Проверка CUDA
Write-Host "Проверка CUDA..." -ForegroundColor Yellow
$cudaCheck = python -c "import torch; print('CUDA:', torch.cuda.is_available()); print('Device:', torch.cuda.get_device_name(0) if torch.cuda.is_available() else 'CPU')" 2>&1
Write-Host $cudaCheck -ForegroundColor Cyan

# Запуск сервиса
Write-Host ""
Write-Host "🚀 Запуск сервиса на http://127.0.0.1:8765" -ForegroundColor Green
Write-Host "📚 Документация: http://127.0.0.1:8765/docs" -ForegroundColor Green
Write-Host ""
Write-Host "Нажмите Ctrl+C для остановки" -ForegroundColor Yellow
Write-Host ""

python app.py
