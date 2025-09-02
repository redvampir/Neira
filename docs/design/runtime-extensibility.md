# Runtime Extensibility (Без перекомпиляции)

Цель: позволить Нейре развиваться и менять функционал без пересборки ядра. Две ступени расширений:
- Лёгкий скриптовый слой (Rhai) для «мелкой логики», маршрутизации, простых UI‑инструментов.
- Песочница WASM (WASI) для более тяжёлых модулей с жёсткими лимитами.

Ядро остаётся оркестратором (SynapseHub + ToolRegistry), расширения — горячо загружаемые плагины.

## 1) Scripting (Rhai)
- Формат: `.rhai` файлы в `plugins/scripts/`.
- Назначение: быстрая логика, обработка событий UI, простые трансформации/валидации.
- Безопасность: sandbox API, таймауты, ограничение операций (без диска/сети, кроме предоставленных хостом функций).
- Жизненный цикл: авто‑перезагрузка по watch (inotify), или через API `reload`.

### Манифест (script)
```yaml
id: ui.pencil
version: 0.1.0
kind: script
entry: plugins/scripts/ui_pencil.rhai
permissions:
  scopes: [write]      # требуется для сохранения аннотаций
ui:
  buttons:
    - id: pencil
      title: Карандаш
      icon: pencil
      event: ui.pencil.draw
      params_schema: { color: string?, size: int? }
```

## 2) WASM (WASI)
- Формат: `.wasm` в `plugins/wasm/` + манифест.
- Назначение: CPU‑интенсивная обработка, интеграции.
- Безопасность: ограничение памяти/времени, доступы только через предоставленный ABI.
- Жизненный цикл: регистрация/включение/выключение/перезагрузка через API.

### Манифест (wasm)
```yaml
id: tool.some_heavy
version: 0.1.0
kind: wasm
entry: plugins/wasm/some_heavy.wasm
permissions:
  scopes: [read]
abi:
  function: handle(input_json) -> output_json
```

## 3) Tool Registry & API
- Registry хранит манифесты + состояния (enabled/disabled) на диске (например, `plugins/index.json`).
- Поддерживает горячую перезагрузку.

### API (admin)
- GET `/api/neira/plugins` — список плагинов
- POST `/api/neira/plugins` — регистрация/обновление (манифест + файл/скрипт)
- POST `/api/neira/plugins/:id/enable` | `.../disable` | `.../reload`
- GET `/api/neira/ui/tools` — дескрипторы UI (кнопки/меню)
- POST `/api/neira/ui/events` — события из фронтенда (`{ tool_id, event, params, context }`)

## 4) Интеграция с Hub
- Hub отдаёт фронтенду UI‑дескрипторы активных плагинов.
- События UI транслируются в плагины (script/wasm/process) и возвращают действия: сохранение аннотаций, создание заметок, запуск задач и т.п.
- Сохранение данных — через предоставленные хост‑функции (storage.save_annotation, storage.save_message, …).

## 5) Пример: «Карандаш» (аннотации)
- UI: кнопка `pencil` ➜ фронтенд рисует, отправляет событие `ui.pencil.draw` с координатами/цветом/толщиной.
- Script: `ui_pencil.rhai` принимает событие, проверяет, формирует объект аннотации.
- Host: сохраняет NDJSON в `context/<chat_id>/<session_id>.annotations.ndjson` или как вложение.
- Поиск/экспорт: те же механизмы, что для сообщений; можно добавить фильтр `type=annotation`.

## 6) Безопасность и политики
- Включение/выключение плагинов — admin + feature‑gate.
- Safe‑mode: запрещает enable/modify; write‑операции доступны только admin.
- Лимиты: таймаут, память, квоты I/O.
- Подписи артефактов (опционально) — проверка при регистрации.

## 7) Версионирование и откат
- Хранить N предыдущих версий (скрипт/wasm), быстрый rollback по API.
- Журналирование (JOURNALING.md + neira:meta) для ключевых изменений.

## 8) Capability Gates
- `runtime_scripting_rhai` — experimental
- `runtime_wasm_plugins` — locked
- `ui_tools_registry` — experimental
- `annotations_pencil` — experimental

## 9) Минимальная реализация (этапы)
- E0: регистр плагинов + выдача `GET /api/neira/ui/tools` (без исполнения)
- E1: скриптовые плагины Rhai (events ➜ actions), карандаш‑пример
- E2: горячая перезагрузка скриптов; журнал событий
- E3: песочница WASM (регистрация, выполнение, лимиты)

## 10) Политика «Зачаточного» режима (Embryo)

Чтобы не «перегрузить» Нейру на старте, механизм живого расширения присутствует с нулевого дня,
но включён в безопасной конфигурации и раскрывается по шагам:

- По умолчанию (Stage 0):
  - Развёрнут каркас: каталоги `plugins/scripts/`, `plugins/wasm/`, индекс `plugins/index.json`.
  - Доступен API чтения: `GET /api/neira/plugins`, `GET /api/neira/ui/tools` (читает индекс),
    однако список пустой до явной регистрации.
  - Админ‑эндпоинты регистрации/enable/disable/reload доступны, но **зафичефлажены**:
    требуют admin‑скоуп и соответствующий гейт (`ui_tools_registry`, `runtime_scripting_rhai`).
  - Safe‑mode запрещает любые модификации плагинов (enable/disable/reload/registrations).
  - События UI (`POST /api/neira/ui/events`) принимаются в режиме dry‑run:
    валидируются и журналируются, но **не исполняются**, пока не включён гейт.
  - Все изменения журналируются (JOURNALING.md) и сопровождаются метаданными (neira:meta id).

- Разблокировка (позже, простыми фразами):
  - `ui_tools_registry` → experimental: включить чтение и отдачу UI‑дескрипторов зарегистрированных скриптовых инструментов.
  - `runtime_scripting_rhai` → experimental: разрешить выполнение Rhai‑скриптов на событиях (с лимитами/таймаутами).
  - `runtime_wasm_plugins` остаётся locked до стабильности скриптового слоя.

- Критерии продвижения: см. docs/roadmap.md (Stage 1) и CAPABILITIES.md (сигналы/статусы).

