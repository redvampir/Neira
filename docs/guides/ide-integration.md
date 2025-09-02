# Интеграция IDE

<!-- neira:meta
id: NEI-20250305-ide-cell-runtime
intent: docs
summary: Уточнено, что используется Cell runtime (Node.js).
-->

Этот документ описывает настройку редакторов для работы с проектом Neira.

## VS Code
1. Установите расширения:
   - `rust-analyzer`
   - `ESLint`
   - `Jest` или `vscode-jest`
2. Добавьте файл `.vscode/tasks.json` для запуска тестов и линтеров:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "Rust tests",
      "type": "shell",
      "command": "cargo test",
      "group": "test"
    },
    {
      "label": "TS tests",
      "type": "npm",
      "script": "test",
      "group": "test"
    },
    {
      "label": "Rust lint",
      "type": "shell",
      "command": "cargo clippy",
      "group": "build"
    },
    {
      "label": "TS lint",
      "type": "shell",
      "command": "npx eslint .",
      "group": "build"
    }
  ]
}
```

Выберите нужную задачу в панели *Run Task* для запуска тестов или проверок.

## JetBrains IDE
1. Установите плагины:
   - **Rust**
   - **Cell runtime (Node.js)** и **TypeScript**
   - **ESLint**
2. Создайте конфигурации запуска:
   - **Cargo Command** c `test` для Rust-тестов
   - **npm** со скриптом `test` для тестов на TypeScript
   - **Cargo Command** c `clippy` для проверки Rust-кода
   - **External Tool** с командой `npx eslint .` для линтинга TypeScript
3. Запускайте конфигурации через панель *Run/Debug* IDE.

