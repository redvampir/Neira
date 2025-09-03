<!-- neira:meta
id: NEI-20270223-spinal-digestive-doc
intent: docs
summary: Добавлен раздел DigestivePipeline с форматами, конфигурацией и примерами.
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
