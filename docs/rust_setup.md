# Rust setup

Этот раздел описывает подготовку окружения для сборки и использования компонентов Нейры,
написанных на Rust.

## Установка Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
rustc --version
```

## Установка maturin

`maturin` связывает код Rust с Python. Установите его через pip:

```bash
pip install maturin
```

## Сборка расширения

Перейдите в каталог с исходным кодом и соберите модуль:

```bash
cd rust/neira_rust
maturin develop
```

Команда построит расширение и установит его в активное виртуальное окружение.
После изменения кода Rust повторите `maturin develop`.

## Вызов из Python

После сборки модуль доступен как `neira_rust`:

```python
import neira_rust
print(neira_rust.ping())
```

Также можно использовать классы, реализующие быстрые структуры данных,
например `neira_rust.MemoryIndex`.
