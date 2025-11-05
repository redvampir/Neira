# Как запустить удалённый сервер Нейры — Пошаговая инструкция

## 🎯 Цель
Настроить Neira так, чтобы можно было подключаться к ней с других устройств в сети (телефон, планшет, другой компьютер).

---

## 📋 Шаг 1: Проверка готовности проекта

### 1.1 Убедитесь, что проект собран
```powershell
# Перейдите в папку проекта
cd F:\Neyra\neira

# Проверьте, существует ли скомпилированный файл
Get-Item "target\release\neira.exe" | Format-Table Name, LastWriteTime

# Если файл старый или отсутствует, пересоберите проект:
cargo build --release
```

**Ожидаемый результат**: Файл `neira.exe` существует и датирован недавно (не старше нескольких дней).

---

## 🌐 Шаг 2: Узнайте IP-адрес вашего компьютера

### 2.1 Найдите локальный IP-адрес
```powershell
# Windows - найти IPv4 адрес
ipconfig | Select-String "IPv4"

# Или более точно:
(Get-NetIPAddress -AddressFamily IPv4 | Where-Object { $_.IPAddress -like "192.168.*" -or $_.IPAddress -like "10.*" }).IPAddress
```

**Пример вывода**: `192.168.1.105` или `10.0.0.15`

**Запомните или запишите этот адрес** — он понадобится для подключения.

---

## 🔧 Шаг 3: Проверка конфигурации сервера

### 3.1 Проверьте файл `run.bat`
```powershell
# Откройте файл для проверки
notepad run.bat
```

**Должны быть такие строки:**
```bat
set "NEIRA_BIND_ADDR=0.0.0.0:9090"
set "ORGANS_BUILDER_ENABLED=true"
set "FACTORY_ADAPTER_ENABLED=true"
```

- `0.0.0.0` — означает "слушать на всех сетевых интерфейсах" (доступно удалённо)
- `9090` — порт сервера
- Если вы видите `127.0.0.1` вместо `0.0.0.0` — **измените на `0.0.0.0`**

### 3.2 Альтернатива: установить переменные вручную
Если не хотите менять `run.bat`, можете задать переменные перед запуском:
```powershell
$env:NEIRA_BIND_ADDR="0.0.0.0:9090"
$env:ORGANS_BUILDER_ENABLED="true"
$env:FACTORY_ADAPTER_ENABLED="true"
```

---

## 🚀 Шаг 4: Запуск сервера

### 4.1 Запустите Neira
```powershell
# Из папки проекта:
.\run.bat
```

**Или запустите в отдельном окне:**
```powershell
Start-Process powershell -ArgumentList "-NoExit", "-Command", "cd '$PWD'; .\run.bat"
```

### 4.2 Проверьте запуск
Подождите 10-15 секунд, затем проверьте:
```powershell
# Проверка порта
Test-NetConnection -ComputerName localhost -Port 9090 -InformationLevel Quiet

# Должно вывести: True
```

### 4.3 Проверьте логи
```powershell
# Последние 20 строк логов
Get-Content "logs\neira.log" -Tail 20
```

**Ищите строки типа:**
- `INFO neira: Starting Neira server...`
- `INFO axum::serve: listening on 0.0.0.0:9090`
- Должны быть логи о запуске consciousness, organ_builder

**⚠️ Если видите ошибки** — смотрите раздел "Troubleshooting" ниже.

---

## 🔍 Шаг 5: Локальное тестирование

### 5.1 Проверьте health endpoint
```powershell
Invoke-RestMethod -Uri "http://localhost:9090/health"
```
**Должно вернуть**: `{"status":"ok"}` или подобное.

### 5.2 Откройте Web Dashboard
Откройте в браузере:
```
http://localhost:9090/static/consciousness/
```

**Что должно отображаться:**
- Статистика (Thoughts, Biases, Organs, Tasks)
- График личности (Personality Evolution Chart)
- Список органов (Organ Growth Monitor)
- Панель записи мыслей (Thought Recording)

### 5.3 Проверьте Consciousness API
```powershell
# Получить статистику
Invoke-RestMethod -Uri "http://localhost:9090/api/neira/consciousness/stats"

# Должно вернуть JSON с counts
```

---

## 🌍 Шаг 6: Удалённое подключение

### 6.1 Настройка брандмауэра Windows

**Вариант А: Через PowerShell (рекомендуется)**
```powershell
# Создать правило для входящих подключений на порт 9090
New-NetFirewallRule -DisplayName "Neira Server" `
  -Direction Inbound `
  -Protocol TCP `
  -LocalPort 9090 `
  -Action Allow `
  -Profile Private,Domain

# Проверить правило создано
Get-NetFirewallRule -DisplayName "Neira Server"
```

**Вариант Б: Через GUI**
1. Нажмите `Win + R`, введите `wf.msc`, Enter
2. Слева выберите "Правила для входящих подключений"
3. Справа нажмите "Создать правило..."
4. Выберите "Для порта" → Далее
5. TCP → Определённые локальные порты: `9090` → Далее
6. Разрешить подключение → Далее
7. Профиль: отметьте "Частный" и "Доменный" → Далее
8. Имя: `Neira Server` → Готово

### 6.2 Проверка с другого устройства

**На телефоне/планшете/другом ПК в той же сети:**

1. **Откройте браузер**
2. **Введите адрес**:
   ```
   http://ВАШ_IP:9090/static/consciousness/
   ```
   Например: `http://192.168.1.105:9090/static/consciousness/`

3. **Должен открыться Dashboard** — так же, как на локальной машине

### 6.3 Тестирование API удалённо
```bash
# С Linux/Mac/другого Windows
curl http://ВАШ_IP:9090/api/neira/consciousness/stats

# Или в PowerShell
Invoke-RestMethod -Uri "http://ВАШ_IP:9090/api/neira/consciousness/stats"
```

---

## 🔒 Шаг 7: Безопасность (важно!)

### 7.1 Текущий режим
⚠️ **Сейчас сервер работает БЕЗ аутентификации** — это удобно для разработки, но **небезопасно** для постоянного использования.

### 7.2 Для домашней сети (краткосрочно)
Если вы используете Neira только дома и на короткий срок — можно оставить как есть.

### 7.3 Для постоянного использования
См. документацию: `docs/remote_access.md` — там описаны:
- Настройка API key аутентификации
- Ограничение CORS
- HTTPS через reverse proxy (Nginx/Caddy)
- Cloudflare Tunnel для безопасного доступа из интернета

---

## 🧪 Шаг 8: Запуск тестов

### 8.1 Комплексное тестирование
```powershell
# Запустить все тесты
.\run_all_tests.ps1
```

Этот скрипт проверит:
- ✅ Consciousness API (stats, thoughts, personality, reports)
- ✅ Organ Builder API (создание, рост, persistence)
- ✅ Метрики и состояние системы

### 8.2 Отдельные тесты
```powershell
# Только органы
.\test_organ_api.ps1

# Только consciousness
.\test_consciousness_api.ps1
```

---

## 📱 Шаг 9: Подключение с телефона

### 9.1 Android
1. Подключитесь к той же Wi-Fi сети, что и компьютер с Neira
2. Откройте Chrome/Firefox
3. Введите: `http://ВАШ_IP:9090/static/consciousness/`
4. Добавьте в закладки для быстрого доступа

### 9.2 iOS
1. Подключитесь к той же Wi-Fi
2. Откройте Safari
3. Введите адрес
4. Нажмите "Поделиться" → "На экран «Домой»" для создания ярлыка

### 9.3 Оптимизация для мобильных
Dashboard уже адаптивный (responsive), но если хотите улучшить — см. файл:
`spinal_cord/static/consciousness/dashboard.css`

---

## 🔄 Шаг 10: Доступ из интернета (опционально)

### Вариант А: Port Forwarding на роутере
1. Войдите в админку роутера (обычно `http://192.168.1.1`)
2. Найдите раздел "Port Forwarding" или "Виртуальный сервер"
3. Добавьте правило:
   - Внешний порт: `9090`
   - Внутренний IP: `ВАШ_IP` (например, 192.168.1.105)
   - Внутренний порт: `9090`
   - Протокол: TCP
4. Узнайте ваш внешний IP: https://whatismyipaddress.com/
5. Подключайтесь: `http://ВАШ_ВНЕШНИЙ_IP:9090/static/consciousness/`

⚠️ **Внимание**: Ваш IP может меняться (динамический). Используйте DDNS сервис (No-IP, DuckDNS).

### Вариант Б: Cloudflare Tunnel (рекомендуется)
```powershell
# Установите cloudflared
# Скачайте: https://github.com/cloudflare/cloudflared/releases

# Создайте туннель
cloudflared tunnel create neira-tunnel

# Запустите туннель
cloudflared tunnel --url http://localhost:9090 run neira-tunnel
```

Теперь доступ через безопасный URL: `https://neira.yourdomain.com`

**Подробнее**: `docs/remote_access.md` → раздел "Cloudflare Tunnel"

---

## ❌ Troubleshooting — Решение проблем

### Проблема 1: "False" при Test-NetConnection
**Причина**: Сервер не запущен или не слушает на 9090.

**Решение**:
```powershell
# Проверьте процесс
Get-Process -Name neira -ErrorAction SilentlyContinue

# Если нет — запустите заново
.\run.bat

# Проверьте логи
Get-Content "logs\neira.log" -Tail 30
```

### Проблема 2: 404 на /api/neira/consciousness/stats
**Причина**: Старый бинарник без consciousness модулей.

**Решение**:
```powershell
# Остановите сервер
Stop-Process -Name neira -Force

# Пересоберите
cargo build --release

# Запустите заново
.\run.bat
```

### Проблема 3: Не подключается с телефона
**Причины**:
- Не в одной сети Wi-Fi
- Брандмауэр блокирует
- Неправильный IP

**Решение**:
```powershell
# Проверьте IP снова
ipconfig | Select-String "IPv4"

# Проверьте брандмауэр
Get-NetFirewallRule -DisplayName "Neira Server"

# Если правила нет — создайте (см. Шаг 6.1)
```

### Проблема 4: Dashboard открывается, но нет данных
**Причина**: API endpoints не отвечают или JavaScript ошибки.

**Решение**:
1. Откройте браузер DevTools (F12)
2. Вкладка "Console" — ищите ошибки
3. Вкладка "Network" — проверьте запросы к API
4. Если 404 — см. Проблему 2

### Проблема 5: Сервер "висит" или медленный
**Решение**:
```powershell
# Проверьте нагрузку
Get-Process -Name neira | Select-Object CPU, WorkingSet

# Если CPU >50% или Memory >500MB долго — перезапустите
Stop-Process -Name neira -Force
.\run.bat
```

---

## 📊 Проверочный чек-лист

Перед тем как считать настройку завершённой, убедитесь:

- [ ] `neira.exe` скомпилирован и актуален (дата не старше сегодня)
- [ ] `run.bat` содержит `NEIRA_BIND_ADDR=0.0.0.0:9090`
- [ ] Сервер запущен и отвечает на `http://localhost:9090/health`
- [ ] Dashboard открывается локально: `http://localhost:9090/static/consciousness/`
- [ ] Dashboard показывает данные (stats, chart, organs)
- [ ] IP-адрес компьютера известен (192.168.x.x или 10.x.x.x)
- [ ] Правило брандмауэра создано для порта 9090
- [ ] Dashboard открывается удалённо с телефона/другого ПК
- [ ] API отвечает удалённо: `http://ВАШ_IP:9090/api/neira/consciousness/stats`
- [ ] Тесты проходят: `.\run_all_tests.ps1` → все зелёные

---

## 🎓 Дополнительные материалы

- **Полная API документация**: `docs/api/consciousness.md`
- **Remote Access (английский)**: `docs/remote_access.md`
- **Тестирование**: `test_organ_growth.md`
- **Архитектура**: `docs/index.md`

---

## 🆘 Поддержка

Если ничего не помогло:
1. Соберите информацию:
   ```powershell
   # Версия Windows
   [Environment]::OSVersion.Version
   
   # Логи Neira
   Get-Content "logs\neira.log" -Tail 50 | Out-File "neira_logs.txt"
   
   # Конфигурация сети
   ipconfig /all > network_config.txt
   ```

2. Создайте Issue на GitHub с файлами `neira_logs.txt` и `network_config.txt`

---

## 📝 Краткая шпаргалка

### Запуск сервера
```powershell
cd F:\Neyra\neira
.\run.bat
```

### Локальный доступ
```
http://localhost:9090/static/consciousness/
```

### Удалённый доступ
```
http://ВАШ_IP:9090/static/consciousness/
```
Где `ВАШ_IP` — результат команды:
```powershell
ipconfig | Select-String "IPv4"
```

### Остановка
```powershell
Stop-Process -Name neira -Force
```

### Тесты
```powershell
.\run_all_tests.ps1
```

---

**Версия**: 1.0  
**Дата**: 4 ноября 2025  
**Статус**: ✅ Актуально

**🚀 Успешного запуска Neira!**
