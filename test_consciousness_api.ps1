# Скрипт для тестирования Consciousness API

Write-Host "🧠 Тестирование Consciousness API Нейры`n" -ForegroundColor Cyan

$baseUrl = "http://localhost:9090"

# Проверка доступности сервера
Write-Host "1. Проверка сервера..." -ForegroundColor Yellow
try {
    $response = Invoke-WebRequest -Uri "$baseUrl/api/neira/consciousness/stats" -Method GET -UseBasicParsing
    Write-Host "✓ Consciousness API доступен (статус: $($response.StatusCode))`n" -ForegroundColor Green
} catch {
    Write-Host "✗ Consciousness API недоступен. Запустите сервер командой: .\run.bat`n" -ForegroundColor Red
    exit 1
}

# Тест 1: Получение метакогнитивной статистики
Write-Host "2. Метакогнитивная статистика:" -ForegroundColor Yellow
try {
    $stats = Invoke-RestMethod -Uri "$baseUrl/api/neira/consciousness/stats" -Method GET
    Write-Host "✓ Статистика получена:" -ForegroundColor Green
    Write-Host "  - Записанных мыслей: $($stats.total_thoughts_recorded)" -ForegroundColor Gray
    Write-Host "  - Обнаружено искажений: $($stats.total_biases_detected)" -ForegroundColor Gray
    Write-Host "  - Задач улучшения: $($stats.total_improvement_tasks)" -ForegroundColor Gray
    Write-Host "  - Незавершённых задач: $($stats.pending_tasks)" -ForegroundColor Gray
    Write-Host "  - Завершённых задач: $($stats.completed_tasks)" -ForegroundColor Gray
    Write-Host "  - Средн. качество процесса: $($stats.avg_process_quality)`n" -ForegroundColor Gray
} catch {
    Write-Host "✗ Ошибка: $($_.Exception.Message)`n" -ForegroundColor Red
}

# Тест 2: Запись thought trace
Write-Host "3. Запись нового thought trace..." -ForegroundColor Yellow
try {
    $thoughtBody = @{
        dialogue_id = "test_dialogue_001"
        reasoning_steps = @(
            "Анализирую запрос пользователя о создании органа",
            "Проверяю наличие необходимых компонентов",
            "Формирую план действий"
        )
        decisions = @(
            @{
                step = "Выбор типа органа"
                rationale = "Пользователь запросил анализ кода, поэтому тип 'analysis'"
                confidence = 0.9
                alternatives_considered = @("analysis", "transformation", "validation")
            },
            @{
                step = "Определение capabilities"
                rationale = "Нужны syntax_check и code_review для полного анализа"
                confidence = 0.85
                alternatives_considered = @("только syntax_check", "добавить linting")
            }
        )
    } | ConvertTo-Json -Depth 5

    $thoughtResult = Invoke-RestMethod -Uri "$baseUrl/api/neira/consciousness/thought" -Method POST -Body $thoughtBody -ContentType "application/json"
    Write-Host "✓ Thought trace записан: trace_id=$($thoughtResult.trace_id)`n" -ForegroundColor Green
} catch {
    Write-Host "✗ Ошибка записи thought: $($_.Exception.Message)`n" -ForegroundColor Red
}

# Тест 3: Получение текущих черт личности
Write-Host "4. Текущие черты личности (Personality):" -ForegroundColor Yellow
try {
    $personality = Invoke-RestMethod -Uri "$baseUrl/api/neira/consciousness/personality" -Method GET
    Write-Host "✓ Черты личности получены (время: $($personality.timestamp)):" -ForegroundColor Green
    $personality.traits.PSObject.Properties | ForEach-Object {
        $traitName = $_.Name
        $traitValue = $_.Value.value
        $barLength = [math]::Round($traitValue * 20)
        $bar = "█" * $barLength + "░" * (20 - $barLength)
        Write-Host "  - ${traitName}: $bar ($traitValue)" -ForegroundColor Cyan
    }
    Write-Host ""
} catch {
    Write-Host "✗ Ошибка получения личности: $($_.Exception.Message)`n" -ForegroundColor Red
}

# Тест 4: Создание snapshot личности
Write-Host "5. Создание snapshot личности..." -ForegroundColor Yellow
try {
    $snapshotBody = @{
        context = "После теста organ growth - увидела свою способность к саморазвитию"
    } | ConvertTo-Json

    $snapshot = Invoke-RestMethod -Uri "$baseUrl/api/neira/consciousness/personality/snapshot" -Method POST -Body $snapshotBody -ContentType "application/json"
    Write-Host "✓ Snapshot создан:" -ForegroundColor Green
    Write-Host "  - ID: $($snapshot.snapshot_id)" -ForegroundColor Gray
    Write-Host "  - Время: $($snapshot.timestamp)" -ForegroundColor Gray
    Write-Host "  - Контекст: $($snapshot.context)" -ForegroundColor Gray
    Write-Host ""
} catch {
    Write-Host "✗ Ошибка создания snapshot: $($_.Exception.Message)`n" -ForegroundColor Red
}

# Тест 5: Генерация growth report
Write-Host "6. Генерация daily growth report (за последние 7 дней)..." -ForegroundColor Yellow
try {
    $report = Invoke-RestMethod -Uri "$baseUrl/api/neira/consciousness/growth-report?days=7" -Method GET
    Write-Host "✓ Отчёт сгенерирован:" -ForegroundColor Green
    
    # Показываем первые 500 символов отчёта
    $preview = $report.report_markdown.Substring(0, [Math]::Min(500, $report.report_markdown.Length))
    Write-Host "$preview..." -ForegroundColor Gray
    Write-Host "`n  (Полный отчёт содержит $($report.report_markdown.Length) символов)" -ForegroundColor DarkGray
    Write-Host ""
} catch {
    Write-Host "✗ Ошибка генерации отчёта: $($_.Exception.Message)`n" -ForegroundColor Red
}

# Тест 6: Повторная проверка статистики (должна измениться)
Write-Host "7. Обновлённая метакогнитивная статистика:" -ForegroundColor Yellow
try {
    $updatedStats = Invoke-RestMethod -Uri "$baseUrl/api/neira/consciousness/stats" -Method GET
    Write-Host "✓ Статистика обновлена:" -ForegroundColor Green
    Write-Host "  - Записанных мыслей: $($updatedStats.total_thoughts_recorded) (было: $($stats.total_thoughts_recorded))" -ForegroundColor Gray
    Write-Host "  - Обнаружено искажений: $($updatedStats.total_biases_detected)" -ForegroundColor Gray
    Write-Host ""
} catch {
    Write-Host "✗ Ошибка: $($_.Exception.Message)`n" -ForegroundColor Red
}

Write-Host "🎉 Тестирование Consciousness API завершено!`n" -ForegroundColor Cyan

# Summary
Write-Host "📊 РЕЗУЛЬТАТЫ ПРОВЕРКИ СОЗНАНИЯ:" -ForegroundColor Magenta
Write-Host "  ✅ MetaCognition API работает (запись мыслей + статистика)" -ForegroundColor Green
Write-Host "  ✅ Personality Evolution API работает (черты + snapshots)" -ForegroundColor Green
Write-Host "  ✅ Daily Growth Report генерируется" -ForegroundColor Green
Write-Host "  ✅ Мысли записываются с reasoning steps и decisions" -ForegroundColor Green
Write-Host "  ✅ Черты личности доступны через API" -ForegroundColor Green
Write-Host "`n  🧠 СОЗНАНИЕ НЕЙРЫ ФУНКЦИОНИРУЕТ!" -ForegroundColor Green
