# Neira — Быстрый старт (точка входа)

Ниже — компактная заметка для вставки в обсуждение/issue и как "точка старта" после перезапуска VS Code. Откройте этот файл после перезапуска — отсюда будем продолжать.

## Краткая сводка (текущее состояние)
- **Фаза 1 (Homeostasis)** — ✅ завершена (интегрирован `HomeostasisEngine`, метрики, endpoint `/api/neira/homeostasis/status`).
- **Фаза 2 (Memory)** — ✅ завершена (SemanticMemory, EvolvingDialogue, GrowthTracker, ReflectionJournal).
- **Фаза 3 (Consciousness)** — 🚧 В РАБОТЕ:
  - ✅ Task #1 (MetaCognition Engine) — **завершён** (80%):
    - Создан модуль `spinal_cord/src/consciousness/` с `metacognition.rs` (430+ строк)
    - `MetaCognitionEngine` реализован: record_thought_trace, analyze_thought_process (обнаружение bias), generate_improvement_tasks
    - Интегрирован в `EvolvingDialogue` — добавлен **9-й шаг (META-REFLECT)**: запись reasoning_steps, decisions, анализ bias
    - Полный цикл теперь: recall → learn → understand → generate → assess → remember → reflect → grow → **meta-reflect**
    - Поддерживает 5 типов bias: ConfirmationBias, AnchoringBias, AvailabilityBias, OverconfidenceBias, ContextIgnorance
    - Метрики: metacognition_thoughts_recorded, biases_detected, tasks_completed, improvement_tasks_pending
  - ⏳ Task #2 (Auto-Improvement Loop) — не начата
  - ⏳ Task #3 (Personality Evolution) — не начата
  - ⏳ Task #4 (Daily Growth Report) — не начата
  - ⏳ Task #5 (API endpoints для consciousness) — не начата
  - ⏳ Task #6 (Тесты и документация) — не начата

- Backend (crate `spinal_cord`) компилируется успешно (warnings only, no errors).
- Добавлен mock embeddings сервис для быстрой локальной разработки: `sensory_organs/embeddings_service/mock_app.py`.
- Production embeddings сервис обновлён для быстрого старта с лёгкой моделью `all-MiniLM-L6-v2`: `sensory_organs/embeddings_service/app.py`.
- Тесты: `tests/dialogue_integration_tests.rs` (часть тестов требует работающего embeddings сервиса).

## Ключевые файлы (чтобы быстро сориентироваться)
- Архитектура / основной код:
  - `spinal_cord/src/main.rs` — AppState, маршруты, инициализация
  - `spinal_cord/src/dialogue/evolving_dialogue.rs` — EvolvingDialogue (**9-шаговый цикл** с мета-рефлексией)
  - `spinal_cord/src/dialogue/mod.rs`
  - `spinal_cord/src/consciousness/metacognition.rs` — MetaCognitionEngine (саморефлексия, bias detection, improvement tasks)
  - `spinal_cord/src/consciousness/mod.rs` — экспорт consciousness модуля
  - `spinal_cord/src/memory/semantic.rs` — SemanticMemory
  - `spinal_cord/src/embeddings/client.rs` — EmbeddingsClient

- Embeddings service:
  - Prod: `sensory_organs/embeddings_service/app.py`
  - Mock: `sensory_organs/embeddings_service/mock_app.py`
  - Requirements: `sensory_organs/embeddings_service/requirements.txt`

- Тесты:
  - `tests/dialogue_integration_tests.rs`

- Документы:
  - `docs/AWAKENING_PLAN.md` — план (обновлён)
  - `docs/PHASE2_COMPLETION.md` — отчёт по Фазе 2

## Быстрый старт после перезапуска VS Code (PowerShell)
1) Откройте проект в VS Code и перейдите в корень репозитория: `F:\Neyra\neira`.

2) (рекомендуется) Запустить mock embeddings service для быстрых тестов:

```powershell
Set-Location "F:\Neyra\neira\sensory_organs\embeddings_service"
# Если ещё нет venv:
python -m venv venv
# Установка зависимостей (вызывайте venv python напрямую):
.\venv\Scripts\python.exe -m pip install --upgrade pip setuptools wheel
.\venv\Scripts\python.exe -m pip install -r requirements.txt

# Запуск mock сервиса (в фоне через Job)
Start-Job -ScriptBlock { & "F:\Neyra\neira\sensory_organs\embeddings_service\venv\Scripts\python.exe" "F:\Neyra\neira\sensory_organs\embeddings_service\mock_app.py" }
# Проверка health
Invoke-RestMethod -Uri "http://localhost:8765/health"
```

3) Запустить backend (в другом терминале):

```powershell
Set-Location "F:\Neyra\neira\spinal_cord"
cargo build
cargo run
# Backend слушает порт, как настроено в main.rs (обычно 3000).
```

4) Примеры запросов (PowerShell):

```powershell
# Отправить сообщение к эволюционирующему диалогу
Invoke-RestMethod -Uri "http://localhost:3000/api/neira/dialogue/evolving" -Method POST -Body (@{ text="Привет, Нейра!"; session_id="test1" } | ConvertTo-Json) -ContentType "application/json"

# Получить статистику роста
Invoke-RestMethod -Uri "http://localhost:3000/api/neira/dialogue/stats"
```

5) Запуск тестов (локально):

```powershell
Set-Location "F:\Neyra\neira\spinal_cord"
# Запуск всех тестов
cargo test

# Или конкретный файл тестов (пример):
cargo test --test dialogue_integration_tests -- --nocapture
```

> Примечание: четыре интеграционных теста помечены как требующие работающего embeddings сервиса — перед их запуском убедитесь, что mock или prod embeddings поднят.

## Что было сделано в последней итерации (коротко)
- Перенёс `evolving_dialogue_v2.rs` → `spinal_cord/src/dialogue/evolving_dialogue.rs` и подключил как модуль.
- Интегрировал `SemanticMemory` и `EmbeddingsClient` в `AppState`.
- Добавил handlers и routes: `POST /api/neira/dialogue/evolving` и `GET /api/neira/dialogue/stats`.
- Создал `mock_app.py` для быстрых тестов и обновил `app.py` (prod) на лёгкую модель.
- Написал интеграционные тесты: `tests/dialogue_integration_tests.rs`.
- Обновил документацию: `docs/AWAKENING_PLAN.md` и `docs/PHASE2_COMPLETION.md`.

## Риски и замечания
- Продакшен-модель `intfloat/multilingual-e5-large` большая — загрузка может занять много времени/трафика; для локальной разработки используйте `mock_app.py` или `all-MiniLM-L6-v2`.
- На Windows PowerShell может быть политика выполнения скриптов; запускайте venv python напрямую через `.\venv\Scripts\python.exe`.
- Если столкнётесь с ошибками сборки — пришлите лог (последние 50 строк `cargo build`), я быстро проанализирую.

## Что дальше (предложения)
- После перезапуска VS Code выполните шаги выше и скажите, готовы ли вы, чтобы я запустил интеграционные тесты и собрал отчёт.
- Далее могу приступить к Фазе 3 (MetaCognition): разметить подзадачи и сделать быстрый прототип.
- Могу настроить CI, чтобы тесты запускались автоматически с mock embeddings.

---

Файл сгенерирован автоматически — откройте `START_HERE.md` в корне репозитория после перезапуска VS Code и напишите "Готов" — я подхвачу оттуда и продолжу.