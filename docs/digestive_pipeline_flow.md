<!-- neira:meta
id: NEI-20270223-digestive-diagram
intent: docs
summary: Добавлена диаграмма потока данных DigestivePipeline.
-->

# Поток данных DigestivePipeline

```mermaid
flowchart LR
    A[Raw input (JSON/YAML/XML)] --> B[DigestivePipeline]
    B --> C[JSON Schema validation]
    C --> D[ParsedInput]
    D --> E[MemoryCell]
```

DigestivePipeline нормализует вход и сохраняет результат в память, что позволяет
остальным органам работать с унифицированными данными.
