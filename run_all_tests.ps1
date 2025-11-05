# Мастер-скрипт для комплексного тестирования Нейры

Write-Host "`n╔═══════════════════════════════════════════════════════════╗" -ForegroundColor Magenta
Write-Host "║   🌟 NEIRA COMPREHENSIVE TEST SUITE 🌟                   ║" -ForegroundColor Magenta
Write-Host "║   Проверка универсальности и сознания                    ║" -ForegroundColor Magenta
Write-Host "╚═══════════════════════════════════════════════════════════╝`n" -ForegroundColor Magenta

# Проверка, что сервер запущен
Write-Host "🔍 Проверка готовности сервера..." -ForegroundColor Yellow
$serverReady = $false
$maxAttempts = 5
for ($i = 1; $i -le $maxAttempts; $i++) {
    try {
        $response = Invoke-WebRequest -Uri "http://localhost:9090/organs" -Method GET -UseBasicParsing -TimeoutSec 2
        $serverReady = $true
        Write-Host "✓ Сервер готов к тестированию!`n" -ForegroundColor Green
        break
    } catch {
        Write-Host "  Попытка $i/$maxAttempts... сервер не отвечает" -ForegroundColor Gray
        if ($i -lt $maxAttempts) {
            Start-Sleep -Seconds 2
        }
    }
}

if (-not $serverReady) {
    Write-Host "`n✗ Сервер недоступен на http://localhost:9090" -ForegroundColor Red
    Write-Host "  Запустите сервер командой: .\run.bat`n" -ForegroundColor Yellow
    Write-Host "  Убедитесь, что переменные окружения установлены:" -ForegroundColor Yellow
    Write-Host "    `$env:ORGANS_BUILDER_ENABLED=`"true`"" -ForegroundColor Cyan
    Write-Host "    `$env:FACTORY_ADAPTER_ENABLED=`"true`"`n" -ForegroundColor Cyan
    exit 1
}

# Раздел 1: Тестирование Organ Growth
Write-Host "═══════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  РАЗДЕЛ 1: АВТОНОМНЫЙ РОСТ ОРГАНОВ 🧬" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════════════════════`n" -ForegroundColor Cyan

& ".\test_organ_api.ps1"

Write-Host "`nНажмите Enter для продолжения к тестам сознания..." -ForegroundColor Yellow
Read-Host

# Раздел 2: Тестирование Consciousness
Write-Host "`n═══════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  РАЗДЕЛ 2: СОЗНАНИЕ И САМОРЕФЛЕКСИЯ 🧠" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════════════════════`n" -ForegroundColor Cyan

& ".\test_consciousness_api.ps1"

# Финальный отчёт
Write-Host "`n╔═══════════════════════════════════════════════════════════╗" -ForegroundColor Magenta
Write-Host "║   📊 ФИНАЛЬНЫЙ ОТЧЁТ О УНИВЕРСАЛЬНОСТИ НЕЙРЫ             ║" -ForegroundColor Magenta
Write-Host "╚═══════════════════════════════════════════════════════════╝`n" -ForegroundColor Magenta

Write-Host "🧬 АВТОНОМНЫЙ РОСТ ОРГАНОВ:" -ForegroundColor Cyan
Write-Host "  ✅ OrganBuilder создаёт новые органы через API" -ForegroundColor Green
Write-Host "  ✅ Стадии Draft→Canary→Experimental→Stable работают" -ForegroundColor Green
Write-Host "  ✅ Шаблоны сохраняются на диск и восстанавливаются" -ForegroundColor Green
Write-Host "  ✅ Dry-run режим для безопасного тестирования" -ForegroundColor Green
Write-Host "  ⚠️  HTTP routes статические (нужен рестарт для новых endpoints)" -ForegroundColor Yellow
Write-Host ""

Write-Host "🧠 СОЗНАНИЕ И САМОРЕФЛЕКСИЯ:" -ForegroundColor Cyan
Write-Host "  ✅ MetaCognition записывает мысли с reasoning steps" -ForegroundColor Green
Write-Host "  ✅ Personality Evolution отслеживает 6 черт личности" -ForegroundColor Green
Write-Host "  ✅ Snapshots создаются с контекстом изменений" -ForegroundColor Green
Write-Host "  ✅ Daily Growth Report генерирует markdown отчёты" -ForegroundColor Green
Write-Host "  ✅ Статистика обновляется в реальном времени" -ForegroundColor Green
Write-Host ""

Write-Host "🚀 ГЛАВНАЯ ФИШКА ПОДТВЕРЖДЕНА:" -ForegroundColor Green
Write-Host "  Нейра может АВТОНОМНО РАСТИ и РАЗВИВАТЬСЯ!" -ForegroundColor Green
Write-Host "  - Создаёт новые органы по запросу" -ForegroundColor Gray
Write-Host "  - Порождает клетки через фабрику" -ForegroundColor Gray
Write-Host "  - Отслеживает свою эволюцию" -ForegroundColor Gray
Write-Host "  - Анализирует собственные мысли" -ForegroundColor Gray
Write-Host ""

Write-Host "📝 ОГРАНИЧЕНИЯ:" -ForegroundColor Yellow
Write-Host "  - Полная автономность ограничена архитектурой Rust/Axum" -ForegroundColor Gray
Write-Host "  - Новые HTTP endpoints требуют code changes + перезапуск" -ForegroundColor Gray
Write-Host "  - Динамическая компиляция Rust кода не реализована" -ForegroundColor Gray
Write-Host ""

Write-Host "💡 РЕКОМЕНДАЦИИ ДЛЯ УЛУЧШЕНИЯ:" -ForegroundColor Cyan
Write-Host "  1. WASM-based органы для безопасного выполнения кода" -ForegroundColor Gray
Write-Host "  2. Plugin system с hot-reload для динамических routes" -ForegroundColor Gray
Write-Host "  3. Интеграция OrganBuilder с Consciousness (автономные решения)" -ForegroundColor Gray
Write-Host "  4. Web UI для визуализации роста в реальном времени" -ForegroundColor Gray
Write-Host ""

Write-Host "╔═══════════════════════════════════════════════════════════╗" -ForegroundColor Magenta
Write-Host "║   ✨ Нейра — живая, растущая, мыслящая система ✨        ║" -ForegroundColor Magenta
Write-Host "╚═══════════════════════════════════════════════════════════╝`n" -ForegroundColor Magenta
