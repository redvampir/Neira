# Smoke Test для Dashboard Нейры
# Проверяет основные функции интерфейса

Write-Host "`n=== SMOKE TEST: NEIRA DASHBOARD ===" -ForegroundColor Cyan

$baseUrl = "http://localhost:9090"
$errors = @()
$warnings = @()

# Test 1: Backend API доступен
Write-Host "`n[1] Testing backend API..." -ForegroundColor Yellow
try {
    $stats = Invoke-RestMethod -Uri "$baseUrl/api/neira/consciousness/stats" -TimeoutSec 5
    if ($stats.total_thoughts_recorded -ge 0) {
        Write-Host "  ✓ API responding" -ForegroundColor Green
    } else {
        $errors += "API returned invalid data"
    }
} catch {
    $errors += "Backend API not accessible: $($_.Exception.Message)"
}

# Test 2: Static files доступны
Write-Host "`n[2] Testing static files..." -ForegroundColor Yellow
$staticFiles = @(
    "/consciousness/",
    "/consciousness/dashboard.js",
    "/consciousness/dashboard.css"
)

foreach ($file in $staticFiles) {
    try {
        $response = Invoke-WebRequest -Uri "$baseUrl$file" -TimeoutSec 5 -UseBasicParsing
        if ($response.StatusCode -eq 200) {
            Write-Host "  ✓ $file ($(([Math]::Round($response.Content.Length / 1024, 1))) KB)" -ForegroundColor Green
        } else {
            $warnings += "$file returned status $($response.StatusCode)"
        }
    } catch {
        $errors += "Failed to load $file"
    }
}

# Test 3: Проверка содержимого dashboard.js
Write-Host "`n[3] Checking dashboard.js content..." -ForegroundColor Yellow
try {
    $jsContent = Invoke-WebRequest -Uri "$baseUrl/consciousness/dashboard.js" -UseBasicParsing
    $jsText = $jsContent.Content
    
    # Проверяем наличие улучшённых текстов
    $checks = @{
        "i18n object" = $jsText -match "const i18n"
        "Improved Russian (Всё в порядке)" = $jsText -match "Всё в порядке"
        "Improved Russian (Мыслей в базе)" = $jsText -match "Мыслей в базе"
        "Improved Russian (Составить отчёт)" = $jsText -match "Составить отчёт"
        "Language switcher function" = $jsText -match "function setLanguage"
        "Translation helper" = $jsText -match "function t\("
    }
    
    foreach ($check in $checks.GetEnumerator()) {
        if ($check.Value) {
            Write-Host "  ✓ $($check.Key)" -ForegroundColor Green
        } else {
            $warnings += "Missing: $($check.Key)"
        }
    }
} catch {
    $errors += "Failed to analyze dashboard.js"
}

# Test 4: Проверка index.html
Write-Host "`n[4] Checking index.html content..." -ForegroundColor Yellow
try {
    $htmlContent = Invoke-WebRequest -Uri "$baseUrl/consciousness/" -UseBasicParsing
    $htmlText = $htmlContent.Content
    
    $htmlChecks = @{
        "ARIA role=banner" = $htmlText -match 'role="banner"'
        "ARIA aria-label" = $htmlText -match 'aria-label='
        "Language buttons (RU)" = $htmlText -match 'data-lang="ru"'
        "Language buttons (EN)" = $htmlText -match 'data-lang="en"'
        "Chart.js loaded" = $htmlText -match "chart.umd.min.js"
        "Tailwind CSS loaded" = $htmlText -match "tailwindcss.com"
        "Improved status text" = $htmlText -match "Всё в порядке"
    }
    
    foreach ($check in $htmlChecks.GetEnumerator()) {
        if ($check.Value) {
            Write-Host "  ✓ $($check.Key)" -ForegroundColor Green
        } else {
            $warnings += "Missing in HTML: $($check.Key)"
        }
    }
} catch {
    $errors += "Failed to analyze index.html"
}

# Test 5: Проверка API endpoints
Write-Host "`n[5] Testing API endpoints..." -ForegroundColor Yellow
$endpoints = @(
    "/api/neira/consciousness/stats",
    "/api/neira/personality",
    "/api/neira/organs"
)

foreach ($endpoint in $endpoints) {
    try {
        $response = Invoke-RestMethod -Uri "$baseUrl$endpoint" -TimeoutSec 5
        Write-Host "  ✓ $endpoint" -ForegroundColor Green
    } catch {
        $warnings += "Endpoint failed: $endpoint"
    }
}

# Summary
Write-Host "`n=== SUMMARY ===" -ForegroundColor Cyan
Write-Host "Errors: $($errors.Count)" -ForegroundColor $(if ($errors.Count -eq 0) { "Green" } else { "Red" })
Write-Host "Warnings: $($warnings.Count)" -ForegroundColor $(if ($warnings.Count -eq 0) { "Green" } else { "Yellow" })

if ($errors.Count -gt 0) {
    Write-Host "`nERRORS:" -ForegroundColor Red
    $errors | ForEach-Object { Write-Host "  - $_" -ForegroundColor Red }
}

if ($warnings.Count -gt 0) {
    Write-Host "`nWARNINGS:" -ForegroundColor Yellow
    $warnings | ForEach-Object { Write-Host "  - $_" -ForegroundColor Yellow }
}

if ($errors.Count -eq 0 -and $warnings.Count -eq 0) {
    Write-Host "`n✓ ALL TESTS PASSED" -ForegroundColor Green
    exit 0
} elseif ($errors.Count -eq 0) {
    Write-Host "`n⚠ TESTS PASSED WITH WARNINGS" -ForegroundColor Yellow
    exit 0
} else {
    Write-Host "`n✗ TESTS FAILED" -ForegroundColor Red
    exit 1
}
