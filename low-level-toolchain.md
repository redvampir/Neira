# Низкоуровневая цепочка инструментов

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
- [Низкоуровневая цепочка инструментов](low-level-toolchain.md)

## Оглавление

- [Общий процесс](#общий-процесс)
- [Основные инструменты](#основные-инструменты)
- [Полезные команды (Rust/Cargo)](#полезные-команды-rustcargo)
- [Рекомендации по расширению под Neira](#рекомендации-по-расширению-под-neira)
- [Как применить](#как-применить)

## Общий процесс

1. **Исходный код** — текстовые файлы на Rust (`*.rs`) или ассемблере (`*.asm`).
2. **Компиляция / ассемблирование**
   - для Rust: `rustc` преобразует исходники в объектные файлы;
   - для ассемблера: `nasm` или `gas` собирают `.asm` в объектные файлы.
3. **Линковка** — линкер (`lld` или системный `ld`) объединяет объектные файлы и библиотеки, создавая исполняемый файл (`.exe`, `.elf`, `.so`).
4. **Среда выполнения** — загрузчик ОС подгружает бинарник и при необходимости динамические библиотеки (стандартная библиотека, рантайм).
5. **Запуск** — управление передаётся функции `_start`/`main`, после чего CPU выполняет инструкции программы.

## Основные инструменты

| Инструмент | Описание | Документация | Исходники |
| --- | --- | --- | --- |
| `rustc` | Компилятор языка Rust | [doc.rust-lang.org/rustc](https://doc.rust-lang.org/rustc/) | [github.com/rust-lang/rust](https://github.com/rust-lang/rust) |
| `Cargo` | Сборка, управление зависимостями и тесты | [doc.rust-lang.org/cargo](https://doc.rust-lang.org/cargo/) | [github.com/rust-lang/cargo](https://github.com/rust-lang/cargo) |
| `LLVM` | Backend генерации кода, оптимизаторы, линкер `lld` | [llvm.org/docs](https://llvm.org/docs/) | [github.com/llvm/llvm-project](https://github.com/llvm/llvm-project) |
| `NASM` | Ассемблер для x86/x86_64 | [nasm.us/doc](https://www.nasm.us/doc/) | [github.com/netwide-assembler/nasm](https://github.com/netwide-assembler/nasm) |
| `GAS` | Ассемблер для Unix-платформ | [sourceware.org/binutils/docs/as](https://sourceware.org/binutils/docs/as/) | [sourceware.org/git/binutils-gdb.git](https://sourceware.org/git/binutils-gdb.git) |
| `glibc` / `musl` | Стандартные библиотеки и рантайм-функции | glibc: [gnu.org/software/libc/manual](https://www.gnu.org/software/libc/manual/) <br> musl: [musl.libc.org/docs](https://musl.libc.org/docs.html) | glibc: [sourceware.org/git/glibc.git](https://sourceware.org/git/glibc.git) <br> musl: [musl.libc.org/git/musl](https://musl.libc.org/git/musl) |

## Полезные команды (Rust/Cargo)

```bash
# Сборка с инкрементальной компиляцией
CARGO_INCREMENTAL=1 cargo build

# Генерация документации
cargo doc --open

# Проверка и запуск тестов
cargo test
```

## Рекомендации по расширению под Neira

- **Инкрементальная сборка** — использовать `CARGO_INCREMENTAL=1` и отслеживать изменения файлов, чтобы перекомпилировать только затронутые модули.
- **Логирование** — подключить `tracing` или `log` + `env_logger` для записи всех стадий сборки и возможных ошибок.
- **Версионирование и откаты** — хранить несколько проверенных версий компилятора/бинарника; при проблемах откатываться на предыдущую стабильную.
- **Автоматизация** — создать свой cargo-плагин (например, `cargo neira`) для автоматических сборок, тестов и переключения версий.

## Как применить

1. Склонировать исходные репозитории инструментов, при необходимости модифицировать (например, `rustc` или `cargo`) и собрать.
2. Настроить систему автоматизации (плагин или отдельный скрипт), чтобы Neira могла пересобирать себя самостоятельно.
3. Расширять функциональность логирования и механизмов отката по мере роста системы.
