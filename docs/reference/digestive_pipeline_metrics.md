<!-- neira:meta
id: NEI-20251101-digestive-metrics-counters-doc
intent: docs
summary: Добавлены/задокументированы счетчики DigestivePipeline: parsed_inputs_total и ошибки парсинга/валидации/фоллбэка.
-->

# DigestivePipeline Metrics (counters)

- `parsed_inputs_total` — increments when a parsed input is stored in MemoryCell.
- `digestive_parse_errors_total` — increments on XML parse failure leading to text fallback.
- `digestive_validation_errors_total` — increments when JSON Schema validation fails.
- `digestive_fallback_schema_used_total` — increments when main schema is missing and fallback schema is used.

See also histograms in docs/reference/metrics.md: `digestive_parse_duration_ms`, `digestive_validation_duration_ms`.

