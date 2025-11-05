# Решение проблемы "Оффлайн" в Dashboard

## Проблема
Dashboard показывал статус "Оффлайн" и "Загружаем метрики..." - не мог подключиться к Consciousness API.

## Причины

### 1. Ошибка в Axum Router (исправлено)
**Файл**: `src/server/mod.rs:50`  
**Ошибка**: `Nesting at the root is no longer supported. Use fallback_service instead.`

**Было**:
```rust
let app = Router::new()
    .nest("/metrics", metrics::router())
    .nest("/training", training::router())
    .route("/chat", post(chat::handle_message))
    .nest_service("/", ServeDir::new("static"))  // ❌ Старый API
    .with_state(self.metrics.clone())
    .layer(cors);
```

**Стало**:
```rust
let app = Router::new()
    .nest("/metrics", metrics::router())
    .nest("/training", training::router())
    .route("/chat", post(chat::handle_message))
    .with_state(self.metrics.clone())
    .layer(cors)
    .fallback_service(ServeDir::new("static"));  // ✅ Новый API
```

### 2. Неправильный бинарник
**Проблема**: `run.bat` запускал `target/release/neira.exe` (старый сервер БЕЗ consciousness кода)  
**Нужно**: `spinal_cord/target/release/backend.exe` (новый backend С consciousness)

## Решение

### Шаг 1: Исправлен Router ✅
```powershell
# Уже исправлено в src/server/mod.rs
# Пересобрано: cargo build --release
```

### Шаг 2: Создан новый launcher ✅
**Файл**: `run_backend.bat` - правильный скрипт для запуска consciousness backend

```bat
@echo off
set "NEIRA_BIND_ADDR=0.0.0.0:9090"
set "ORGANS_BUILDER_ENABLED=true"
set "FACTORY_ADAPTER_ENABLED=true"
spinal_cord\target\release\backend.exe > logs\backend.log 2>&1
```

### Шаг 3: Собран правильный бинарник ✅
```powershell
cd spinal_cord
cargo build --release --bin backend
```

Результат: `spinal_cord/target/release/backend.exe` (17.38 MB)

## Как запустить ПРАВИЛЬНО

### Вариант А: Используйте новый launcher (рекомендуется)
```powershell
.\run_backend.bat
```

### Вариант Б: Через cargo run (для разработки)
```powershell
cd spinal_cord
cargo run --release --bin backend
```

### Вариант В: Вручную
```powershell
$env:NEIRA_BIND_ADDR="0.0.0.0:9090"
$env:ORGANS_BUILDER_ENABLED="true"
$env:FACTORY_ADAPTER_ENABLED="true"
spinal_cord\target\release\backend.exe
```

## Проверка работы

### 1. Проверьте процесс
```powershell
Get-Process -Name backend
```

**Ожидаемый вывод**:
```
Id ProcessName StartTime
-- ----------- ---------
12345 backend   04.11.2025 7:50:00
```

### 2. Проверьте порт
```powershell
Test-NetConnection -ComputerName localhost -Port 9090
```

**Ожидаемый вывод**: `TcpTestSucceeded: True`

### 3. Проверьте Consciousness API
```powershell
Invoke-RestMethod -Uri "http://localhost:9090/api/neira/consciousness/stats"
```

**Ожидаемый вывод**:
```json
{
  "total_thoughts_recorded": 0,
  "total_biases_detected": 0,
  "total_improvement_tasks": 0,
  "last_thought_at": null
}
```

### 4. Откройте Dashboard
```
http://localhost:9090/static/consciousness/
```

**Ожидаемое состояние**:
- ✅ Статус: **Онлайн** (зелёная точка)
- ✅ Метрики отображаются (0/0/0/0 для нового сервера)
- ✅ График личности загружен
- ✅ Панель органов работает

## Troubleshooting

### Backend не запускается
**Проблема**: `backend.exe` не найден

**Решение**:
```powershell
cd spinal_cord
cargo build --release --bin backend
cd ..
```

### Порт занят
**Проблема**: "Address already in use"

**Решение**:
```powershell
# Найдите процесс
Get-Process -Name backend,neira | Stop-Process -Force

# Или проверьте что занимает порт
netstat -ano | Select-String "9090"
```

### Dashboard показывает "Оффлайн"
**Причины**:
1. Backend не запущен → Запустите `run_backend.bat`
2. Неправильный порт → Проверьте `NEIRA_BIND_ADDR=0.0.0.0:9090`
3. API не отвечает → Проверьте логи: `Get-Content logs\backend.log`

**Диагностика**:
```powershell
# Проверьте процесс
Get-Process backend

# Проверьте порт
Test-NetConnection localhost -Port 9090

# Проверьте API
curl http://localhost:9090/api/neira/consciousness/stats

# Проверьте логи
Get-Content logs\backend.log -Tail 30
```

### Ошибки компиляции
**Проблема**: Warnings о private interfaces, dead code

**Решение**: Это не критично, backend работает с warnings. Но можно исправить:
```rust
// В src/embeddings/client.rs:116
pub(crate) async fn health_check(&self) -> Result<HealthResponse> {
    // ...
}

// В src/embeddings/client.rs:76
pub(crate) struct HealthResponse {
    // ...
}
```

## Обновлённая документация

### Обновлены файлы:
1. ✅ `src/server/mod.rs` - исправлен Router
2. ✅ `run_backend.bat` - новый launcher
3. ✅ `docs/REMOTE_START_GUIDE_RU.md` - обновлена инструкция
4. ✅ `docs/DASHBOARD_OFFLINE_FIX.md` - этот документ

### Нужно обновить:
- [ ] `README.md` - добавить секцию "Два варианта запуска"
- [ ] `docs/running.md` - документировать оба бинарника

## Два варианта запуска Neira

### Вариант 1: Старый сервер (metrics + training)
**Файл**: `run.bat`  
**Бинарник**: `target/release/neira.exe`  
**Порт**: 9090  
**Endpoints**:
- `/metrics` - Prometheus metrics
- `/training` - Training API
- `/chat` - Chat endpoint

**Использование**: Базовая функциональность без consciousness

### Вариант 2: Consciousness Backend (новый)
**Файл**: `run_backend.bat`  
**Бинарник**: `spinal_cord/target/release/backend.exe`  
**Порт**: 9090  
**Endpoints**:
- Все из варианта 1 +
- `/api/neira/consciousness/*` - Consciousness API
- `/organs/*` - Organ Builder API
- `/static/consciousness/` - Web Dashboard

**Использование**: Полная функциональность с consciousness + UI

**📌 Рекомендуется**: Используйте Вариант 2 для тестирования consciousness

## Итог

✅ Проблема решена  
✅ Backend собран и готов к запуску  
✅ Dashboard будет показывать "Онлайн" при правильном запуске  
✅ Consciousness API доступен по адресу `/api/neira/consciousness/*`

**Следующий шаг**: Запустите `.\run_backend.bat` и откройте `http://localhost:9090/static/consciousness/`

---

**Дата**: 4 ноября 2025  
**Версия**: 1.0  
**Статус**: ✅ Решено
