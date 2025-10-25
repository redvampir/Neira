# Документация по обучению (русский)

<!-- neira:meta
id: NEI-20270318-120050-training-orchestrator-doc
intent: docs
summary: |-
  Добавлен раздел про TrainingOrchestrator: анти-айдл автозапуск, гейты и
  переменные окружения.
-->
<!-- neira:meta
id: NEI-20280401-120010-russian-curriculum-doc
intent: docs
summary: Описан учебный курс по русскому алфавиту и способ его загрузки.
-->
<!-- neira:meta
id: NEI-20280402-120000-russian-vocabulary-seed-doc
intent: docs
summary: Добавлено описание базового словаря вопросов и события vocabulary_seeded.
-->

<!-- neira:meta
id: NEI-20260413-training-rename
intent: docs
summary: Обновлены пути на spinal_cord/.
-->
<!-- neira:meta
id: NEI-20270318-120050-training-orchestrator-doc
intent: docs
summary: |-
  Добавлен раздел про TrainingOrchestrator: анти-айдл автозапуск, гейты и
  переменные окружения.
-->

## Назначение

«Сценарный узел обучения» позволяет запускать заранее описанные тест‑сценарии обучения/валидации Neira без риска галлюцинаций: сценарии задают чёткие шаги (HTTP‑запросы), ожидания и проверки. Результаты сохраняются в историю (NDJSON), собирается прогресс, формируется JUnit/HTML отчёт.

## Где лежит код

- Клетка: `spinal_cord/src/action/scripted_training_cell.rs`
- Роуты API/стрима: `spinal_cord/src/http/training_routes.rs`
- Инициализация: `spinal_cord/src/main.rs`
- Оркестратор: `spinal_cord/src/training/orchestrator.rs`
- Пример сценария: `examples/training_script.yaml`
- История и отчёты: `CONTEXT_DIR` (по умолчанию `context/`), файлы в `context/training/`
- Учебные данные по русской грамоте: `spinal_cord/static/training/russian_literacy.json`
- Загрузчик курса: `spinal_cord/src/training/curriculum.rs`
- Метод загрузки в память: `SynapseHub::train_russian_literacy`

## Формат сценария (YAML)

Корень:
- `name`: строка
- `vars`: карта ключ→значение (базовые переменные сценария)
- `steps`: список шагов

Шаг:
- `method`: `GET` | `POST` | `PUT` | `PATCH` | `DELETE` (по умолчанию `GET`)
- `url`: строка (поддержка `${VAR}`, `${FILE:/path}`)
- `headers`: карта заголовков (значения тоже с подстановкой переменных)
- `body`: JSON (любой), строки внутри — тоже с подстановкой
- `expect_status`: ожидаемый HTTP‑статус
- `expect_contains`: ожидаемая подстрока в тексте ответа
- `assertions`: список проверок JSONPath
  - `path`: JSONPath (например, `$.json.price`)
  - `equals` | `contains` | `gt` | `lt`: один или несколько предикатов
- `dataset`: массив объектов — параметризация шага; поля из объекта доступны как `${key}`
- `timeout_ms`: таймаут шага
- `pre_hook` | `post_hook`: хук (`sleep_ms` | `set_env` | `shell`), `shell` работает только при `TRAINING_ALLOW_SHELL=true` и не в `dry_run`
- `retry`: `{ attempts, backoff_ms }`: повторы с задержкой

## Подстановки

- `${VAR}`: берётся из `vars` или окружения (env)
- `${FILE:/path}` / `${VAR_FILE:/path}`: читается содержимое файла (без переноса)
- Подстановки работают в `url`/`headers`/`body`/`expect_contains`; в JSON — рекурсивно для строковых значений.

## Запуск

- UI: `http://127.0.0.1:3000/training`, кнопка `Run`
- Из чата: сообщение `train script="examples/training_script.yaml" dry=true`
- Через API: `POST /api/neira/training/run` `{ script, dry_run }`

## Прогресс и история

- Прогресс в `TRAINING_PROGRESS` (по умолчанию `context/training/progress.json`), поле `last_completed`.
- История шагов — NDJSON в `context/training/run-YYYYMMDD.ndjson`
- SSE‑стрим: `/api/neira/training/stream` — можно читать на UI или подключить `EventSource`.

## Отчёты

- JUnit XML: `context/training/report.xml`
- HTML: `context/training/report.html` (есть ссылки на snippets при падениях)

## Переменные окружения

- `TRAINING_SCRIPT`: путь к сценарию (по умолчанию `examples/training_script.yaml`)
- `TRAINING_PROGRESS`: путь к файлу прогресса (`context/training/progress.json`)
- `TRAINING_DRY_RUN`: `true|false` — «сухой» прогон, без внешних вызовов и шелл‑хуков
- `TRAINING_ALLOW_SHELL`: `true|false` — разрешить shell‑хуки
- `TRAINING_INTERVAL_MS`: периодический автозапуск (мс)
- `LEARNING_MICROTASKS_ENABLED`: включить очередь учебных микрозадач (Anti-Idle)
- `TRAINING_PIPELINE_ENABLED`: разрешить scripted training (ручной + авто)
- `TRAINING_AUTORUN_ENABLED`: активировать TrainingOrchestrator
- `TRAINING_AUTORUN_INTERVAL_MINUTES`: минимум минут между автозапусками
- `TRAINING_AUTORUN_MIN_IDLE_STATE`: минимальное состояние простоя (1=short,2=long,3=deep)
- `TRAINING_AUTORUN_MAX_FAILURES`: остановить автозапуск после N ошибок подряд
- `CONTEXT_DIR`: директория хранения истории/отчётов

## Оркестратор обучения

- TrainingOrchestrator регистрируется в AntiIdleMicrotaskService и стартует при `LEARNING_MICROTASKS_ENABLED && TRAINING_PIPELINE_ENABLED && TRAINING_AUTORUN_ENABLED`.
- Учитывает состояние простоя (`TRAINING_AUTORUN_MIN_IDLE_STATE`), кулдаун (`TRAINING_AUTORUN_INTERVAL_MINUTES`) и лимит ошибок (`TRAINING_AUTORUN_MAX_FAILURES`).
- Метрики: `auto_tasks_*{task="training.orchestrator"}`, `training_*{mode="auto"}`.
- Статус и очередь видны в `/api/neira/introspection/status` → `anti_idle.microtasks`.

## Учебный курс «Русская грамота»

- **Назначение**: даёт Нейре базовый набор букв, слогов и 100 простых слов
  для начального словаря.
- **Файл**: `spinal_cord/static/training/russian_literacy.json` — описание алфавита,
  слогов и словарного запаса в формате JSON.
- **Валидация**: при загрузке проверяется уникальность букв, наличие всех слогов в словах
  и ограничение словаря (≤ 100 слов).
- **Загрузка**:
  ```rust
  use backend::synapse_hub::SynapseHub;

  // hub: Arc<SynapseHub>
  let result = hub.train_russian_literacy(None)?;
  ```
  Путь можно переопределить через `train_russian_literacy(Some(path))`.
- **Сохранение**: курс сохраняется в `MemoryCell` как `ParsedInput::Json`,
  поэтому доступен анализатору и клеткам памяти без дополнительных преобразований.
- **События**: после загрузки публикуется событие `training.curriculum.loaded`
  с количеством букв, слогов и слов. Его можно отследить через EventBus или
  `logs/events.ndjson`.
- **Базовый словарь вопросов**: `SynapseHub::train_russian_literacy` формирует
  отдельный JSON с 10–30 простыми словами (темы «семья», «жильё», «еда»,
  «город», «природа») и сохраняет его в `MemoryCell` с полем
  `"purpose": "inquiry_vocabulary"`. Это позволяет Нейре быстро задавать
  уточняющие вопросы вроде «Что это такое?» или «Где это находится?».
- **Дополнительное событие**: при активации базового словаря публикуется
  `training.curriculum.vocabulary_seeded` с перечнем слов и назначением.
- **Проверка**: для автоматического теста см. `spinal_cord/tests/training_curriculum_test.rs`.

## Лучшие практики

- Хранить секреты в файлах и подставлять через `${FILE:/secret/token}`, а не в `.env`.
- Для нестабильных эндпоинтов добавлять `retry` и увеличивать `timeout_ms`.
- Все проверяемые значения оформлять `assertions` с JSONPath — это уменьшает ложные «зелёные».
- Использовать `dataset` для многократных прогонов одного шага с разными входными данными.
- Включать `dry_run` при отладке сценария (быстро проверять подстановки/структуру).
- Для автообучения выставить `TRAINING_INTERVAL_MS` и наблюдать стрим/отчёты.

