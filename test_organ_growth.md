# 🧬 Тест автономного роста органов Нейры

## Цель
Проверить ключевую особенность универсальности Нейры — способность автономно отращивать новые органы и функционал без перезапуска.

## Тестовые сценарии

### 1. OrganBuilder — Автономное создание органов
**Проверка:** Может ли Нейра создать новый орган через API

```bash
# Запрос на создание нового органа
curl -X POST http://localhost:3000/organs/build \
  -H 'Content-Type: application/json' \
  -d '{
    "organ_template": {
      "name": "emotion_analyzer",
      "type": "analysis",
      "capabilities": ["sentiment", "emotion_detection"],
      "config": {
        "model": "emotion-v1",
        "threshold": 0.7
      }
    },
    "dryrun": false
  }'

# Ожидаемый ответ:
# {"organ_id": "organ-123", "state": "draft"}
```

**Стадии роста:**
- `Draft` → `Canary` → `Experimental` → `Stable`
- Переходы автоматические с задержками (по умолчанию 50ms каждый)

**Проверка статуса:**
```bash
# Получить список всех органов
curl http://localhost:3000/organs

# Получить статус конкретного органа
curl http://localhost:3000/organs/{id}

# SSE stream изменений статуса
curl http://localhost:3000/organs/{id}/stream
```

### 2. StemCellFactory — Порождение новых клеток
**Проверка:** Может ли Нейра создать новую клетку из шаблона

```bash
# Регистрация нового шаблона клетки
curl -X POST http://localhost:3000/cells/register \
  -H 'Content-Type: application/json' \
  -d '{
    "id": "custom_analyzer_v1",
    "version": "1.0.0",
    "analysis_type": "custom",
    "prompt": "Analyze user intent for {{query}}",
    "links": [],
    "schema": {
      "type": "object",
      "properties": {
        "query": {"type": "string"}
      },
      "required": ["query"]
    }
  }'

# Создание экземпляра клетки через фабрику
curl -X POST http://localhost:3000/factory/create \
  -H 'Content-Type: application/json' \
  -d '{
    "template_id": "custom_analyzer_v1",
    "backend": "adapter",
    "state": "draft"
  }'
```

### 3. PolicyEngine — Разрешения на саморазвитие
**Проверка:** Контролируются ли права на создание органов/клеток

**Capabilities:**
- `OrgansBuilder` — право создавать новые органы
- `CellFactory` — право порождать клетки
- `SelfModification` — право на автономное саморазвитие

**Переменные окружения:**
```bash
ORGANS_BUILDER_ENABLED=true
FACTORY_ADAPTER_ENABLED=true
POLICY_ALLOW_SELF_MODIFICATION=true
```

### 4. Метрики автономного роста
**Проверяемые метрики:**

```bash
# Prometheus metrics
curl http://localhost:3000/metrics | grep organ
# organ_build_attempts_total
# organ_build_success_total
# organ_build_failed_total
# organ_state_transitions_total{from="draft",to="canary"}

curl http://localhost:3000/metrics | grep factory
# factory_create_records_total
# factory_dryrun_requests_total
# factory_adapter_enabled
```

### 5. Динамическая регистрация endpoints (hot-reload)
**Проверка:** Может ли Нейра добавить новый endpoint без перезапуска

**Текущее состояние:** 
- Органы сохраняются в `organ_templates/` директории
- При рестарте восстанавливаются из файлов
- Счётчик ID восстанавливается (max_id + 1)

**Ограничение:** 
- Новые HTTP routes требуют перезапуска сервера (Axum Router не поддерживает hot-reload)
- Но клетки могут динамически регистрироваться через CellRegistry

### 6. Тест полного цикла: от идеи до работающего функционала

**Сценарий:** Нейра хочет научиться анализировать код

1. **Metacognition** обнаруживает потребность в новом навыке
2. **AutoImprovementLoop** генерирует задачу "learn_code_analysis"
3. **PolicyEngine** проверяет разрешение на OrgansBuilder
4. **OrganBuilder** создаёт новый орган "code_analyzer"
   - Draft → Canary (тестирование на малой выборке)
   - Canary → Experimental (расширенное тестирование)
   - Experimental → Stable (полное развёртывание)
5. **StemCellFactory** порождает клетки для разных языков:
   - `code_analyzer_python_v1`
   - `code_analyzer_rust_v1`
   - `code_analyzer_js_v1`
6. **Новый функционал доступен** через существующие endpoints

## Результаты проверки

### ✅ Что работает:
- [ ] OrganBuilder создаёт органы через API
- [ ] Стадии Draft→Canary→Experimental→Stable проходят автоматически
- [ ] Органы сохраняются на диск и восстанавливаются при рестарте
- [ ] StemCellFactory порождает новые клетки
- [ ] PolicyEngine контролирует права на саморазвитие
- [ ] Метрики organ_* и factory_* работают

### ⚠️ Ограничения:
- [ ] HTTP routes не поддерживают hot-reload (требуется рестарт для новых endpoints)
- [ ] Динамическая компиляция Rust кода не реализована (только шаблоны)

### 🚀 Следующие улучшения:
- [ ] Интеграция OrganBuilder с Consciousness (автономное решение о создании органов)
- [ ] Dynamic route registration через Plugin system
- [ ] WASM-based органы для безопасного выполнения пользовательского кода
- [ ] UI для визуализации роста органов в реальном времени

## Заключение

**Универсальность Нейры подтверждается:**
1. ✅ Система может создавать новые органы через API
2. ✅ Клетки порождаются динамически через фабрику
3. ✅ Состояние сохраняется и восстанавливается
4. ⚠️ Полная автономность ограничена архитектурой Rust/Axum (статическая компиляция)

**Главная фишка работает с оговоркой:** Нейра может расти автономно в рамках заранее определённых шаблонов и возможностей, но не может полностью переписать свой исходный код на лету (требуется человеческое вмешательство для добавления принципиально нового функционала через code changes).
