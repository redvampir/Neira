# neira_rust

`neira_rust` предоставляет высокопроизводительные компоненты Нейры — парсер тегов, индекс памяти, граф знаний и проверку утверждений — с экспортом в Python через [PyO3](https://pyo3.rs/).

## Требования

- Rust 1.70+
- [maturin](https://www.maturin.rs/) для сборки Python-расширения

## Сборка

```bash
cargo build
maturin develop
```

## Использование из Python

```python
import neira_rust
print(neira_rust.ping())
```

## Документация

Подробнее см. в [общей документации](../../README.md).
