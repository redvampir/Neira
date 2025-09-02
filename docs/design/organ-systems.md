# Organ Systems (Сенсоры/Эффекторы) для Нейры

Идея: собрать «органы» как связки клеток (sensors → processing → policy → actuators), с жёсткими гейтами, безопасными дефолтами и прозрачным аудитом.

Принципы
- Consent‑first: любые органы, затрагивающие приватность/управление, включаются только с явного согласия.
- Safe‑mode: запрещает эффекторы (motor), ограничивает сенсоры до read‑only/локальных.
- Gate‑based: каждый орган имеет capability gate (locked → experimental → stable).
- Audit: все включения/действия — в журнал + метрики.
- Sandbox: минимальные привилегии, белые списки (окна/приложения/каталоги), таймауты/квоты.

Таксономия
- Sensors (внешняя среда):
  - Vision (экран): ScreenCaptureCell → OCR/DetectorCell → RedactionCell
  - Hearing (микрофон): AudioCaptureCell → STTCell → Diarization/Redaction
  - FS Watch: FsObserverCell → Indexer/Classifier
  - Net Probe: HttpProbeCell (head/get) → SafetyPolicy
- Internals (внутренние):
  - Proprioception: CapabilityProbes (CPU/Mem/IO/Net)
  - Interoception: Runtime health (latency/errors/queues)
- Effectors:
  - Motor: InputControlCell (mouse/keyboard) + Replay/Script
  - Voice: TTSCell
  - FileOps: SafeWriterCell (whitelist paths)

Органы (предложения)
- Vision System (орган зрения)
  - Клетки: ScreenCapture → OCR → UIContext → Redaction
  - Гейт: organ_vision_readonly (experimental), organ_vision_active (locked)
  - Safe‑mode: только readonly, захват ограничен в окнах белого списка
  - Метрики: vision_frames_per_min, vision_ocr_latency_ms, vision_redactions_applied
  - Риски: приватность экрана → обязательная маскировка и согласие

- Hearing System (орган слуха)
  - Клетки: AudioCapture → NoiseSuppress → STT → Redaction
  - Гейт: organ_hearing (locked)
  - Safe‑mode: запрещено
  - Метрики: hearing_minutes_captured, stt_latency_ms, stt_tokens

- Voice System (речь)
  - Клетки: TTS
  - Гейт: organ_voice (experimental)
  - Safe‑mode: чтение допустимо

- Motor System (движение)
  - Клетки: InputControl (mouse/keyboard), Macro/Replay
  - Гейт: organ_motor (locked)
  - Safe‑mode: запрещено; вне safe‑mode — только белый список окон/действий
  - Метрики: motor_actions_executed, motor_blocks

- FS System (файловая чувствительность)
  - Клетки: FsObserver → ContentIndexer → Redaction
  - Гейт: organ_fs (experimental)
  - Safe‑mode: read‑only
  - Метрики: fs_events_seen, fs_index_updates

- Net System (сетевые щупы)
  - Клетки: HttpProbe (head/get, без POST)
  - Гейт: organ_net_probe (locked)
  - Safe‑mode: запрещено

- Memory LTM (долгая память)
  - Клетки: EmbeddingsIndex → Retriever
  - Гейт: memory_vector_store (experimental)

- Executive/Planner (исполнитель)
  - Клетки: PlannerCell → PolicyCell → Scheduler
  - Гейт: executive_planner (experimental)

Минимум для зачаточного режима (Embryo)
- Всё locked, кроме:
  - Proprioception/Interoception (внутренние пробы) — experimental (read‑only)
  - organ_voice (experimental) — TTS при необходимости
  - organ_fs (experimental) — только чтение в выделенном каталоге (например, проект)
- Любые внешние сенсоры (Vision/Hearing) и особенно Motor — locked.

Активация по шагам
- Шаг 1: organ_voice (озвучивание отчётов), organ_fs (read‑only), Proprioception
- Шаг 2: organ_vision_readonly (белый список окон + редакция), memory_vector_store, executive_planner
- Шаг 3: organ_hearing (локально, с красными линиями), organ_motor (ограниченные макросы, только whitelisted)
- Шаг 4: organ_vision_active, organ_net_probe — только после зрелости политик/аудита

Интеграции
- Runtime Extensibility: UI‑инструменты (например, «Карандаш») встраиваются как плагины и используют органы через санкционированный API.
- Anti‑Idle: органы могут поставлять сигналы (например, Vision‑observations), но Auto‑задачи обязаны уважать гейт/политики.

Юр/этика
- Хранение/трансляция экран/аудио требует явного согласия; по умолчанию off.
- Маскирование/редакция — обязательно для любых приватных полей.
- Подробный аудит и быстрый Stop/Emergency‑Stop.

## Прогрессивная «чёткость» (Fidelity) и поэтапное усложнение

Идея: начинать с максимально простого и дешёвого по ресурсам восприятия/действий, затем повышать «чёткость» (fidelity)
по сигналам стабильности (SLO/метрики) и явному разрешению. Это снижает перегрузку и риски.

Общие защитные меры
- Бюджеты: fps/битрейт/частоты дискретизации/макс. событий в сек; мягкая деградация при перегрузе.
- ROI/whitelist: работать только в областях/окнах/каталогах белого списка.
- Сэмплирование: переход с polling на event‑driven при наличии сигналов.
- Маскировка/редакция: всегда на ранних уровнях (до расширения).

Тиражирование по органам (уровни)
- Vision (экран)
  - V0: выключено
  - V1: B/W low‑res, low‑fps (например, 1 fps, downscale, бинаризация) — только сигнал «видно/не видно»; редактирование по regex/ROI
  - V2: Grayscale low‑res, low‑fps + базовый OCR по ROI
  - V3: Color low‑res, event‑driven (по изменению), OCR/детекторы по whitelisted окнам
  - V4: Color + foveated ROI (высокая чёткость только на нужных зонах)
  - Гейты: `organ_vision_readonly` (V1–V3), `organ_vision_active` (V4+ эффекторные действия)

- Hearing (микрофон)
  - H0: выключено
  - H1: VU‑meter (уровни громкости), агрегированные секундные окна
  - H2: Spectral/MFCC summary (без слов), keyword‑spotting off
  - H3: Keyword spotting по whitelisted ключам, без сохранения «сырого» аудио
  - H4: STT локально/офлайн для коротких окон, с маскировкой
  - Гейт: `organ_hearing` (H1–H4)

- Voice (TTS)
  - T0: выключено
  - T1: TTS низкого качества для отчётов (ограниченная скорость/частота)
  - T2: TTS c профилем (настройка темпа/тона)
  - Гейт: `organ_voice`

- Motor (мышь/клавиатура)
  - M0: выключено
  - M1: Только подсказки/подсветка курсора (suggest‑only)
  - M2: Макросы на whitelisted окнах (требует подтверждения на запуск)
  - M3: Одиночные жесты в пределах вайтлиста (порог на действия/минуту)
  - Гейт: `organ_motor`

- FS (файловая система)
  - F0: выключено
  - F1: Read‑only watcher в 1 каталоге (низкая частота событий)
  - F2: Индексация по content c маскировкой (ограниченный объём/скорость)
  - Гейт: `organ_fs`

- Net Probe (сеть)
  - N0: выключено
  - N1: HEAD/GET по строгим whitelist URL (редкие проверки)
  - Гейт: `organ_net_probe`

Метрики для контроля fidelity
- vision_frames_ingested, vision_ocr_latency_ms, vision_bytes_per_min, vision_redactions_applied
- hearing_seconds_captured, hearing_keywords_detected, stt_latency_ms
- motor_actions_suggested/executed/blocked
- fs_events_seen/index_updates

Переход вверх по уровням
- Только при выполняющихся SLO: латентность/ошибки/нагрузка в норме за окно времени.
- Только с явной разблокировкой соответствующего гейта.
- Всегда с возможностью быстрого отката (фиксируем в JOURNALING.md и через neira:meta).

---

## OrganTemplate & Builder (дополнение)

- OrganTemplate: декларативное описание графа клеток (роли/каналы/политики/зависимости).
- OrganBuilder (Action): сборка по шаблону с dry‑run/canary/HITL, проверка интеграций с Nervous/Immune.
- Совместимость: проверка версий/линков, автопубликация метрик RED/USE, регистрация в Introspection.
- Управление: CAPABILITIES `organs_builder=experimental`, approve/rollback, журнал (JOURNALING).

См. также: design/factory-system.md, CAPABILITIES.md.
