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
- [Пример](#пример)
  - [JSON](#json)
  - [YAML](#yaml)
- [Проверка](#проверка)


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
    "schema": "1.1.0",
    "source": "https://example.org"
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
  schema: "1.1.0"
  source: "https://example.org"
```

## Проверка

Файл можно проверить с помощью JSON Schema. Сохраните шаблон в файл и выполните:

```bash
npx ajv validate -s schemas/node-template/v1.0.0.json -d node-template.json
npx ajv validate -s schemas/node-template/v1.0.0.json -d node-template.yaml
```

### Программная загрузка

В Rust‑коде схема загружается функцией `load_schema(version)`, которая читает файл по указанной версии. Каталог с версиями можно переопределить переменной окружения `NODE_TEMPLATE_SCHEMA_DIR`; по умолчанию используется `schemas/node-template` из текущего репозитория. Для явной загрузки по произвольному пути доступна функция `load_schema_from`.

```rust
use backend::node_template::{load_schema, load_schema_from};
use std::path::Path;

let schema = load_schema("1.0.0").unwrap();
let same_schema = load_schema_from(Path::new("schemas/node-template/v1.0.0.json")).unwrap();
```

## Схемы

JSON‑схемы расположены в каталоге [schemas](schemas). Схемы для NodeTemplate: `schemas/node-template/vX.Y.Z.json`. При несовместимых изменениях повышайте версию: `1.0.0` → `1.1.0`.
