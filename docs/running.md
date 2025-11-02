<!-- neira:meta
id: NEI-20251101-120600-training-pipeline-doc
intent: docs
summary: |
  Обновлено руководство по запуску: добавлен графический лаунчер и уточнён удалённый сценарий.
-->

# Запуск Neira

## Переменные окружения

```bash
export NEIRA_DATA_DIR="./data"
export NEIRA_SUCCESS_THRESHOLD=0.8
export NEIRA_FAILURE_THRESHOLD=0.6
```

## Основные компоненты обучения

- `LearningMetrics` сохраняет прогресс в `${NEIRA_DATA_DIR}/learning_progress.json` после вызова `complete_lesson`.
- `AdaptiveScheduler` подбирает сложность заданий и пересчитывает план после каждого `complete_lesson`.
- `RussianLiteracyCurriculum` содержит набор уроков и загружается через `load_default`.

```rust
use std::sync::Arc;
use spinal_cord::training::{
    curriculum::RussianLiteracyCurriculum,
    metrics::LearningMetrics,
    scheduler::AdaptiveScheduler,
};

let metrics = Arc::new(LearningMetrics::new());
let curriculum = Arc::new(RussianLiteracyCurriculum::load_default()?);
let scheduler = AdaptiveScheduler::new(curriculum, metrics.clone());

let lesson = scheduler.next_lesson().await;
scheduler.complete_lesson(&lesson, 0.72).await?;
```

`NEIRA_SUCCESS_THRESHOLD`, `NEIRA_FAILURE_THRESHOLD` и `NEIRA_MIN_ATTEMPTS` регулируют пороги принятия решений.

## Быстрый запуск из консоли

```bash
cargo run --release

curl http://localhost:9090/metrics
```

## Графический лаунчер (Windows)

В репозитории добавлен лаунчер `tools/launcher/neira_launcher.ps1`, который предоставляет простое окно с кнопками запуска:

1. Создайте ярлык на рабочем столе с командой  
   `powershell.exe -WindowStyle Hidden -ExecutionPolicy Bypass -File "F:\Neyra\neira\tools\launcher\neira_launcher.ps1"`  
   (замените путь на актуальный).
2. Выберите в окне режим:
   - «Для одного (127.0.0.1)» — сервер доступен только на этом компьютере.
   - «Для нескольких (0.0.0.0)» — можно подключаться по внешнему IP/VPN (например, через Tailscale).
3. Нажмите «Запустить». Лаунчер автоматически выполнит `cargo build --release`, поднимет сервер и покажет ссылки:
   - локальный: `http://localhost:9090`
   - удалённый: подсказка по Tailscale или напоминание подставить свой IP.
4. Кнопки «Остановить», «Открыть интерфейс», «Открыть лог» управляют текущим процессом.

При закрытии окна лаунчер автоматически завершает сервер.

## Удалённый доступ (через Tailscale)

1. Установите Tailscale на компьютер с Neira и на устройство, с которого будете подключаться. Авторизуйтесь в одном tailnet.
2. В лаунчере выберите режим «Для нескольких (0.0.0.0)» или вручную установите `NEIRA_BIND_ADDR=0.0.0.0:9090`.
3. Убедитесь, что порт 9090 разрешён в брандмауэре Windows.
4. На другом устройстве откройте `http://<tailscale-ip>:9090`. Адрес можно получить командой `tailscale ip -4`.
5. После завершения работы вернитесь в локальный режим или выключите туннель, чтобы закрыть доступ снаружи.

Этот сценарий не требует проброса портов в интернет и остаётся доступным только участникам tailnet.

## Интеграция с Prometheus

```yaml
scrape_configs:
  - job_name: 'neira'
    static_configs:
      - targets: ['localhost:9090']
```
