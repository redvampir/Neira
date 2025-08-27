# NodeTemplate

## Навигация
- [Обзор Нейры](README.md)
- [Узлы действий](action-nodes.md)
- [Узлы анализа](analysis-nodes.md)
- [Узлы памяти](memory-nodes.md)
- [Архитектура анализа](analysis-architecture.md)
- [Поддерживающие системы](support-systems.md)
- [Личность Нейры](personality.md)
- [Шаблон узла](node-template.md)
- [Политика источников](source-policy.md)
- [Механизм саморазвивающейся системы](self-updating-system.md)

## Оглавление
- [Обязательные поля](#обязательные-поля)
- [Дополнительные поля](#дополнительные-поля)
- [Расширение metadata](#расширение-metadata)
- [Пример](#пример)
  - [JSON](#json)
  - [YAML](#yaml)
- [Проверка](#проверка)
- [Рекомендации по валидации и обратной совместимости](#рекомендации-по-валидации-и-обратной-совместимости)
- [Примеры и тесты](#примеры-и-тесты)


Шаблон для создания узлов анализа. Обязательными являются поля `id`, `analysis_type` и `metadata`.

## Обязательные поля

| Поле | Тип | Обязательное | Описание |
| --- | --- | --- | --- |
| `id` | string | да | Уникальный идентификатор шаблона. |
| `analysis_type` | string | да | Тип создаваемого узла. |
| `links` | array<string> | нет, по умолчанию `[]` | Список связей с другими узлами. |
| `confidence_threshold` | number | нет | Минимальная допустимая `credibility` для принятия результата. |
| `draft_content` | string | нет | Черновое содержимое узла. |
| `metadata` | object | да | Дополнительные метаданные в формате ключ‑значение. Должно содержать поле `schema`. |

## Дополнительные поля

Поле `metadata` допускает произвольные ключи.

## Расширение metadata

`metadata` может включать дополнительные поля для специфичных нужд. Рекомендуемые ключи:

- `author` — строка с именем автора.
- `tags` — массив строковых тегов.
- `version` — строка с версией содержимого по SemVer.
- `source` — ссылка или путь к исходным данным.

### Правила именования

- используйте `snake_case` в нижнем регистре;
- допускаются только латинские буквы, цифры и подчёркивания;
- не начинайте пользовательские ключи с `schema`.

### Пример кастомного поля

```json
"metadata": {
  "schema": "1.0.0",
  "author": "Alice",
  "dataset_id": "ds-42"
}
```

В коде поле можно получить из `metadata.extra`:

```rust
use backend::node_template::NodeTemplate;

let template: NodeTemplate = serde_json::from_str(json).unwrap();
if let Some(id) = template.metadata.extra.get("dataset_id").and_then(|v| v.as_str()) {
    println!("dataset id: {id}");
}
```

## Пример

### JSON

```json
{
  "id": "example.template",
  "analysis_type": "ProgrammingSyntaxNode",
  "links": ["prog.syntax.base"],
  "confidence_threshold": 0.8,
  "draft_content": "Initial description",
  "metadata": {
    "schema": "1.0.0",
    "source": "https://example.org",
    "author": "Alice",
    "tags": ["demo", "template"],
    "version": "0.1.0"
  }
}
```

### YAML

```yaml
id: example.template
analysis_type: ProgrammingSyntaxNode
links:
  - prog.syntax.base
confidence_threshold: 0.8
draft_content: Initial description
metadata:
  schema: "1.0.0"
  source: "https://example.org"
  author: Alice
  tags:
    - demo
    - template
  version: "0.1.0"
```

## Проверка

Файл можно проверить с помощью JSON Schema. Сохраните шаблон в файл и выполните:

```bash
npx ajv validate -s schemas/v1/node-template.schema.json -d node-template.json
npx ajv validate -s schemas/v1/node-template.schema.json -d node-template.yaml
```

### Программная загрузка

В Rust‑коде схема выбирается на основе поля `metadata.schema`, из которого извлекается мажорная версия. Файлы схем ожидаются в каталоге `schemas/vX/`, путь можно переопределить переменной окружения `NODE_TEMPLATE_SCHEMAS_DIR`. Для явной загрузки по произвольному пути доступна функция `load_schema_from`.

```rust
use backend::node_template::load_schema_from;
use std::path::Path;

let schema = load_schema_from(Path::new("schemas/v1/node-template.schema.json")).unwrap();
```

## Рекомендации по валидации и обратной совместимости

Используйте актуальные JSON Schema для проверки шаблонов. Добавляя новые поля, делайте их необязательными и сохраняйте поддержку предыдущих версий схем. При изменении структуры повышайте номер версии в `metadata.schema` и предоставляйте миграционный путь.

## Примеры и тесты

- Полный пример с дополнительными полями: [tests/example_node_template.rs](tests/example_node_template.rs)
- Тесты на валидацию шаблонов: [tests/node_template_test.rs](tests/node_template_test.rs), [tests/node_template_validation_test.rs](tests/node_template_validation_test.rs)

## Схемы

JSON‑схемы расположены в каталоге [schemas](schemas). Схемы для NodeTemplate хранятся в `schemas/vX/node-template.schema.json`, где `X` — номер мажорной версии. При несовместимых изменениях повышайте версию: `1.0.0` → `2.0.0`.
