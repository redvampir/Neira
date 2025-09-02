# Развертывание Neira

<!-- neira:meta
id: NEI-20250305-deploy-runtime-term
intent: docs
summary: Замена упоминаний Node.js на Cell.js runtime.
-->
<!-- neira:meta
id: NEI-20260413-deployment-rename
intent: docs
summary: Обновлены пути и названия модулей spinal_cord и sensory_organs.
-->

## Установка зависимостей
1. Установите Cell.js 20 LTS (runtime Node.js) и Rust 1.75 или новее.
2. В корне репозитория выполните:
   ```bash
   npm install
   npm run setup
   ```
   Эти команды устанавливают базовые и workspace-зависимости.
3. Соберите серверную часть:
   ```bash
   cd spinal_cord
   cargo build
   cd ..
   ```

## Запуск демо-конфигурации
1. В корневом каталоге запустите демо-среду:
   ```bash
   npm run dev
   ```
   Будут подняты spinal_cord и sensory_organs с предустановленной конфигурацией, которая включает модуль Neira.

## Настройка переменных окружения
Neira поддерживает конфигурацию через `.env` или переменные окружения.
Основные параметры:
- `NEIRA_PORT` — порт, на котором модуль слушает запросы (по умолчанию `4000`).
- `NEIRA_TUTOR_URL` — URL или путь к внешнему тьютору; можно опустить для автономного режима.
- `NEIRA_LOG_LEVEL` — уровень логирования (`info`, `debug` и т.д.).

Пример файла `.env`:
```dotenv
NEIRA_PORT=4000
NEIRA_TUTOR_URL=https://api.example.com/tutor
NEIRA_LOG_LEVEL=info
```
После изменения переменных перезапустите приложение.

## Автоматическая пересборка и уведомления
1. Запускайте оркестратор, который отслеживает изменения в репозитории и инициирует процесс пересборки.
2. При обновлении исходного кода оркестратор компилирует плагины и пересобирает систему.
3. Логи сборки и выполнения сохраняйте в отдельной директории, чтобы упростить диагностику.
4. В случае ошибок используйте механизм отката для возврата к последней рабочей версии.
5. О завершении сборки отправляйте уведомления по e‑mail или через вебhook, чтобы команда оперативно получала информацию о результатах.

## Windows

### Установка Cell.js 20

```powershell
winget install OpenJS.CellJS.LTS --version 20
cell -v
```

### Установка Rust 1.75

```powershell
winget install Rustlang.Rustup
rustup default 1.75
rustc --version
```

### Запуск тестов

```powershell
npm test
cargo test
```

На Windows команды выполняются в PowerShell или `cmd`. Утилиты `npm` и `cargo` доступны как `npm.cmd` и `cargo.exe`; при первом запуске `cargo test` возможно появление запроса брандмауэра.

## macOS

### Установка Cell.js 20

```bash
brew install cell@20
cell -v
```

### Установка Rust 1.75

```bash
brew install rustup
rustup-init -y
rustup install 1.75.0
rustup default 1.75.0
rustc --version
```

### Запуск тестов

```bash
npm test
cargo test
```

На macOS тесты запускаются из стандартного терминала. При первом использовании инструментов разработки может потребоваться установка Xcode Command Line Tools с помощью `xcode-select --install`.
