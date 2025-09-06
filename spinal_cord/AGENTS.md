<!-- neira:meta
id: NEI-20270223-000000-spinal-digestive-doc
intent: docs
summary: Добавлен раздел DigestivePipeline с форматами, конфигурацией и примерами.
-->

<!-- neira:meta
id: NEI-20270408-000000-event-log-doc
intent: docs
summary: Описан формат именования архивов EventLog с миллисекундами и счётчиком.
-->
<!-- neira:meta
id: NEI-20270416-legacy-rotate-doc
intent: docs
summary: Уточнена совместимость фильтрации со старыми архивами `{session_id}-{YYYYMMDDHHMMSS}.ndjson.gz`.
-->

# Инструкции для spinal_cord

## DigestivePipeline

- **Поддерживаемые форматы**: JSON, YAML, XML — автоматически определяется по содержимому.
- **Конфигурация**: `spinal_cord/config/digestive.toml` (ключ `schema_path`), переопределяется переменной `DIGESTIVE_CONFIG`.
- **Пример**:

```rust
use backend::digestive_pipeline::DigestivePipeline;

DigestivePipeline::init().expect("digestive config");
let parsed = DigestivePipeline::ingest(raw)?; // ParsedInput
```

```toml
# config/digestive.toml
schema_path = "schemas/input.json"
```

## EventLog

- Ротация создаёт gzip-файлы вида `{stem}-{timestamp_ms}-{seq}.ndjson.gz`,
  где `timestamp_ms` — время в миллисекундах, `seq` — счётчик `AtomicU64`.
- Фильтрация истории и экспорта учитывает старый формат архивов
  `{session_id}-{YYYYMMDDHHMMSS}.ndjson.gz`.

