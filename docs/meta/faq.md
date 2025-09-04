# FAQ

<!-- neira:meta
id: NEI-20250904-120710-faq-cell-runtime
intent: docs
summary: Уточнены требования к окружению с упоминанием Cell runtime.
-->
<!-- neira:meta
id: NEI-20260413-faq-rename
intent: docs
summary: Обновлены инструкции для каталога spinal_cord.
-->

### Как развернуть Neira?
Установите Cell runtime (Node.js 20 LTS) и Rust 1.75+. Затем выполните `npm install`, `npm run setup` и соберите spinal_cord командой `cargo build` внутри каталога `spinal_cord`. Подробности см. в [deployment.md](deployment.md).

### Какие системные требования и ограничения по ресурсам?
Минимум: 4 ядра CPU, 8 ГБ RAM и Cell runtime (Node.js 20 LTS). GPU не требуется, но ускоряет работу. Лимиты по времени и итерациям задаются планировщиком `TaskScheduler`. Подробности см. в [analysis-architecture.md](analysis-architecture.md).

### Как настроить лимиты итераций и времени клеток?
В конфигурации планировщика задайте `max_iterations`, `priority` и `time_slice_ms`. Эти параметры ограничивают циклы обработки и контроль ресурсов. Подробности см. в [analysis-architecture.md](analysis-architecture.md).

### Какие режимы личности поддерживаются?
По умолчанию активен образ 14‑летней девочки, который постепенно взрослеет. Также доступен сухой режим без персонажа. Подробности см. в [personality.md](personality.md).

### Как переключиться в сухой режим без личности?
Отправьте POST-запрос к `/api/neira/personality` с `{"enabled": false}` или используйте UI для временного отключения образа. Подробности см. в [personality.md#переключение-режима-без-личности](personality.md#переключение-режима-без-личности).

### Как подключить или сменить внешнего тьютора?
Укажите переменную `NEIRA_TUTOR_URL` в `.env` или окружении и перезапустите приложение. Если переменная не задана, Neira
работает автономно. Подробности см. в [deployment.md#настройка-переменных-окружения](deployment.md#настройка-переменных-окружения).
