# Neira Change Log

Автоматически сгенерированный журнал изменений из `neira:meta` комментариев.

---

## ANALYSIS

### `NEI-20251103-consciousness-analysis`
- **Файл**: `docs\analysis\NEIRA_CONSCIOUSNESS_ANALYSIS.md`
- **Суть**: |

## BUGFIX

### `NEI-20250418-duplicate-id-check`
- **Файл**: `spinal_cord\backend\src\cell_registry.rs`
- **Суть**: |- Проверяет уникальность идентификатора перед регистрацией шаблона.

### `NEI-20250418-duplicate-id-check`
- **Файл**: `spinal_cord\src\cell_registry.rs`
- **Суть**: |- Проверяет уникальность идентификатора перед регистрацией шаблона.

### `NEI-20250210-register-template-validate`
- **Файл**: `spinal_cord\backend\src\cell_registry.rs`
- **Суть**: Проверяет шаблон узла перед сохранением на диск.

### `NEI-20250210-register-template-validate`
- **Файл**: `spinal_cord\src\cell_registry.rs`
- **Суть**: Проверяет шаблон узла перед сохранением на диск.

### `NEI-20240728-brain-loop-local-event`
- **Файл**: `spinal_cord\backend\src\brain.rs`
- **Суть**: События из DataFlowController публикуются локально без повторной отправки.

### `NEI-20240728-brain-loop-local-event`
- **Файл**: `spinal_cord\src\brain.rs`
- **Суть**: События из DataFlowController публикуются локально без повторной отправки.

### `NEI-20240725-brain-local-dispatch`
- **Файл**: `spinal_cord\backend\src\brain.rs`
- **Суть**: Задачи ставятся локально и сразу отправляются в клетку анализа без повторной переотправки.

### `NEI-20240725-brain-local-dispatch`
- **Файл**: `spinal_cord\src\brain.rs`
- **Суть**: Задачи ставятся локально и сразу отправляются в клетку анализа без повторной переотправки.

## CHORE

### `NEI-20280502-120800-interaction-verbs-test`
- **Файл**: `spinal_cord\tests\interaction_verbs_test.rs`
- **Суть**: Проверяет детектор глаголов взаимодействия и публикацию события SynapseHub.

### `NEI-20280501-120200-tone-state-tests`
- **Файл**: `spinal_cord\tests\tone_state_test.rs`
- **Суть**: Добавлены юнит-тесты контроллера тонов с проверкой метрик и событий.

### `NEI-20280501-120150-chat-hub-tone-test`
- **Файл**: `spinal_cord\tests\chat_hub_test.rs`
- **Суть**: | Расширены интеграционные тесты чата проверкой обновления тонального состояния

### `NEI-20280106-120000-voice-tests`
- **Файл**: `spinal_cord\tests\voice_organ_test.rs`
- **Суть**: Покрывают голосовой орган: регистрация клеток, интеграция с фабрикой и TTS/STT через кодек.

### `NEI-20270618-event-bus-pathbuf`
- **Файл**: `spinal_cord\backend\src\event_bus.rs`
- **Суть**: Импортирован PathBuf для поля location события лимфатического фильтра.

### `NEI-20270618-event-bus-pathbuf`
- **Файл**: `spinal_cord\src\event_bus.rs`
- **Суть**: Импортирован PathBuf для поля location события лимфатического фильтра.

### `NEI-20270610-120200-lymphatic-event-test`
- **Файл**: `spinal_cord\tests\lymphatic_filter_event.rs`
- **Суть**: Проверяет публикацию lymphatic_filter.activated и реакцию подписчика.

### `NEI-20270405-brain-subscriber-import-cleanup`
- **Файл**: `spinal_cord\tests\brain_subscriber_test.rs`
- **Суть**: Удалён неиспользуемый FlowEvent из импорта.

### `NEI-20270310-120400-event-log-tests`
- **Файл**: `spinal_cord\tests\event_log_test.rs`
- **Суть**: Проверка записи и выборки событий из EventLog.

### `NEI-20270305-schema-sync-timeout`
- **Файл**: `spinal_cord\backend\src\main.rs`
- **Суть**: Синхронизация схем выполняется асинхронно и использует reqwest с таймаутом.

### `NEI-20270305-schema-sync-timeout`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: Синхронизация схем выполняется асинхронно и использует reqwest с таймаутом.

### `NEI-20261124-digestive-memory-hook`
- **Файл**: `spinal_cord\backend\src\main.rs`
- **Суть**: DigestivePipeline получает ссылку на MemoryCell для сохранения входов.

### `NEI-20260920-digestive-tracing`
- **Файл**: `spinal_cord\backend\src\digestive_pipeline.rs`
- **Суть**: Добавлены tracing-логи входа, формата и результата валидации.

### `NEI-20260920-digestive-tracing`
- **Файл**: `spinal_cord\src\digestive_pipeline.rs`
- **Суть**: Добавлены tracing-логи входа, формата и результата валидации.

### `NEI-20260725-digestive-init-main`
- **Файл**: `spinal_cord\backend\src\main.rs`
- **Суть**: Конфигурация DigestivePipeline загружается при старте.

### `NEI-20260710-quick-xml`
- **Файл**: `spinal_cord\backend\src\digestive_pipeline.rs`
- **Суть**: Использован quick-xml вместо serde_xml_rs для разбора XML.

### `NEI-20260710-quick-xml`
- **Файл**: `spinal_cord\src\digestive_pipeline.rs`
- **Суть**: Использован quick-xml вместо serde_xml_rs для разбора XML.

### `NEI-20250211-163900-training-module-index`
- **Файл**: `src\training\mod.rs`
- **Суть**: | Экспортировал модуль config для доступа к настройкам обучения из сервера.

### `NEI-20241003-brain-subscriber-test-update`
- **Файл**: `spinal_cord\tests\brain_subscriber_test.rs`
- **Суть**: Тест адаптирован к FlowReceiver с учётом новых счётчиков.

### `NEI-20241003-brain-loop-test-update`
- **Файл**: `tests\brain_loop_test.rs`
- **Суть**: Тесты обновлены под счётчики кровотока и FlowReceiver без промежуточных каналов.

### `NEI-20241003-brain-flow-test-update`
- **Файл**: `spinal_cord\tests\brain_flow_test.rs`
- **Суть**: Обновлён тест под новый FlowReceiver и публикацию метрик кровотока.

### `NEI-20240513-training-routes-lints`
- **Файл**: `spinal_cord\backend\src\http\training_routes.rs`
- **Суть**: Убраны предупреждения Clippy в training_routes: убран лишний паттерн и to_string.

### `NEI-20240513-training-routes-lints`
- **Файл**: `spinal_cord\src\http\training_routes.rs`
- **Суть**: Убраны предупреждения Clippy в training_routes: убран лишний паттерн и to_string.

### `NEI-20240513-training-result-alias`
- **Файл**: `spinal_cord\backend\src\action\scripted_training_cell.rs`
- **Суть**: Добавлен type alias TrainingResult для уменьшения сложности типа в отчётах.

### `NEI-20240513-training-result-alias`
- **Файл**: `spinal_cord\src\action\scripted_training_cell.rs`
- **Суть**: Добавлен type alias TrainingResult для уменьшения сложности типа в отчётах.

### `NEI-20240513-synapse-lints`
- **Файл**: `spinal_cord\backend\src\synapse_hub.rs`
- **Суть**: Убраны предупреждения Clippy: is_none_or/is_some_and, устранены while-let на итераторах, добавлены allow для больших ошибок и количества аргументов.

### `NEI-20240513-synapse-lints`
- **Файл**: `spinal_cord\src\synapse_hub.rs`
- **Суть**: Убраны предупреждения Clippy: is_none_or/is_some_and, устранены while-let на итераторах, добавлены allow для больших ошибок и количества аргументов.

### `NEI-20240513-storage-lints`
- **Файл**: `spinal_cord\backend\src\context\context_storage.rs`
- **Суть**: Убраны предупреждения Clippy в файловом хранилище контекста: лишние приведения, manual_flatten и др.

### `NEI-20240513-storage-lints`
- **Файл**: `spinal_cord\src\context\context_storage.rs`
- **Суть**: Убраны предупреждения Clippy в файловом хранилище контекста: лишние приведения, manual_flatten и др.

### `NEI-20240513-scheduler-lints`
- **Файл**: `spinal_cord\backend\src\task_scheduler.rs`
- **Суть**: Derive для Default и Priority, реализация Iterator вместо метода next, добавлен allow для too_many_arguments.

### `NEI-20240513-scheduler-lints`
- **Файл**: `spinal_cord\src\task_scheduler.rs`
- **Суть**: Derive для Default и Priority, реализация Iterator вместо метода next, добавлен allow для too_many_arguments.

### `NEI-20240513-policy-default`
- **Файл**: `spinal_cord\backend\src\policy\mod.rs`
- **Суть**: Добавлен Default для PolicyEngine для удовлетворения lint new_without_default.

### `NEI-20240513-policy-default`
- **Файл**: `spinal_cord\src\policy\mod.rs`
- **Суть**: Добавлен Default для PolicyEngine для удовлетворения lint new_without_default.

### `NEI-20240513-main-lints`
- **Файл**: `spinal_cord\backend\src\main.rs`
- **Суть**: Исправлены предупреждения Clippy в main.rs: объединены ветки if, убраны ненужные to_string и cast, и т.д.

### `NEI-20240513-main-lints`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: Исправлены предупреждения Clippy в main.rs: объединены ветки if, убраны ненужные to_string и cast, и т.д.

### `NEI-20240513-lib-test-allow`
- **Файл**: `spinal_cord\backend\src\lib.rs`
- **Суть**: Разрешён clippy::type_complexity для тестов через cfg_attr.

### `NEI-20240513-lib-test-allow`
- **Файл**: `spinal_cord\src\lib.rs`
- **Суть**: Разрешён clippy::type_complexity для тестов через cfg_attr.

### `NEI-20240513-initconfig-default`
- **Файл**: `spinal_cord\backend\src\security\init_config_cell.rs`
- **Суть**: Добавлен Default для InitConfigCell во избежание lint new_without_default.

### `NEI-20240513-initconfig-default`
- **Файл**: `spinal_cord\src\security\init_config_cell.rs`
- **Суть**: Добавлен Default для InitConfigCell во избежание lint new_without_default.

### `NEI-20240513-immune-lint`
- **Файл**: `spinal_cord\backend\src\immune_system\mod.rs`
- **Суть**: Подавлено предупреждение result_large_err для preflight_check.

### `NEI-20240513-immune-lint`
- **Файл**: `spinal_cord\src\immune_system\mod.rs`
- **Суть**: Подавлено предупреждение result_large_err для preflight_check.

### `NEI-20240513-idempotent-lints`
- **Файл**: `spinal_cord\backend\src\idempotent_store.rs`
- **Суть**: Явно указано truncate(false) и заменён flatten на map_while(Result::ok) при чтении.

### `NEI-20240513-idempotent-lints`
- **Файл**: `spinal_cord\src\idempotent_store.rs`
- **Суть**: Явно указано truncate(false) и заменён flatten на map_while(Result::ok) при чтении.

### `NEI-20240513-hostmetrics-lint`
- **Файл**: `spinal_cord\backend\src\nervous_system\host_metrics.rs`
- **Суть**: Использован saturating_sub для корректного подсчёта новых клеток.

### `NEI-20240513-hostmetrics-lint`
- **Файл**: `spinal_cord\src\nervous_system\host_metrics.rs`
- **Суть**: Использован saturating_sub для корректного подсчёта новых клеток.

### `NEI-20240513-factory-lints`
- **Файл**: `spinal_cord\backend\src\factory\mod.rs`
- **Суть**: Устранены предупреждения Clippy: заменены `if let Some(_)` на `is_some`, убрана лишняя `to_string` и подавлен result_large_err.

### `NEI-20240513-factory-lints`
- **Файл**: `spinal_cord\src\factory\mod.rs`
- **Суть**: Устранены предупреждения Clippy: заменены `if let Some(_)` на `is_some`, убрана лишняя `to_string` и подавлен result_large_err.

### `NEI-20240513-echochatcell-lint`
- **Файл**: `spinal_cord\backend\src\action\chat_cell.rs`
- **Суть**: EchoChatCell перестал быть unit-struct, чтобы убрать предупреждение clippy о default_constructed_unit_structs.

### `NEI-20240513-echochatcell-lint`
- **Файл**: `spinal_cord\src\action\chat_cell.rs`
- **Суть**: EchoChatCell перестал быть unit-struct, чтобы убрать предупреждение clippy о default_constructed_unit_structs.

## CODE

### `NEI-20280430-120400-healing-config`
- **Файл**: `spinal_cord\backend\src\healing_sleep\config.rs`
- **Суть**: | Конфигурация «Исцеляющего сна», загрузка из TOML и пороги серьёзности.

### `NEI-20280430-120400-healing-config`
- **Файл**: `spinal_cord\src\healing_sleep\config.rs`
- **Суть**: | Конфигурация «Исцеляющего сна», загрузка из TOML и пороги серьёзности.

### `NEI-20280430-120300-healing-trainer`
- **Файл**: `spinal_cord\backend\src\healing_sleep\trainer.rs`
- **Суть**: | Мини-тренер подбирает формулировки вопросов и хранит примеры для «исцеляющего сна».

### `NEI-20280430-120300-healing-trainer`
- **Файл**: `spinal_cord\src\healing_sleep\trainer.rs`
- **Суть**: | Мини-тренер подбирает формулировки вопросов и хранит примеры для «исцеляющего сна».

### `NEI-20280430-120200-healing-journal`
- **Файл**: `spinal_cord\backend\src\healing_sleep\journal.rs`
- **Суть**: | Журнал инцидентов и советов для цикла «Исцеляющий сон».

### `NEI-20280430-120200-healing-journal`
- **Файл**: `spinal_cord\src\healing_sleep\journal.rs`
- **Суть**: | Журнал инцидентов и советов для цикла «Исцеляющий сон».

### `NEI-20280430-120100-healing-advisor`
- **Файл**: `spinal_cord\backend\src\healing_sleep\advisor.rs`
- **Суть**: | Определяет интерфейс внешнего консультанта и типы для советов, полученных во время «исцеляющего сна».

### `NEI-20280430-120100-healing-advisor`
- **Файл**: `spinal_cord\src\healing_sleep\advisor.rs`
- **Суть**: | Определяет интерфейс внешнего консультанта и типы для советов, полученных во время «исцеляющего сна».

### `NEI-20280430-120000-healing-incident`
- **Файл**: `spinal_cord\backend\src\healing_sleep\incident.rs`
- **Суть**: |

### `NEI-20280430-120000-healing-incident`
- **Файл**: `spinal_cord\src\healing_sleep\incident.rs`
- **Суть**: |

### `NEI-20280105-voice-phonemes`
- **Файл**: `spinal_cord\backend\src\voice\phonemes.rs`
- **Суть**: |

### `NEI-20280105-voice-phonemes`
- **Файл**: `spinal_cord\src\voice\phonemes.rs`
- **Суть**: |

### `NEI-20280105-voice-organ`
- **Файл**: `spinal_cord\backend\src\voice\organ.rs`
- **Суть**: |

### `NEI-20280105-voice-organ`
- **Файл**: `spinal_cord\src\voice\organ.rs`
- **Суть**: |

### `NEI-20280105-voice-module`
- **Файл**: `spinal_cord\backend\src\voice\mod.rs`
- **Суть**: |

### `NEI-20280105-voice-module`
- **Файл**: `spinal_cord\src\voice\mod.rs`
- **Суть**: |

### `NEI-20280105-voice-error`
- **Файл**: `spinal_cord\backend\src\voice\error.rs`
- **Суть**: |-

### `NEI-20280105-voice-error`
- **Файл**: `spinal_cord\src\voice\error.rs`
- **Суть**: |-

### `NEI-20280105-voice-cells`
- **Файл**: `spinal_cord\backend\src\voice\cells.rs`
- **Суть**: |

### `NEI-20280105-voice-cells`
- **Файл**: `spinal_cord\src\voice\cells.rs`
- **Суть**: |

### `NEI-20280105-voice-backend`
- **Файл**: `spinal_cord\backend\src\voice\backend.rs`
- **Суть**: |

### `NEI-20280105-voice-backend`
- **Файл**: `spinal_cord\src\voice\backend.rs`
- **Суть**: |

### `NEI-20270520-lib-action-engine-export`
- **Файл**: `spinal_cord\backend\src\lib.rs`
- **Суть**: Экспортирован модуль action_engine.

### `NEI-20270520-lib-action-engine-export`
- **Файл**: `spinal_cord\src\lib.rs`
- **Суть**: Экспортирован модуль action_engine.

### `NEI-20270310-120200-event-log-export`
- **Файл**: `spinal_cord\backend\src\lib.rs`
- **Суть**: Экспортирован модуль event_log.

### `NEI-20270310-120200-event-log-export`
- **Файл**: `spinal_cord\src\lib.rs`
- **Суть**: Экспортирован модуль event_log.

### `NEI-20261005-time-metrics-export`
- **Файл**: `spinal_cord\backend\src\lib.rs`
- **Суть**: Экспортирован модуль time_metrics.

### `NEI-20261005-time-metrics-export`
- **Файл**: `spinal_cord\src\lib.rs`
- **Суть**: Экспортирован модуль time_metrics.

### `NEI-20260614-brain-loop`
- **Файл**: `spinal_cord\backend\src\brain.rs`
- **Суть**: Обрабатывает сообщения DataFlowController, распределяя события и задачи.

### `NEI-20260614-brain-loop`
- **Файл**: `spinal_cord\src\brain.rs`
- **Суть**: Обрабатывает сообщения DataFlowController, распределяя события и задачи.

### `NEI-20260614-brain-export`
- **Файл**: `spinal_cord\backend\src\lib.rs`
- **Суть**: Экспортирован модуль brain.

### `NEI-20260614-brain-export`
- **Файл**: `spinal_cord\src\lib.rs`
- **Суть**: Экспортирован модуль brain.

### `NEI-20260530-digestive-export`
- **Файл**: `spinal_cord\backend\src\lib.rs`
- **Суть**: Экспортирован модуль digestive_pipeline.

### `NEI-20260530-digestive-export`
- **Файл**: `spinal_cord\src\lib.rs`
- **Суть**: Экспортирован модуль digestive_pipeline.

### `NEI-20260514-preflight-check`
- **Файл**: `spinal_cord\backend\src\immune_system\mod.rs`
- **Суть**: Добавлена заглушка preflight_check для валидации записей.

### `NEI-20260514-preflight-check`
- **Файл**: `spinal_cord\src\immune_system\mod.rs`
- **Суть**: Добавлена заглушка preflight_check для валидации записей.

### `NEI-20260514-preflight-call`
- **Файл**: `spinal_cord\backend\src\factory\mod.rs`
- **Суть**: Добавлен вызов immune_system::preflight_check при создании записи.

### `NEI-20260514-preflight-call`
- **Файл**: `spinal_cord\src\factory\mod.rs`
- **Суть**: Добавлен вызов immune_system::preflight_check при создании записи.

### `NEI-20260514-factory-handler-result`
- **Файл**: `spinal_cord\backend\src\main.rs`
- **Суть**: Обработчик учитывает ошибки preflight_check.

### `NEI-20260514-factory-handler-result`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: Обработчик учитывает ошибки preflight_check.

### `NEI-20260514-factory-create-result`
- **Файл**: `spinal_cord\backend\src\synapse_hub.rs`
- **Суть**: Возвращает Result с ошибкой валидации при создании записи.

### `NEI-20260514-factory-create-result`
- **Файл**: `spinal_cord\src\synapse_hub.rs`
- **Суть**: Возвращает Result с ошибкой валидации при создании записи.

### `NEI-20260501-organ-stream-route`
- **Файл**: `spinal_cord\backend\src\main.rs`
- **Суть**: добавлен WS /organs/{id}/stream для трансляции смен статуса.

### `NEI-20260501-organ-stream-route`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: добавлен WS /organs/{id}/stream для трансляции смен статуса.

### `NEI-20260501-organ-status-broadcast`
- **Файл**: `spinal_cord\backend\src\organ_builder.rs`
- **Суть**: добавлен канал оповещений о смене статуса органа.

### `NEI-20260501-organ-status-broadcast`
- **Файл**: `spinal_cord\src\organ_builder.rs`
- **Суть**: добавлен канал оповещений о смене статуса органа.

### `NEI-20260501-organ-builder-subscribe`
- **Файл**: `spinal_cord\backend\src\synapse_hub.rs`
- **Суть**: проксирует подписку на события смены статуса органа.

### `NEI-20260501-organ-builder-subscribe`
- **Файл**: `spinal_cord\src\synapse_hub.rs`
- **Суть**: проксирует подписку на события смены статуса органа.

### `NEI-20260407-organs-list-route`
- **Файл**: `spinal_cord\backend\src\main.rs`
- **Суть**: добавлен GET /organs для выдачи id и state всех органов.

### `NEI-20260407-organs-list-route`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: добавлен GET /organs для выдачи id и state всех органов.

### `NEI-20260407-organ-list-hub`
- **Файл**: `spinal_cord\backend\src\synapse_hub.rs`
- **Суть**: проксирует орган-билдер для выдачи списка органов.

### `NEI-20260407-organ-list-hub`
- **Файл**: `spinal_cord\src\synapse_hub.rs`
- **Суть**: проксирует орган-билдер для выдачи списка органов.

### `NEI-20260407-organ-builder-list`
- **Файл**: `spinal_cord\backend\src\organ_builder.rs`
- **Суть**: добавлен метод list для выдачи идентификаторов и стадий всех органов.

### `NEI-20260407-organ-builder-list`
- **Файл**: `spinal_cord\src\organ_builder.rs`
- **Суть**: добавлен метод list для выдачи идентификаторов и стадий всех органов.

### `NEI-20260301-anti-idle-module`
- **Файл**: `spinal_cord\backend\src\nervous_system\anti_idle.rs`
- **Суть**: |-

### `NEI-20260301-anti-idle-module`
- **Файл**: `spinal_cord\src\nervous_system\anti_idle.rs`
- **Суть**: |-

### `NEI-20260214-loop-detector`
- **Файл**: `spinal_cord\backend\src\nervous_system\loop_detector.rs`
- **Суть**: |- Sliding window detector for repetitive SSE sequences; publishes loop_detected_total.

### `NEI-20260214-loop-detector`
- **Файл**: `spinal_cord\src\nervous_system\loop_detector.rs`
- **Суть**: |- Sliding window detector for repetitive SSE sequences; publishes loop_detected_total.

### `NEI-20251227-organ-built-event`
- **Файл**: `spinal_cord\backend\src\synapse_hub.rs`
- **Суть**: Публикует событие OrganBuilt при запуске сборки.

### `NEI-20251227-organ-built-event`
- **Файл**: `spinal_cord\src\synapse_hub.rs`
- **Суть**: Публикует событие OrganBuilt при запуске сборки.

### `NEI-20251227-nervous-subscriber`
- **Файл**: `spinal_cord\backend\src\nervous_system\mod.rs`
- **Суть**: Подписчик nervous_system на события CellCreated и OrganBuilt.

### `NEI-20251227-nervous-subscriber`
- **Файл**: `spinal_cord\src\nervous_system\mod.rs`
- **Суть**: Подписчик nervous_system на события CellCreated и OrganBuilt.

### `NEI-20251227-immune-subscriber`
- **Файл**: `spinal_cord\backend\src\immune_system\mod.rs`
- **Суть**: Подписчик immune_system на события CellCreated и OrganBuilt.

### `NEI-20251227-immune-subscriber`
- **Файл**: `spinal_cord\src\immune_system\mod.rs`
- **Суть**: Подписчик immune_system на события CellCreated и OrganBuilt.

### `NEI-20251227-event-bus-export`
- **Файл**: `spinal_cord\backend\src\lib.rs`
- **Суть**: Экспортирован модуль event_bus.

### `NEI-20251227-event-bus-export`
- **Файл**: `spinal_cord\src\lib.rs`
- **Суть**: Экспортирован модуль event_bus.

### `NEI-20251220-organ-builder-cleanup`
- **Файл**: `spinal_cord\backend\src\organ_builder.rs`
- **Суть**: добавлен фоновый таймер очистки и удаление записей templates/statuses вместе с файлом.

### `NEI-20251220-organ-builder-cleanup`
- **Файл**: `spinal_cord\src\organ_builder.rs`
- **Суть**: добавлен фоновый таймер очистки и удаление записей templates/statuses вместе с файлом.

### `NEI-20251205-organ-rebuild-route`
- **Файл**: `spinal_cord\backend\src\main.rs`
- **Суть**: добавлен POST /organs/{id}/rebuild для перезапуска сборки органа.

### `NEI-20251205-organ-rebuild-route`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: добавлен POST /organs/{id}/rebuild для перезапуска сборки органа.

### `NEI-20251205-organ-rebuild-method`
- **Файл**: `spinal_cord\backend\src\synapse_hub.rs`
- **Суть**: добавлен метод перезапуска сборки органа по шаблону.

### `NEI-20251205-organ-rebuild-method`
- **Файл**: `spinal_cord\src\synapse_hub.rs`
- **Суть**: добавлен метод перезапуска сборки органа по шаблону.

### `NEI-20251205-organ-rebuild`
- **Файл**: `spinal_cord\backend\src\organ_builder.rs`
- **Суть**: добавлен метод `rebuild` для повторного запуска сборки по сохранённому шаблону.

### `NEI-20251205-organ-rebuild`
- **Файл**: `spinal_cord\src\organ_builder.rs`
- **Суть**: добавлен метод `rebuild` для повторного запуска сборки по сохранённому шаблону.

### `NEI-20251115-organ-cancel-build-route`
- **Файл**: `spinal_cord\backend\src\main.rs`
- **Суть**: добавлен DELETE /organs/{id}/build для остановки сборки органа.

### `NEI-20251115-organ-cancel-build-route`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: добавлен DELETE /organs/{id}/build для остановки сборки органа.

### `NEI-20251115-organ-cancel-build-method`
- **Файл**: `spinal_cord\backend\src\synapse_hub.rs`
- **Суть**: добавлен метод отмены сборки органа.

### `NEI-20251115-organ-cancel-build-method`
- **Файл**: `spinal_cord\src\synapse_hub.rs`
- **Суть**: добавлен метод отмены сборки органа.

### `NEI-20251115-organ-cancel-build`
- **Файл**: `spinal_cord\backend\src\organ_builder.rs`
- **Суть**: сохраняются JoinHandle задач и добавлен cancel_build для их остановки.

### `NEI-20251115-organ-cancel-build`
- **Файл**: `spinal_cord\src\organ_builder.rs`
- **Суть**: сохраняются JoinHandle задач и добавлен cancel_build для их остановки.

### `NEI-20251101-organ-builder-stage-delays`
- **Файл**: `spinal_cord\backend\src\organ_builder.rs`
- **Суть**: Задержки переходов между стадиями читаются из ORGANS_BUILDER_STAGE_DELAYS.

### `NEI-20251101-organ-builder-stage-delays`
- **Файл**: `spinal_cord\src\organ_builder.rs`
- **Суть**: Задержки переходов между стадиями читаются из ORGANS_BUILDER_STAGE_DELAYS.

### `NEI-20251010-organ-status-update-route`
- **Файл**: `spinal_cord\backend\src\main.rs`
- **Суть**: добавлен POST /organs/{id}/status для ручного изменения стадии.

### `NEI-20251010-organ-status-update-route`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: добавлен POST /organs/{id}/status для ручного изменения стадии.

### `NEI-20251010-organ-builder-update`
- **Файл**: `spinal_cord\backend\src\synapse_hub.rs`
- **Суть**: добавлены методы обновления и получения статусов органа.

### `NEI-20251010-organ-builder-update`
- **Файл**: `spinal_cord\src\synapse_hub.rs`
- **Суть**: добавлены методы обновления и получения статусов органа.

### `NEI-20251010-organ-builder`
- **Файл**: `spinal_cord\backend\src\organ_builder.rs`
- **Суть**: |- Асинхронная сборка органов со стадиями Draft→Canary→Experimental→Stable, сохранением шаблонов на диск, удалением по TTL после стабилизации, метрикой времени сборки, остановкой при ручном изменении статуса и восстановлением счётчика идентификаторов при рестарте.

### `NEI-20251010-organ-builder`
- **Файл**: `spinal_cord\src\organ_builder.rs`
- **Суть**: |- Асинхронная сборка органов со стадиями Draft→Canary→Experimental→Stable, сохранением шаблонов на диск, удалением по TTL после стабилизации, метрикой времени сборки, остановкой при ручном изменении статуса и восстановлением счётчика идентификаторов при рестарте.

### `NEI-20250923-policy-engine-core`
- **Файл**: `spinal_cord\backend\src\policy\mod.rs`
- **Суть**: Каркас Policy Engine: проверка capability/ролей и унифицированные отказы.

### `NEI-20250923-policy-engine-core`
- **Файл**: `spinal_cord\src\policy\mod.rs`
- **Суть**: Каркас Policy Engine: проверка capability/ролей и унифицированные отказы.

### `NEI-20250923-factory-core`
- **Файл**: `spinal_cord\backend\src\factory\mod.rs`
- **Суть**: |

### `NEI-20250923-factory-core`
- **Файл**: `spinal_cord\src\factory\mod.rs`
- **Суть**: |

### `NEI-20250922-adaptive-queue-config`
- **Файл**: `spinal_cord\backend\src\queue_config.rs`
- **Суть**: | Адаптивные пороги очередей анализа вычисляются из исторических метрик и могут переопределяться переменными окружения.

### `NEI-20250922-adaptive-queue-config`
- **Файл**: `spinal_cord\src\queue_config.rs`
- **Суть**: | Адаптивные пороги очередей анализа вычисляются из исторических метрик и могут переопределяться переменными окружения.

### `NEI-20250720-immune-alert-handler`
- **Файл**: `spinal_cord\backend\src\immune_system\mod.rs`
- **Суть**: Добавлен обработчик alert для регистрации алертов иммунной системы.

### `NEI-20250720-immune-alert-handler`
- **Файл**: `spinal_cord\src\immune_system\mod.rs`
- **Суть**: Добавлен обработчик alert для регистрации алертов иммунной системы.

### `NEI-20250620-organ-builder-stage-delays-env`
- **Файл**: `spinal_cord\backend\src\organ_builder.rs`
- **Суть**: переименована переменная на ORGANS_BUILDER_STAGE_DELAYS.

### `NEI-20250620-organ-builder-stage-delays-env`
- **Файл**: `spinal_cord\src\organ_builder.rs`
- **Суть**: переименована переменная на ORGANS_BUILDER_STAGE_DELAYS.

### `NEI-20250601-organ-builder-restore`
- **Файл**: `spinal_cord\backend\src\organ_builder.rs`
- **Суть**: выделено восстановление шаблонов из templates_dir с логированием количества.

### `NEI-20250601-organ-builder-restore`
- **Файл**: `spinal_cord\src\organ_builder.rs`
- **Суть**: выделено восстановление шаблонов из templates_dir с логированием количества.

### `NEI-20250320-factory-auto-response-duration`
- **Файл**: `spinal_cord\backend\src\factory\mod.rs`
- **Суть**: Замеряем длительность auto_heal и auto_rollback.

### `NEI-20250320-factory-auto-response-duration`
- **Файл**: `spinal_cord\src\factory\mod.rs`
- **Суть**: Замеряем длительность auto_heal и auto_rollback.

### `NEI-20250317-organ-status-update-errors`
- **Файл**: `spinal_cord\backend\src\main.rs`
- **Суть**: logs rejection reasons and maps errors to 404/409 codes.

### `NEI-20250317-organ-status-update-errors`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: logs rejection reasons and maps errors to 404/409 codes.

### `NEI-20250317-organ-builder-status-error`
- **Файл**: `spinal_cord\backend\src\organ_builder.rs`
- **Суть**: increments error counter when updating status of missing organ.

### `NEI-20250317-organ-builder-status-error`
- **Файл**: `spinal_cord\src\organ_builder.rs`
- **Суть**: increments error counter when updating status of missing organ.

### `NEI-20250310-factory-auto-failure-metrics`
- **Файл**: `spinal_cord\backend\src\factory\mod.rs`
- **Суть**: Добавлены счётчики неудачных auto_heal и auto_rollback.

### `NEI-20250310-factory-auto-failure-metrics`
- **Файл**: `spinal_cord\src\factory\mod.rs`
- **Суть**: Добавлены счётчики неудачных auto_heal и auto_rollback.

### `NEI-20250226-dataflow-controller`
- **Файл**: `spinal_cord\backend\src\circulatory_system.rs`
- **Суть**: Простая шина передачи данных между органами через mpsc.

### `NEI-20250226-dataflow-controller`
- **Файл**: `spinal_cord\src\circulatory_system.rs`
- **Суть**: Простая шина передачи данных между органами через mpsc.

### `NEI-20250226-circulatory-export`
- **Файл**: `spinal_cord\backend\src\lib.rs`
- **Суть**: Экспортирован модуль circulatory_system.

### `NEI-20250226-circulatory-export`
- **Файл**: `spinal_cord\src\lib.rs`
- **Суть**: Экспортирован модуль circulatory_system.

### `NEI-20250215-ns-watch`
- **Файл**: `spinal_cord\backend\src\nervous_system\mod.rs`
- **Суть**: Добавлен заглушечный watch для мониторинга записей фабрики.

### `NEI-20250215-ns-watch`
- **Файл**: `spinal_cord\src\nervous_system\mod.rs`
- **Суть**: Добавлен заглушечный watch для мониторинга записей фабрики.

### `NEI-20250215-immune-module`
- **Файл**: `spinal_cord\backend\src\immune_system\mod.rs`
- **Суть**: Создан модуль immune_system с функцией observe.

### `NEI-20250215-immune-module`
- **Файл**: `spinal_cord\src\immune_system\mod.rs`
- **Суть**: Создан модуль immune_system с функцией observe.

### `NEI-20250215-immune-export`
- **Файл**: `spinal_cord\backend\src\lib.rs`
- **Суть**: Экспортирован модуль immune_system.

### `NEI-20250215-immune-export`
- **Файл**: `spinal_cord\src\lib.rs`
- **Суть**: Экспортирован модуль immune_system.

### `NEI-20250215-factory-auto-responses`
- **Файл**: `spinal_cord\backend\src\factory\mod.rs`
- **Суть**: Добавлены auto_heal и auto_rollback для реакций immune_system.

### `NEI-20250215-factory-auto-responses`
- **Файл**: `spinal_cord\src\factory\mod.rs`
- **Суть**: Добавлены auto_heal и auto_rollback для реакций immune_system.

### `NEI-20250214-watchdog-module`
- **Файл**: `spinal_cord\backend\src\nervous_system\watchdog.rs`
- **Суть**: |-

### `NEI-20250214-watchdog-module`
- **Файл**: `spinal_cord\src\nervous_system\watchdog.rs`
- **Суть**: |-

### `NEI-20250214-organ-builder-cli`
- **Файл**: `spinal_cord\backend\src\bin\organ_builder.rs`
- **Суть**: CLI для управления сборкой органов: build, status и cancel.

### `NEI-20250214-organ-builder-cli`
- **Файл**: `spinal_cord\src\bin\organ_builder.rs`
- **Суть**: CLI для управления сборкой органов: build, status и cancel.

### `NEI-20250101-000000-context-dir-helper`
- **Файл**: `spinal_cord\backend\src\context\mod.rs`
- **Суть**: Добавлена функция context_dir с учётом переменной CONTEXT_DIR.

### `NEI-20250101-000000-context-dir-helper`
- **Файл**: `spinal_cord\src\context\mod.rs`
- **Суть**: Добавлена функция context_dir с учётом переменной CONTEXT_DIR.

### `NEI-20240519-hearing-wrapper`
- **Файл**: `spinal_cord\backend\src\hearing.rs`
- **Суть**: | Обёртка вокруг tracing, отправляющая сообщения в Систему раздражителей.

### `NEI-20240519-hearing-wrapper`
- **Файл**: `spinal_cord\src\hearing.rs`
- **Суть**: | Обёртка вокруг tracing, отправляющая сообщения в Систему раздражителей.

## DESIGN

### `NEI-20251115-organ-cancel-build-design`
- **Файл**: `docs\design\factory-system.md`
- **Суть**: упомянут DELETE /organs/:id/build в API эскизе.

## DOCS

### `NEI-YYYYMMDD-HHMMSS-lymphatic-filter-doc`
- **Файл**: `COMMENTING.md`
- **Суть**: | Описание подсистемы «Лимфатический фильтр».

### `NEI-20280430-120700-healing-sleep-design`
- **Файл**: `docs\design\healing_sleep.md`
- **Суть**: Описывает систему «Исцеляющий сон»: журнал, тренер вопросов и внешний консультант.

### `NEI-20280415-120510-training-inquiry-doc`
- **Файл**: `spinal_cord\TRAINING.md`
- **Суть**: |- Добавлена тема «вопросы», лимит seed-выборки и примеры базовых вопросительных фраз из курса русской грамоты.

### `NEI-20280401-120010-russian-curriculum-doc`
- **Файл**: `spinal_cord\TRAINING.md`
- **Суть**: Описан учебный курс по русскому алфавиту и способ его загрузки.

### `NEI-20270615-lymphatic-impl-doc`
- **Файл**: `docs\architecture\lymphatic_filter.md`
- **Суть**: Описана реализация сканирования репозитория и событие duplicate_found.

### `NEI-20270615-lymphatic-dup-adr`
- **Файл**: `DECISIONS.md`
- **Суть**: ADR о включении анализа дубликатов через лимфатический фильтр.

### `NEI-20270615-lymphatic-capability-update`
- **Файл**: `CAPABILITIES.md`
- **Суть**: Уточнён статус и метрики события duplicate_found.

### `NEI-20270610-120400-lymphatic-activated-doc`
- **Файл**: `docs\architecture\lymphatic_filter.md`
- **Суть**: Добавлено событие lymphatic_filter.activated и его поля.

### `NEI-20270515-000001-training-interface-plan`
- **Файл**: `docs\design\training-interface-improvements.md`
- **Суть**: Сформулированы улучшения обучающих интерфейсов, метрики и интеграция с оркестратором.

### `NEI-20270424-shared-workspace-deps`
- **Файл**: `docs\guides\deployment.md`
- **Суть**: Добавлено примечание о едином наборе зависимостей для sensory_organs.

### `NEI-20270424-contrib-shared-deps`
- **Файл**: `docs\meta\CONTRIBUTING.md`
- **Суть**: Уточнено, что sensory_organs использует зависимости из корня.

### `NEI-20270408-000000-event-log-doc`
- **Файл**: `spinal_cord\AGENTS.md`
- **Суть**: Описан формат именования архивов EventLog с миллисекундами и счётчиком.

### `NEI-20270330-usage-workspace-install`
- **Файл**: `docs\guides\usage-example.md`
- **Суть**: Уточнена установка зависимостей через npm/pnpm.

### `NEI-20270330-faq-workspace-install`
- **Файл**: `docs\meta\faq.md`
- **Суть**: Уточнена установка зависимостей через npm/pnpm.

### `NEI-20270330-deployment-workspace-install`
- **Файл**: `docs\guides\deployment.md`
- **Суть**: Уточнена установка зависимостей через npm/pnpm workspace.

### `NEI-20270330-contrib-workspace-install`
- **Файл**: `docs\meta\CONTRIBUTING.md`
- **Суть**: Добавлена установка зависимостей через npm/pnpm workspace.

### `NEI-20270323-heartbeat-docs`
- **Файл**: `docs\design\nervous_system.md`
- **Суть**: Добавлены детали про пульс и метрику `sse_active`.

### `NEI-20270318-120050-training-orchestrator-doc`
- **Файл**: `spinal_cord\TRAINING.md`
- **Суть**: |-

### `NEI-20270318-120050-training-orchestrator-doc`
- **Файл**: `spinal_cord\TRAINING.md`
- **Суть**: |-

### `NEI-20270318-120040-roadmap-training`
- **Файл**: `docs\roadmap.md`
- **Суть**: |-

### `NEI-20270318-120030-training-metrics-docs`
- **Файл**: `docs\reference\metrics.md`
- **Суть**: |-

### `NEI-20270318-120020-training-env-docs`
- **Файл**: `docs\reference\env.md`
- **Суть**: |- Добавлены переменные LEARNING_MICROTASKS_ENABLED и TRAINING_AUTORUN_* для автоматизированного обучения и микрозадач анти-айдла.

### `NEI-20270318-120010-anti-idle-microtasks-doc`
- **Файл**: `docs\design\anti-idle-system.md`
- **Суть**: |-

### `NEI-20270318-120000-capabilities-training`
- **Файл**: `CAPABILITIES.md`
- **Суть**: |- Переведены training_pipeline и learning_microtasks в experimental, описана роль TrainingOrchestrator и очереди микрозадач.

### `NEI-20270305-schema-sync-timeout-env`
- **Файл**: `docs\reference\env.md`
- **Суть**: Добавлен SCHEMAS_SYNC_TIMEOUT_SECS для таймаута запросов синхронизации схем.

### `NEI-20270223-readme-digestive-overview`
- **Файл**: `README.md`
- **Суть**: Кратко описан DigestivePipeline и пример использования.

### `NEI-20270223-000000-spinal-digestive-doc`
- **Файл**: `spinal_cord\AGENTS.md`
- **Суть**: Добавлен раздел DigestivePipeline с форматами, конфигурацией и примерами.

### `NEI-20270210-schema-sync-env`
- **Файл**: `docs\reference\env.md`
- **Суть**: Добавлены SCHEMAS_ARCHIVE_URL и SCHEMAS_DIR для автообновления схем.

### `NEI-20261020-digestive-config-cache-env`
- **Файл**: `spinal_cord\ENV.md`
- **Суть**: Описано кэширование конфигурации DigestivePipeline и способ сброса.

### `NEI-20261015-digestive-cache-doc`
- **Файл**: `README.md`
- **Суть**: Описан кэш JSON Schema DigestivePipeline и способ сброса.

### `NEI-20261005-digestive-metrics-doc`
- **Файл**: `docs\reference\metrics.md`
- **Суть**: Документированы метрики digestive_parse_duration_ms и digestive_validation_duration_ms.

### `NEI-20260519-dataflow-controller`
- **Файл**: `README.md`
- **Суть**: Добавлен раздел о DataFlowController и связи органов с мозгом.

### `NEI-20260511-readme-spinalcord-app`
- **Файл**: `README.md`
- **Суть**: Уточнено, что модуль запускается как часть приложения spinal_cord.

### `NEI-20260501-organ-stream-doc`
- **Файл**: `docs\api\factory.md`
- **Суть**: описан WS /organs/:id/stream с примером подключения.

### `NEI-20260427-101700-backend-api-redirect`
- **Файл**: `docs\backend-api.md`
- **Суть**: | Файл перемещён в `api/spinal_cord.md`. Этот файл оставлен как редирект для сохранения обратной совместимости ссылок.

### `NEI-20260427-101500-spinal-api-rename`
- **Файл**: `docs\api\spinal_cord.md`
- **Суть**: | Переименовали справочник Backend API в "Spinal Cord API" и перенесли файл в `docs/api/`. Старый путь оставлен как редирект.

### `NEI-20260413-workflow-rename`
- **Файл**: `WORKFLOW.md`
- **Суть**: Пример scope обновлён под каталог spinal_cord.

### `NEI-20260413-voice-runbook-rename`
- **Файл**: `docs\guides\voice-v1-runbook.md`
- **Суть**: Заменены упоминания backend на spinal_cord.

### `NEI-20260413-training-rename`
- **Файл**: `spinal_cord\TRAINING.md`
- **Суть**: Обновлены пути на spinal_cord/.

### `NEI-20260413-spinal-env-rename`
- **Файл**: `spinal_cord\ENV.md`
- **Суть**: Переименован backend в spinal_cord и обновлены пути.

### `NEI-20260413-sensory-env-rename`
- **Файл**: `sensory_organs\ENV.md`
- **Суть**: Переименован frontend в sensory_organs и уточнён VITE_API_URL.

### `NEI-20260413-ports-rename`
- **Файл**: `docs\reference\ports.md`
- **Суть**: Обновлены названия сервисов и команда запуска spinal_cord.

### `NEI-20260413-metrics-rename`
- **Файл**: `docs\reference\metrics.md`
- **Суть**: Заменены упоминания backend на spinal_cord.

### `NEI-20260413-faq-rename`
- **Файл**: `docs\meta\faq.md`
- **Суть**: Обновлены инструкции для каталога spinal_cord.

### `NEI-20260413-factory-shim-rename`
- **Файл**: `docs\guides\factory-shim.md`
- **Суть**: Обновлены упоминания backend на spinal_cord.

### `NEI-20260413-factory-rename`
- **Файл**: `docs\api\factory.md`
- **Суть**: Обновлён пример запуска sensory_organs вместо frontend.

### `NEI-20260413-example-rename`
- **Файл**: `examples\generate_cell.rs`
- **Суть**: Обновлён путь к spinal_cord/Cargo.toml.

### `NEI-20260413-env-rename`
- **Файл**: `docs\reference\env.md`
- **Суть**: Заменены упоминания backend/frontend на spinal_cord/sensory_organs.

### `NEI-20260413-deployment-rename`
- **Файл**: `docs\guides\deployment.md`
- **Суть**: Обновлены пути и названия модулей spinal_cord и sensory_organs.

### `NEI-20260413-commenting-rename`
- **Файл**: `COMMENTING.md`
- **Суть**: Обновлён пример scope для каталога spinal_cord.

### `NEI-20260301-idle-env-docs`
- **Файл**: `docs\reference\env.md`
- **Суть**: Описаны IDLE_EMA_ALPHA и IDLE_DRYRUN_QUEUE_DEPTH.

### `NEI-20260301-anti-idle-docs`
- **Файл**: `docs\design\nervous_system.md`
- **Суть**: Добавлены пороги простоя и ручка `/api/neira/anti_idle/toggle`.

### `NEI-20260214-loop-detector-env-docs`
- **Файл**: `docs\reference\env.md`
- **Суть**: Добавлена переменная LOOP_ENTROPY_MIN для детектора повторов.

### `NEI-20260214-loop-detector-docs`
- **Файл**: `docs\design\nervous_system.md`
- **Суть**: Описан детектор повторов SSE и переменные LOOP_*.

### `NEI-20260101-navigation-guide`
- **Файл**: `NAVIGATION_HIDE_GUIDE.md`
- **Суть**: Инструкция по скрытию навигационного бара в панели сознания.

### `NEI-20251220-organ-builder-ttl-docs-update`
- **Файл**: `docs\reference\env.md`
- **Суть**: уточнено, что фоновой таймер удаляет шаблоны и статусы старше TTL.

### `NEI-20251116-vite-api-url-env-doc`
- **Файл**: `docs\reference\env.md`
- **Суть**: Добавлена переменная VITE_API_URL для фронтенда.

### `NEI-20251116-vite-api-url-env`
- **Файл**: `sensory_organs\ENV.md`
- **Суть**: Описана переменная VITE_API_URL для указания базового URL API.

### `NEI-20251115-organ-cancel-build-guide`
- **Файл**: `docs\guides\factory-shim.md`
- **Суть**: добавлена ссылка на DELETE /organs/:id/build как зарезервированный маршрут.

### `NEI-20251115-organ-cancel-build-doc`
- **Файл**: `docs\api\factory.md`
- **Суть**: описан DELETE /organs/:id/build для отмены сборки.

### `NEI-20251105-websocket-events-guide`
- **Файл**: `docs\guides\websocket-events-guide.md`
- **Суть**: | Руководство по использованию WebSocket /api/v1/events для получения событий в реальном времени.

### `NEI-20251105-quick-start-guide`
- **Файл**: `docs\guides\quick-start.md`
- **Суть**: | Краткое руководство по началу работы с Neira Desktop + Mobile.

### `NEI-20251105-p2p-collaboration`
- **Файл**: `docs\architecture\p2p-collaboration.md`
- **Суть**: | Архитектура P2P связи между экземплярами Neira для распределённого обучения и совместного решения задач.

### `NEI-20251105-neira-language-vision`
- **Файл**: `docs\architecture\neira-language.md`
- **Суть**: | Концепция собственного языка Neira для внутренней коммуникации, саморедактирования и защиты кода.

### `NEI-20251105-migration-plan`
- **Файл**: `docs\archive\DESKTOP_MOBILE_MIGRATION_PLAN.md`
- **Суть**: | План пошаговой миграции с Web UI на Desktop + Mobile архитектуру.

### `NEI-20251105-desktop-mobile-arch`
- **Файл**: `docs\architecture\desktop-mobile-ui.md`
- **Суть**: | Архитектурный документ Desktop + Mobile UI для Neira. Backend только API, UI — нативные приложения.

### `NEI-20251105-api-v1-migration`
- **Файл**: `docs\guides\api-v1-migration.md`
- **Суть**: | Руководство по миграции на /api/v1/* endpoints и стандартизированный формат ответов.

### `NEI-20251103-phase2-completion`
- **Файл**: `docs\PHASE2_COMPLETION.md`
- **Суть**: | Отчёт о завершении Фазы 2 (Memory Integration) — Семантическая память и саморазвивающиеся диалоги.

### `NEI-20251101-organ-builder-stage-delays-env`
- **Файл**: `spinal_cord\ENV.md`
- **Суть**: добавлена переменная ORGANS_BUILDER_STAGE_DELAYS.

### `NEI-20251101-organ-builder-stage-delays-doc`
- **Файл**: `docs\api\factory.md`
- **Суть**: добавлен пример настройки ORGANS_BUILDER_STAGE_DELAYS.

### `NEI-20251101-digestive-metrics-counters-doc`
- **Файл**: `docs\reference\digestive_pipeline_metrics.md`
- **Суть**: Добавлены/задокументированы счетчики DigestivePipeline: parsed_inputs_total и ошибки парсинга/валидации/фоллбэка.

### `NEI-20251101-120600-training-pipeline-doc`
- **Файл**: `docs\running.md`
- **Суть**: |

### `NEI-20251101-120500-training-metrics-doc`
- **Файл**: `docs\metrics.md`
- **Суть**: | Обновлено описание тренировочных метрик, добавлен пример работы AdaptiveScheduler и фиксации прогресса.

### `NEI-20251015-organ-builder-ttl-docs`
- **Файл**: `docs\reference\env.md`
- **Суть**: добавлена переменная ORGANS_BUILDER_TTL_SECS для очистки шаблонов.

### `NEI-20251010-organ-builder-status-route`
- **Файл**: `docs\api\factory.md`
- **Суть**: описан ручной апдейт статуса органа и метрика длительности сборки.

### `NEI-20251010-organ-builder-env-docs`
- **Файл**: `docs\reference\env.md`
- **Суть**: описаны ORGANS_BUILDER_ENABLED и ORGANS_BUILDER_TEMPLATES_DIR.

### `NEI-20251010-organ-builder-env`
- **Файл**: `spinal_cord\ENV.md`
- **Суть**: описаны переменные ORGANS_BUILDER_ENABLED, ORGANS_BUILDER_TEMPLATES_DIR и ORGANS_BUILDER_TTL_SECS.

### `NEI-20251010-organ-builder-cap-doc`
- **Файл**: `CAPABILITIES.md`
- **Суть**: добавлены примеры активации орган-билдера.

### `NEI-20250922-analysis-queue-env-docs`
- **Файл**: `docs\reference\env.md`
- **Суть**: Добавлены переменные для адаптивных порогов очередей анализа.

### `NEI-20250922-analysis-queue-env`
- **Файл**: `spinal_cord\ENV.md`
- **Суть**: Добавлены переменные управления порогами очередей анализа.

### `NEI-20250916-comment-conflicts-doc`
- **Файл**: `docs\guides\git-and-github.md`
- **Суть**: Добавлено описание автогасящегося конфликтов при изменении только комментариев.

### `NEI-20250915-adaptive-storage-backend-spinal_cord-env`
- **Файл**: `spinal_cord\ENV.md`
- **Суть**: Контекстное хранилище теперь подбирает лимиты по диску; переменные можно переопределить.

### `NEI-20250905-000000-docs-git-github-guide`
- **Файл**: `docs\guides\git-and-github.md`
- **Суть**: Подробная инструкция по Git/GitHub, меткам авто-ребейза и авто-фикса конфликтов, Merge Queue и git rerere. Разъяснение "Allow edits by maintainers".

### `NEI-20250904-dependency-hygiene-guide`
- **Файл**: `docs\guides\dependency-hygiene.md`
- **Суть**: Руководство по гигиене зависимостей: cargo tree -d, cargo-deny, скрипты дублей и CI job Dependency Hygiene.

### `NEI-20250904-121100-contrib-cell-runtime`
- **Файл**: `docs\meta\CONTRIBUTING.md`
- **Суть**: Уточнено, что используется Cell runtime (Node.js 20 LTS).

### `NEI-20250904-121050-glossary-doc`
- **Файл**: `docs\meta\glossary.md`
- **Суть**: |

### `NEI-20250904-121040-runtime-extensibility-doc`
- **Файл**: `docs\design\runtime-extensibility.md`
- **Суть**: |

### `NEI-20250904-121030-organ-systems-doc`
- **Файл**: `docs\design\organ-systems.md`
- **Суть**: |

### `NEI-20250904-121020-anti-idle-doc`
- **Файл**: `docs\design\anti-idle-system.md`
- **Суть**: |

### `NEI-20250904-121010-docs-roadmap-redirect`
- **Файл**: `docs\meta\roadmap.md`
- **Суть**: | Дублирующий roadmap перенесён. Теперь источником истины является docs/roadmap.md; этот файл оставлен как редирект для совместимости.

### `NEI-20250904-121000-persona-kernel`
- **Файл**: `docs\meta\persona-kernel.md`
- **Суть**: |

### `NEI-20250904-120950-usage-cell-registry`
- **Файл**: `docs\guides\usage-example.md`
- **Суть**: Добавлен пример с CellRegistry и уточнён runtime.

### `NEI-20250904-120940-persona-metrics`
- **Файл**: `docs\reference\persona-metrics.md`
- **Суть**: |

### `NEI-20250904-120931-schema-sync-async-doc`
- **Файл**: `docs\reference\schema-sync.md`
- **Суть**: Уточнено, что синхронизация выполняется асинхронно с таймаутом 10s и переменной SCHEMAS_SYNC_TIMEOUT_SECS.

### `NEI-20250904-120930-schema-sync-doc`
- **Файл**: `docs\reference\schema-sync.md`
- **Суть**: Описан автообновление JSON Schema из архива по SCHEMAS_ARCHIVE_URL.

### `NEI-20250904-120920-env-anti-idle`
- **Файл**: `docs\reference\env_anti_idle.md`
- **Суть**: Добавлен флаг ANTI_IDLE_ENABLED и его описание (addendum к ENV).

### `NEI-20250904-120910-web-interface-cell-registry`
- **Файл**: `docs\guides\web-interface.md`
- **Суть**: Добавлен пример регистрации клеток через CellRegistry.

### `NEI-20250904-120900-persona-capabilities`
- **Файл**: `docs\meta\capabilities-persona.md`
- **Суть**: | Перечень capability‑флагов личности и творческих студий с описанием, рисками, safeguards и откатами. Дополнение к CAPABILITIES.md.

### `NEI-20250904-120900-factory-shim-guide`
- **Файл**: `docs\guides\factory-shim.md`
- **Суть**: Внешний оркестратор (Shim) для фабрики: CLI, LLM-агент, безопасные команды dry-run/create/approve без прямой связи с ядром Нейры.

### `NEI-20250904-120850-homeostasis-adaptive-control`
- **Файл**: `docs\design\homeostasis.md`
- **Суть**: |

### `NEI-20250904-120840-docs-readme-redirect`
- **Файл**: `docs\README.md`
- **Суть**: |

### `NEI-20250904-120830-ide-cell-runtime`
- **Файл**: `docs\guides\ide-integration.md`
- **Суть**: Уточнено, что используется Cell runtime (Node.js).

### `NEI-20250904-120820-capabilities-control`
- **Файл**: `docs\meta\capabilities-control.md`
- **Суть**: |

### `NEI-20250904-120810-immune-metrics-doc`
- **Файл**: `docs\immune_system.md`
- **Суть**: добавлен раздел с метриками иммунной системы.

### `NEI-20250904-120800-digestive-diagram`
- **Файл**: `docs\digestive_pipeline_flow.md`
- **Суть**: Добавлена диаграмма потока данных DigestivePipeline.

### `NEI-20250904-120720-brain-doc`
- **Файл**: `docs\system\brain.md`
- **Суть**: Описана структура мозга и взаимодействие с DataFlow, EventBus и TaskScheduler.

### `NEI-20250904-120710-faq-cell-runtime`
- **Файл**: `docs\meta\faq.md`
- **Суть**: Уточнены требования к окружению с упоминанием Cell runtime.

### `NEI-20250904-120700-ports`
- **Файл**: `docs\reference\ports.md`
- **Суть**: Сводка стандартных портов и переменных окружения для сервисов Neira.

### `NEI-20250904-120650-taxonomy`
- **Файл**: `docs\meta\taxonomy.md`
- **Суть**: Консистентная таксономия терминов и соглашения по именованию/версиям.

### `NEI-20250904-120640-curl-examples`
- **Файл**: `docs\examples\curl.md`
- **Суть**: Добавлено упоминание смены порта через NEIRA_BIND_ADDR.

### `NEI-20250904-120630-factory-api-draft`
- **Файл**: `docs\api\factory.md`
- **Суть**: Черновой API Фабрикаторов (dry‑run/approve/rollback) и сборки органов.

### `NEI-20250904-120620-deploy-runtime-term`
- **Файл**: `docs\guides\deployment.md`
- **Суть**: Замена упоминаний Node.js на Cell.js runtime.

### `NEI-20250904-120610-organ-builder-metrics`
- **Файл**: `docs\reference\metrics.md`
- **Суть**: добавлена метрика organ_build_duration_ms и статусные запросы.

### `NEI-20250904-120600-adaptive-storage-env-docs`
- **Файл**: `docs\reference\env.md`
- **Суть**: Обновлено описание CONTEXT_MAX_LINES/CONTEXT_MAX_BYTES: адаптивные лимиты со storage_metrics.json.

### `NEI-20250904-120550-system-lifecycle-docs`
- **Файл**: `docs\design\system-lifecycle.md`
- **Суть**: Согласованная схема слоёв и жизненный цикл «отращивания органа» (от запроса до эксплуатации/отката).

### `NEI-20250904-120540-state-recovery-docs`
- **Файл**: `docs\design\state-and-recovery.md`
- **Суть**: Клетки без потери памяти: постоянное состояние, автоподхват на старте, upgrade hooks, runtime‑параметры и snapshot++.

### `NEI-20250904-120530-policy-engine-docs`
- **Файл**: `docs\design\policy-engine.md`
- **Суть**: Единый слой политик/гейтинга: фич-флаги, safe-mode, approvals, матрица прав и форматы отказов.

### `NEI-20250904-120520-nervous-system-docs`
- **Файл**: `docs\design\nervous_system.md`
- **Суть**: Описание Нервной системы Neira: цели, компоненты (пробы/метрики/живость/вотчдог), интеграции, ENV и диагностика.

### `NEI-20250904-120510-factory-system-design`
- **Файл**: `docs\design\factory-system.md`
- **Суть**: |

### `NEI-20250904-120501-roadmap-cleanup`
- **Файл**: `docs\roadmap.md`
- **Суть**: Чистовая дорожная карта Stage 0 → Stage 1: цели, DoD, интерфейсы, homeostasis/control, persona. Актуализированы ссылки и гейты.

### `NEI-20250904-120400-collab-tooling`
- **Файл**: `docs\guides\collaboration-tooling.md`
- **Суть**: Описаны новые инструменты: проверка neira:meta и Conventional Commits, интеграция в pre-commit и CI.

### `NEI-20250903-012100-doc-map`
- **Файл**: `docs\index.md`
- **Суть**: | Автогенерированный список файлов документации.

### `NEI-20250902-203443-journaling-link`
- **Файл**: `JOURNALING.md`
- **Суть**: | Обновлена ссылка на spinal_cord API в примере записи.

### `NEI-20250902-202115-rename-backend`
- **Файл**: `DECISIONS.md`
- **Суть**: |

### `NEI-20250829-setup-meta-storage`
- **Файл**: `spinal_cord\backend\src\context\context_storage.rs`
- **Суть**: | Файловое хранилище контекста (ndjson + дневная ротация + gzip), индекс index.json, TTL ключевых слов, адаптивные лимиты по диску (storage_metrics.json), маскирование (runtime + пресеты), буферизация записи, импорта.

### `NEI-20250829-setup-meta-storage`
- **Файл**: `spinal_cord\src\context\context_storage.rs`
- **Суть**: | Файловое хранилище контекста (ndjson + дневная ротация + gzip), индекс index.json, TTL ключевых слов, адаптивные лимиты по диску (storage_metrics.json), маскирование (runtime + пресеты), буферизация записи, импорта.

### `NEI-20250829-setup-meta-main`
- **Файл**: `spinal_cord\backend\src\main.rs`
- **Суть**: | Точки входа HTTP (API), SSE с прогрессом и отменой, маскирование с пресетами, поиск по content с фильтрами и пагинацией, rate-limit заголовки, скоупы токенов, включён CORS.

### `NEI-20250829-setup-meta-main`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: | Точки входа HTTP (API), SSE с прогрессом и отменой, маскирование с пресетами, поиск по content с фильтрами и пагинацией, rate-limit заголовки, скоупы токенов, включён CORS.

### `NEI-20250829-setup-meta-idem`
- **Файл**: `spinal_cord\backend\src\idempotent_store.rs`
- **Суть**: | Персистентное хранилище идемпотентных ответов (JSONL + TTL). Используется SynapseHub для выдачи повторных ответов по request_id, переживает рестарт.

### `NEI-20250829-setup-meta-idem`
- **Файл**: `spinal_cord\src\idempotent_store.rs`
- **Суть**: | Персистентное хранилище идемпотентных ответов (JSONL + TTL). Используется SynapseHub для выдачи повторных ответов по request_id, переживает рестарт.

### `NEI-20250829-setup-meta-hub`
- **Файл**: `spinal_cord\backend\src\synapse_hub.rs`
- **Суть**: |

### `NEI-20250829-setup-meta-hub`
- **Файл**: `spinal_cord\src\synapse_hub.rs`
- **Суть**: |

### `NEI-20250829-setup-meta-chatcell`
- **Файл**: `spinal_cord\backend\src\action\chat_cell.rs`
- **Суть**: |

### `NEI-20250829-setup-meta-chatcell`
- **Файл**: `spinal_cord\src\action\chat_cell.rs`
- **Суть**: |

### `NEI-20250829-181229-meta-coverage`
- **Файл**: `META_COVERAGE.md`
- **Суть**: | Определены уровни покрытия метаблоками и быстрый чек-лист для ревью.

### `NEI-20250829-175425-trigger-detector`
- **Файл**: `spinal_cord\backend\src\trigger_detector.rs`
- **Суть**: | Выявляет ключевые слова и запускает микрорефлексы.

### `NEI-20250829-175425-trigger-detector`
- **Файл**: `spinal_cord\src\trigger_detector.rs`
- **Суть**: | Выявляет ключевые слова и запускает микрорефлексы.

### `NEI-20250829-175425-task-scheduler`
- **Файл**: `spinal_cord\backend\src\task_scheduler.rs`
- **Суть**: | Планировщик задач с очередями по длительности и приоритетам.

### `NEI-20250829-175425-task-scheduler`
- **Файл**: `spinal_cord\src\task_scheduler.rs`
- **Суть**: | Планировщик задач с очередями по длительности и приоритетам.

### `NEI-20250829-175425-scripted-training`
- **Файл**: `spinal_cord\backend\src\action\scripted_training_cell.rs`
- **Суть**: | Выполняет сценарии обучения; пути и режим задаются через переменные окружения.

### `NEI-20250829-175425-scripted-training`
- **Файл**: `spinal_cord\src\action\scripted_training_cell.rs`
- **Суть**: | Выполняет сценарии обучения; пути и режим задаются через переменные окружения.

### `NEI-20250829-175425-safe-mode`
- **Файл**: `spinal_cord\backend\src\security\safe_mode_controller.rs`
- **Суть**: | Контролирует переход системы в безопасный режим.

### `NEI-20250829-175425-safe-mode`
- **Файл**: `spinal_cord\src\security\safe_mode_controller.rs`
- **Суть**: | Контролирует переход системы в безопасный режим.

### `NEI-20250829-175425-quarantine-cell`
- **Файл**: `spinal_cord\backend\src\security\quarantine_cell.rs`
- **Суть**: | Переводит подозрительные модули в карантин и активирует безопасный режим.

### `NEI-20250829-175425-quarantine-cell`
- **Файл**: `spinal_cord\src\security\quarantine_cell.rs`
- **Суть**: | Переводит подозрительные модули в карантин и активирует безопасный режим.

### `NEI-20250829-175425-metrics-collector`
- **Файл**: `spinal_cord\backend\src\action\metrics_collector_cell.rs`
- **Суть**: | Сборщик метрик с динамическим интервалом опроса.

### `NEI-20250829-175425-metrics-collector`
- **Файл**: `spinal_cord\src\action\metrics_collector_cell.rs`
- **Суть**: | Сборщик метрик с динамическим интервалом опроса.

### `NEI-20250829-175425-memory-cell`
- **Файл**: `spinal_cord\backend\src\memory_cell.rs`
- **Суть**: | Хранит результаты анализа и метаданные, поддерживает предзагрузку и приоритизацию.

### `NEI-20250829-175425-memory-cell`
- **Файл**: `spinal_cord\src\memory_cell.rs`
- **Суть**: | Хранит результаты анализа и метаданные, поддерживает предзагрузку и приоритизацию.

### `NEI-20250829-175425-io-watcher`
- **Файл**: `spinal_cord\backend\src\nervous_system\io_watcher.rs`
- **Суть**: | Отслеживает задержки ввода-вывода и публикует метрики при превышении порога.

### `NEI-20250829-175425-io-watcher`
- **Файл**: `spinal_cord\src\nervous_system\io_watcher.rs`
- **Суть**: | Отслеживает задержки ввода-вывода и публикует метрики при превышении порога.

### `NEI-20250829-175425-integrity-checker`
- **Файл**: `spinal_cord\backend\src\security\integrity_checker_cell.rs`
- **Суть**: | Проверяет контрольные суммы файлов и отправляет подозрительные в карантин.

### `NEI-20250829-175425-integrity-checker`
- **Файл**: `spinal_cord\src\security\integrity_checker_cell.rs`
- **Суть**: | Проверяет контрольные суммы файлов и отправляет подозрительные в карантин.

### `NEI-20250829-175425-init-config`
- **Файл**: `spinal_cord\backend\src\security\init_config_cell.rs`
- **Суть**: | Инициализирует переменные конфигурации, устанавливая INTEGRITY_ROOT.

### `NEI-20250829-175425-init-config`
- **Файл**: `spinal_cord\src\security\init_config_cell.rs`
- **Суть**: | Инициализирует переменные конфигурации, устанавливая INTEGRITY_ROOT.

### `NEI-20250829-175425-host-metrics`
- **Файл**: `spinal_cord\backend\src\nervous_system\host_metrics.rs`
- **Суть**: | Собирает метрики хоста и пересылает их коллектору.

### `NEI-20250829-175425-host-metrics`
- **Файл**: `spinal_cord\src\nervous_system\host_metrics.rs`
- **Суть**: | Собирает метрики хоста и пересылает их коллектору.

### `NEI-20250829-175425-diagnostics-cell`
- **Файл**: `spinal_cord\backend\src\action\diagnostics_cell.rs`
- **Суть**: | Анализирует поток метрик, фиксирует аномалии и уведомляет разработчика.

### `NEI-20250829-175425-diagnostics-cell`
- **Файл**: `spinal_cord\src\action\diagnostics_cell.rs`
- **Суть**: | Анализирует поток метрик, фиксирует аномалии и уведомляет разработчика.

### `NEI-20250829-175425-cell-template`
- **Файл**: `spinal_cord\backend\src\cell_template.rs`
- **Суть**: | Загружает и валидирует шаблоны ячеек по JSON‑схеме.

### `NEI-20250829-175425-cell-template`
- **Файл**: `spinal_cord\src\cell_template.rs`
- **Суть**: | Загружает и валидирует шаблоны ячеек по JSON‑схеме.

### `NEI-20250829-175425-cell-registry`
- **Файл**: `spinal_cord\backend\src\cell_registry.rs`
- **Суть**: | Отслеживает файлы шаблонов клеток и регистрирует реализации в системе.

### `NEI-20250829-175425-cell-registry`
- **Файл**: `spinal_cord\src\cell_registry.rs`
- **Суть**: | Отслеживает файлы шаблонов клеток и регистрирует реализации в системе.

### `NEI-20250829-175425-base-path-resolver`
- **Файл**: `spinal_cord\backend\src\nervous_system\base_path_resolver.rs`
- **Суть**: | Определяет базовый путь проекта и сохраняет его в памяти.

### `NEI-20250829-175425-base-path-resolver`
- **Файл**: `spinal_cord\src\nervous_system\base_path_resolver.rs`
- **Суть**: | Определяет базовый путь проекта и сохраняет его в памяти.

### `NEI-20250829-175425-analysis-cell`
- **Файл**: `spinal_cord\backend\src\analysis_cell.rs`
- **Суть**: | Общие структуры и интерфейсы для аналитических клеток.

### `NEI-20250829-175425-analysis-cell`
- **Файл**: `spinal_cord\src\analysis_cell.rs`
- **Суть**: | Общие структуры и интерфейсы для аналитических клеток.

### `NEI-20250829-175425-action-cell`
- **Файл**: `spinal_cord\backend\src\action_cell.rs`
- **Суть**: | Базовый интерфейс клеток действий и стандартная реализация предзагрузки.

### `NEI-20250829-175425-action-cell`
- **Файл**: `spinal_cord\src\action_cell.rs`
- **Суть**: | Базовый интерфейс клеток действий и стандартная реализация предзагрузки.

### `NEI-20250829-174731-simplified-block`
- **Файл**: `COMMENTING.md`
- **Суть**: | Добавлен раздел упрощённого блока, уточнены критерии и ссылка на META_COVERAGE.md.

### `NEI-20250720-immune-alert-metric-docs`
- **Файл**: `docs\reference\metrics.md`
- **Суть**: документирован счётчик immune_alerts_total с лейблом severity.

### `NEI-20250704-factory-state-transition-metric-docs`
- **Файл**: `docs\reference\metrics.md`
- **Суть**: документирован счётчик factory_state_transitions_total с лейблами from/to.

### `NEI-20250620-organ-builder-stage-delays-env-rename`
- **Файл**: `spinal_cord\ENV.md`
- **Суть**: переменная переименована в ORGANS_BUILDER_STAGE_DELAYS.

### `NEI-20250620-organ-builder-stage-delays-docs`
- **Файл**: `docs\reference\env.md`
- **Суть**: добавлена переменная ORGANS_BUILDER_STAGE_DELAYS.

### `NEI-20250620-organ-builder-stage-delays-doc-rename`
- **Файл**: `docs\api\factory.md`
- **Суть**: пример обновлён под ORGANS_BUILDER_STAGE_DELAYS.

### `NEI-20250607-factory-disabled-gauge-docs`
- **Файл**: `docs\reference\metrics.md`
- **Суть**: документирован gauge factory_cells_disabled.

### `NEI-20250602-150100-examplecell`
- **Файл**: `examples\cell_template.rs`
- **Суть**: | Переименован тип анализа в JSON-примере на ExampleCell.

### `NEI-20250508-analysis-training-examples`
- **Файл**: `docs\cells\analysis-cells.md`
- **Суть**: Добавлена ссылка на программу обучения и примеры на русском языке.

### `NEI-20250505-000000-immune-action-metrics-docs`
- **Файл**: `docs\reference\metrics.md`
- **Суть**: документированы immune_actions_total и immune_action_failures_total.

### `NEI-20250325-factory-nav-link`
- **Файл**: `README.md`
- **Суть**: Добавлена ссылка на систему фабрикаторов в навигации.

### `NEI-20250320-factory-auto-response-duration`
- **Файл**: `docs\reference\metrics.md`
- **Суть**: документирована метрика factory_auto_response_duration_ms с лейблом action.

### `NEI-20250317-organ-status-error-metric`
- **Файл**: `docs\reference\metrics.md`
- **Суть**: document organ_build_status_errors_total metric.

### `NEI-20250317-120500-analysis-architecture-cell-template`
- **Файл**: `docs\system\analysis-architecture.md`
- **Суть**: Обновлена ссылка на JSON-схему cell-template.

### `NEI-20250317-120400-voice-v1-runbook-cell-template`
- **Файл**: `docs\guides\voice-v1-runbook.md`
- **Суть**: Пошаговый запуск Voice v1 через Factory Adapter, обновлена ссылка на схему cell-template.

### `NEI-20250317-120300-testing-cell-template`
- **Файл**: `docs\guides\testing.md`
- **Суть**: Обновлена ссылка на JSON-схему cell-template в разделе тестирования.

### `NEI-20250317-120200-action-cells-schema-link`
- **Файл**: `docs\cells\action-cells.md`
- **Суть**: Обновлена ссылка на JSON-схему action-cell-template.

### `NEI-20250317-120100-cell-template-schema-links`
- **Файл**: `docs\cells\cell-template.md`
- **Суть**: Обновлены ссылки на JSON-схемы cell-template и action-cell-template.

### `NEI-20250317-120000-analysis-cells-cell-template`
- **Файл**: `docs\cells\analysis-cells.md`
- **Суть**: Обновлены ссылки на схемы cell-template.

### `NEI-20250316-stemcell-rename`
- **Файл**: `docs\design\factory-system.md`
- **Суть**: Термины обновлены на StemCellFactory/StemCellRecord/StemCellState.

### `NEI-20250314-backpressure-probe`
- **Файл**: `spinal_cord\backend\src\nervous_system\backpressure_probe.rs`
- **Суть**: |- Монитор очередей планировщика, публикующий backpressure и выполняющий троттлинг.

### `NEI-20250314-backpressure-probe`
- **Файл**: `spinal_cord\src\nervous_system\backpressure_probe.rs`
- **Суть**: |- Монитор очередей планировщика, публикующий backpressure и выполняющий троттлинг.

### `NEI-20250310-factory-auto-failure-metrics-docs`
- **Файл**: `docs\reference\metrics.md`
- **Суть**: добавлены счётчики неудачных auto_heal и auto_rollback.

### `NEI-20250310-cell-templates-recursive-docs`
- **Файл**: `docs\reference\env.md`
- **Суть**: Уточнено, что CELL_TEMPLATES_DIR поддерживает подкаталоги.

### `NEI-20250310-cell-templates-env-rename`
- **Файл**: `docs\reference\env.md`
- **Суть**: Переименована NODE_TEMPLATES_DIR в CELL_TEMPLATES_DIR с fallback на старое имя.

### `NEI-20250310-cell-templates-env-doc`
- **Файл**: `docs\guides\voice-v1-runbook.md`
- **Суть**: Обновлена переменная окружения на CELL_TEMPLATES_DIR с поддержкой NODE_TEMPLATES_DIR.

### `NEI-20250305-testing-runtime-term`
- **Файл**: `docs\guides\testing.md`
- **Суть**: Заголовки установки переведены на Cell.js runtime.

### `NEI-20250305-factory-shim-runtime-term`
- **Файл**: `docs\guides\factory-shim.md`
- **Суть**: Термин Node.js заменён на Cell runtime.

### `NEI-20250305-analysis-runtime-term`
- **Файл**: `docs\cells\analysis-cells.md`
- **Суть**: Список языков дополнен упоминанием runtime на Node.js.

### `NEI-20250301-pre-commit-pip-doc`
- **Файл**: `README.md`
- **Суть**: Добавлена команда установки pre-commit через pip.

### `NEI-20250225-120100-workflow-doc-map`
- **Файл**: `WORKFLOW.md`
- **Суть**: | Добавлено напоминание обновлять docs/index.md через gen-doc-map.

### `NEI-20250221-env-reference-link`
- **Файл**: `spinal_cord\ENV.md`
- **Суть**: Исправлена ссылка на основной справочник переменных окружения.

### `NEI-20250221-120000-bio-glossary`
- **Файл**: `docs\meta\biology-glossary.md`
- **Суть**: | Биологические метафоры для технических терминов.

### `NEI-20250219-organs-panel-doc`
- **Файл**: `docs\api\factory.md`
- **Суть**: добавлен раздел о запуске панели органов.

### `NEI-20250215-factory-auto-metrics`
- **Файл**: `docs\reference\metrics.md`
- **Суть**: документированы метрики auto_heal и auto_rollback.

### `NEI-20250214-watchdog-env-section`
- **Файл**: `docs\reference\env.md`
- **Суть**: Добавлен раздел с переменными WATCHDOG*.

### `NEI-20250214-watchdog-env-docs`
- **Файл**: `docs\design\nervous_system.md`
- **Суть**: Добавлен раздел про WATCHDOG* переменные.

### `NEI-20250214-organ-builder-cli-docs`
- **Файл**: `README.md`
- **Суть**: Добавлены примеры использования утилиты organ_builder.

### `NEI-20250214-121000-commenting-lymph-filter-example`
- **Файл**: `COMMENTING.md`
- **Суть**: Добавлен пример блока для lymphatic_filter.md.

### `NEI-20250214-120500-lymph-filter-capability`
- **Файл**: `CAPABILITIES.md`
- **Суть**: Добавлен компонент «Лимфатический фильтр» в перечень способностей.

### `NEI-20250214-120100-pre-commit-doc`
- **Файл**: `README.md`
- **Суть**: | Добавлены инструкции по установке локальных pre-commit хуков.

### `NEI-20250214-120000-lymph-filter`
- **Файл**: `DECISIONS.md`
- **Суть**: Добавлено решение о подсистеме «Лимфатический фильтр».

### `NEI-20250210-factory-template-schema-doc`
- **Файл**: `docs\api\factory.md`
- **Суть**: описана структура шаблона клетки.

### `NEI-20250207-factory-sample-templates-doc`
- **Файл**: `docs\api\factory.md`
- **Суть**: добавлен раздел Sample Templates с примерами органных шаблонов.

### `NEI-20250207-capabilities-sample-organs`
- **Файл**: `CAPABILITIES.md`
- **Суть**: добавлены ссылки на примеры шаблонов органов.

### `NEI-20250101-120000-docs-manifest-merge`
- **Файл**: `docs\guides\git-and-github.md`
- **Суть**: Указано, что авто-фиксер объединяет зависимости в Cargo.toml и package.json.

### `NEI-20250101-000005-cell-ids-doc`
- **Файл**: `docs\cell-ids.md`
- **Суть**: | Cell identifiers generated from organ specs.

### `NEI-20250101-000004-pathways-doc`
- **Файл**: `docs\pathways.md`
- **Суть**: | Описание поля pathways в спецификации органа и пример использования.

### `NEI-20241112-120000-web-interface-main-entry`
- **Файл**: `docs\guides\web-interface.md`
- **Суть**: Уточнён входной модуль фронтенда.

### `NEI-20241003-brain-doc-flowreceiver`
- **Файл**: `docs\system\brain.md`
- **Суть**: Пример обновлён для FlowReceiver и try_recv.

### `NEI-20240918-doc-index-brain`
- **Файл**: `docs\index.md`
- **Суть**: Добавлена ссылка на описание модуля brain.

### `NEI-20240909-120000-lymphatic-filter`
- **Файл**: `docs\architecture\lymphatic_filter.md`
- **Суть**: Описан фильтр лимфатической системы и анализ дубликатов.

### `NEI-20240607-systemprobe-doc-stop`
- **Файл**: `README.md`
- **Суть**: Пример SystemProbe обновлён методом stop.

### `NEI-20240601-nervous-toc-env-links`
- **Файл**: `docs\design\nervous_system.md`
- **Суть**: Добавлено оглавление и ссылки на ENV и метрики.

### `NEI-20240517-120003-factory-api-autoresponse`
- **Файл**: `docs\api\factory.md`
- **Суть**: | Добавлены события и API auto_heal/auto_rollback.

### `NEI-20240517-120001-factory-integration-selfheal`
- **Файл**: `docs\design\factory-system.md`
- **Суть**: | Добавлены разделы интеграции с Nervous/Immune и самовосстановления.

### `NEI-20240514-brain-doc-example`
- **Файл**: `docs\system\brain.md`
- **Суть**: Добавлен пример отправки FlowEvent и TaskPayload через DataFlowController.

### `NEI-20240513-fabricators-rename`
- **Файл**: `docs\design\nervous_system.md`
- **Суть**: Переименована подсистема «Фабрика» в «Система фабрикаторов».

### `NEI-20240513-fabricator-system-term`
- **Файл**: `docs\cells\analysis-cells.md`
- **Суть**: Переименована «Фабрика клеток» в «Система фабрикаторов».

## FEAT

### `NEI-20270323-heartbeat-module`
- **Файл**: `spinal_cord\backend\src\nervous_system\heartbeat.rs`
- **Суть**: |- Обёртка для обновления метрики активных SSE-подключений.

### `NEI-20270323-heartbeat-module`
- **Файл**: `spinal_cord\src\nervous_system\heartbeat.rs`
- **Суть**: |- Обёртка для обновления метрики активных SSE-подключений.

### `NEI-20270210-schema-sync`
- **Файл**: `spinal_cord\backend\src\main.rs`
- **Суть**: Автоматическая проверка даты архива схем и их синхронизация при старте.

### `NEI-20270210-schema-sync`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: Автоматическая проверка даты архива схем и их синхронизация при старте.

### `NEI-20241003-hub-flow-metrics`
- **Файл**: `spinal_cord\backend\src\synapse_hub.rs`
- **Суть**: SynapseHub периодически публикует счётчики кровотока в MetricsCollectorCell и gauge метрики.

### `NEI-20241003-hub-flow-metrics`
- **Файл**: `spinal_cord\src\synapse_hub.rs`
- **Суть**: SynapseHub периодически публикует счётчики кровотока в MetricsCollectorCell и gauge метрики.

### `NEI-20241003-flow-counters`
- **Файл**: `spinal_cord\backend\src\circulatory_system.rs`
- **Суть**: Учёт отправленных и полученных сообщений через AtomicU64 и обёртку FlowReceiver.

### `NEI-20241003-flow-counters`
- **Файл**: `spinal_cord\src\circulatory_system.rs`
- **Суть**: Учёт отправленных и полученных сообщений через AtomicU64 и обёртку FlowReceiver.

### `NEI-20241003-brain-flow-metrics`
- **Файл**: `spinal_cord\backend\src\brain.rs`
- **Суть**: Brain публикует счётчики кровотока через MetricsCollectorCell и gauge метрики.

### `NEI-20241003-brain-flow-metrics`
- **Файл**: `spinal_cord\src\brain.rs`
- **Суть**: Brain публикует счётчики кровотока через MetricsCollectorCell и gauge метрики.

### `NEI-20240930-brain-subscriber-hook`
- **Файл**: `spinal_cord\backend\src\synapse_hub.rs`
- **Суть**: Подписывает BrainSubscriber на события EventBus.

### `NEI-20240930-brain-subscriber-hook`
- **Файл**: `spinal_cord\src\synapse_hub.rs`
- **Суть**: Подписывает BrainSubscriber на события EventBus.

### `NEI-20240930-brain-subscriber`
- **Файл**: `spinal_cord\backend\src\brain.rs`
- **Суть**: |- Подписчик BrainSubscriber отправляет события в DataFlowController, игнорируя FlowEvent из кровотока.

### `NEI-20240930-brain-subscriber`
- **Файл**: `spinal_cord\src\brain.rs`
- **Суть**: |- Подписчик BrainSubscriber отправляет события в DataFlowController, игнорируя FlowEvent из кровотока.

### `NEI-20240821-brain-metrics`
- **Файл**: `spinal_cord\backend\src\brain.rs`
- **Суть**: Учёт обработанных задач и событий через MetricsCollectorCell и счётчики.

### `NEI-20240821-brain-metrics`
- **Файл**: `spinal_cord\src\brain.rs`
- **Суть**: Учёт обработанных задач и событий через MetricsCollectorCell и счётчики.

## FEATURE

### `NEI-20280502-120500-interaction-verbs`
- **Файл**: `spinal_cord\backend\src\synapse_hub.rs`
- **Суть**: |- SynapseHub определяет глаголы взаимодействия в сообщениях чата, добавляет триггеры, метрики и публикует события persona.interaction_verb.observed.

### `NEI-20280502-120500-interaction-verbs`
- **Файл**: `spinal_cord\src\synapse_hub.rs`
- **Суть**: |- SynapseHub определяет глаголы взаимодействия в сообщениях чата, добавляет триггеры, метрики и публикует события persona.interaction_verb.observed.

### `NEI-20280502-120100-persona-interaction-verbs`
- **Файл**: `spinal_cord\backend\src\persona\mod.rs`
- **Суть**: Экспортирован детектор глаголов взаимодействия и событие наблюдения.

### `NEI-20280502-120100-persona-interaction-verbs`
- **Файл**: `spinal_cord\src\persona\mod.rs`
- **Суть**: Экспортирован детектор глаголов взаимодействия и событие наблюдения.

### `NEI-20280502-120000-interaction-verbs`
- **Файл**: `spinal_cord\backend\src\persona\interaction_verbs.rs`
- **Суть**: |- Детектор глаголов взаимодействия определяет ожидаемое действие собеседника, нормализует сообщения и публикует событие для шины EventBus.

### `NEI-20280502-120000-interaction-verbs`
- **Файл**: `spinal_cord\src\persona\interaction_verbs.rs`
- **Суть**: |- Детектор глаголов взаимодействия определяет ожидаемое действие собеседника, нормализует сообщения и публикует событие для шины EventBus.

### `NEI-20280501-120030-tone-state-controller`
- **Файл**: `spinal_cord\backend\src\persona\tone_state.rs`
- **Суть**: |-

### `NEI-20280501-120030-tone-state-controller`
- **Файл**: `spinal_cord\src\persona\tone_state.rs`
- **Суть**: |-

### `NEI-20280501-120000-persona-module`
- **Файл**: `spinal_cord\backend\src\persona\mod.rs`
- **Суть**: Экспортирован модуль эмоциональных состояний личности (tone state).

### `NEI-20280501-120000-persona-module`
- **Файл**: `spinal_cord\src\persona\mod.rs`
- **Суть**: Экспортирован модуль эмоциональных состояний личности (tone state).

### `NEI-20280430-120500-healing-sleep`
- **Файл**: `spinal_cord\backend\src\healing_sleep\mod.rs`
- **Суть**: | «Исцеляющий сон» — модуль, который фиксирует инциденты, собирает советы и учится их уточнять.

### `NEI-20280430-120500-healing-sleep`
- **Файл**: `spinal_cord\src\healing_sleep\mod.rs`
- **Суть**: | «Исцеляющий сон» — модуль, который фиксирует инциденты, собирает советы и учится их уточнять.

### `NEI-20280425-120240-curriculum-theme-log`
- **Файл**: `spinal_cord\backend\src\synapse_hub.rs`
- **Суть**: |- Публикация курса сопровождается срезом по темам и передаёт статистику в событие training.curriculum.loaded для дальнейшей аналитики.

### `NEI-20280425-120240-curriculum-theme-log`
- **Файл**: `spinal_cord\src\synapse_hub.rs`
- **Суть**: |- Публикация курса сопровождается срезом по темам и передаёт статистику в событие training.curriculum.loaded для дальнейшей аналитики.

### `NEI-20280425-120230-curriculum-theme-stats`
- **Файл**: `spinal_cord\backend\src\event_bus.rs`
- **Суть**: |- В событие training.curriculum.loaded добавлены агрегаты по темам словаря, чтобы подписчики видели баланс навыков сразу при загрузке.

### `NEI-20280425-120230-curriculum-theme-stats`
- **Файл**: `spinal_cord\src\event_bus.rs`
- **Суть**: |- В событие training.curriculum.loaded добавлены агрегаты по темам словаря, чтобы подписчики видели баланс навыков сразу при загрузке.

### `NEI-20280425-120200-curriculum-editor`
- **Файл**: `spinal_cord\backend\src\bin\curriculum_editor.rs`
- **Суть**: | Добавлен CLI-инструмент curriculum_editor для просмотра статистики и расширения словаря учебного курса с валидацией.

### `NEI-20280425-120200-curriculum-editor`
- **Файл**: `spinal_cord\src\bin\curriculum_editor.rs`
- **Суть**: | Добавлен CLI-инструмент curriculum_editor для просмотра статистики и расширения словаря учебного курса с валидацией.

### `NEI-20280415-120500-inquiry-seed`
- **Файл**: `spinal_cord\backend\src\training\curriculum.rs`
- **Суть**: | Реализована build_inquiry_seed и приоритизация темы «вопросы» в учебном курсе, чтобы ограниченная выборка включала ключевые вопросительные слова.

### `NEI-20280415-120500-inquiry-seed`
- **Файл**: `spinal_cord\src\training\curriculum.rs`
- **Суть**: | Реализована build_inquiry_seed и приоритизация темы «вопросы» в учебном курсе, чтобы ограниченная выборка включала ключевые вопросительные слова.

### `NEI-20280401-120040-curriculum-test`
- **Файл**: `spinal_cord\tests\training_curriculum_test.rs`
- **Суть**: Проверяет загрузку курса русской грамоты: память, событие и данные.

### `NEI-20280401-120030-russian-curriculum-hub`
- **Файл**: `spinal_cord\backend\src\synapse_hub.rs`
- **Суть**: SynapseHub умеет загружать курс русской грамоты и публикует событие о прогрессе обучения.

### `NEI-20280401-120030-russian-curriculum-hub`
- **Файл**: `spinal_cord\src\synapse_hub.rs`
- **Суть**: SynapseHub умеет загружать курс русской грамоты и публикует событие о прогрессе обучения.

### `NEI-20280401-120020-curriculum-event`
- **Файл**: `spinal_cord\backend\src\event_bus.rs`
- **Суть**: Добавлено событие training.curriculum.loaded для фиксации загрузки учебного курса.

### `NEI-20280401-120020-curriculum-event`
- **Файл**: `spinal_cord\src\event_bus.rs`
- **Суть**: Добавлено событие training.curriculum.loaded для фиксации загрузки учебного курса.

### `NEI-20280401-120000-russian-curriculum`
- **Файл**: `spinal_cord\backend\src\training\curriculum.rs`
- **Суть**: |

### `NEI-20280401-120000-russian-curriculum`
- **Файл**: `spinal_cord\src\training\curriculum.rs`
- **Суть**: |

### `NEI-20280105-voice-api-main`
- **Файл**: `spinal_cord\backend\src\main.rs`
- **Суть**: Добавлены обработчики /voice/speak и /voice/transcribe, инициализация VoiceOrgan.

### `NEI-20280105-voice-api-main`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: Добавлены обработчики /voice/speak и /voice/transcribe, инициализация VoiceOrgan.

### `NEI-20270618-000000-lymphatic-filter-tests`
- **Файл**: `spinal_cord\tests\lymphatic_filter.rs`
- **Суть**: Юнит-тесты лимфатического фильтра на поиск дубликатов и работу флага.

### `NEI-20270618-000000-lymphatic-filter-module`
- **Файл**: `spinal_cord\backend\src\immune_system\lymphatic_filter.rs`
- **Суть**: Лимфатический фильтр сканирует рабочее пространство и выявляет дубликаты функций, поддерживая кэш, гибкие параметры и генерацию патчей.

### `NEI-20270618-000000-lymphatic-filter-module`
- **Файл**: `spinal_cord\src\immune_system\lymphatic_filter.rs`
- **Суть**: Лимфатический фильтр сканирует рабочее пространство и выявляет дубликаты функций, поддерживая кэш, гибкие параметры и генерацию патчей.

### `NEI-20270615-lymphatic-duplicate-event`
- **Файл**: `spinal_cord\backend\src\event_bus.rs`
- **Суть**: Добавлено событие LymphaticDuplicateFound для фиксации дубликатов функций.

### `NEI-20270615-lymphatic-duplicate-event`
- **Файл**: `spinal_cord\src\event_bus.rs`
- **Суть**: Добавлено событие LymphaticDuplicateFound для фиксации дубликатов функций.

### `NEI-20270610-120100-event-log-payload`
- **Файл**: `spinal_cord\backend\src\event_log.rs`
- **Суть**: LoggedEvent хранит произвольные данные события в поле data.

### `NEI-20270610-120100-event-log-payload`
- **Файл**: `spinal_cord\src\event_log.rs`
- **Суть**: LoggedEvent хранит произвольные данные события в поле data.

### `NEI-20270610-120000-lymphatic-event`
- **Файл**: `spinal_cord\backend\src\event_bus.rs`
- **Суть**: Добавлено событие lymphatic_filter.activated и метод data для передачи полей события.

### `NEI-20270610-120000-lymphatic-event`
- **Файл**: `spinal_cord\src\event_bus.rs`
- **Суть**: Добавлено событие lymphatic_filter.activated и метод data для передачи полей события.

### `NEI-20270520-action-engine`
- **Файл**: `spinal_cord\backend\src\action_engine.rs`
- **Суть**: | Асинхронный движок для файловых, сетевых и системных операций с проверкой прав.

### `NEI-20270520-action-engine`
- **Файл**: `spinal_cord\src\action_engine.rs`
- **Суть**: | Асинхронный движок для файловых, сетевых и системных операций с проверкой прав.

### `NEI-20270520-action-cell-engine`
- **Файл**: `spinal_cord\backend\src\action_cell.rs`
- **Суть**: | Добавлена поддержка ActionEngine для выполнения команд клетками.

### `NEI-20270520-action-cell-engine`
- **Файл**: `spinal_cord\src\action_cell.rs`
- **Суть**: | Добавлена поддержка ActionEngine для выполнения команд клетками.

### `NEI-20270505-events-ws`
- **Файл**: `spinal_cord\backend\src\main.rs`
- **Суть**: WebSocket-поток EventLog для живых подписок.

### `NEI-20270505-events-ws`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: WebSocket-поток EventLog для живых подписок.

### `NEI-20270505-event-log-broadcast`
- **Файл**: `spinal_cord\backend\src\event_log.rs`
- **Суть**: |- Подписчики получают новые события через broadcast-канал.

### `NEI-20270505-event-log-broadcast`
- **Файл**: `spinal_cord\src\event_log.rs`
- **Суть**: |- Подписчики получают новые события через broadcast-канал.

### `NEI-20270501-event-log-name-filter`
- **Файл**: `spinal_cord\backend\src\event_log.rs`
- **Суть**: query фильтрует события по имени.

### `NEI-20270501-event-log-name-filter`
- **Файл**: `spinal_cord\src\event_log.rs`
- **Суть**: query фильтрует события по имени.

### `NEI-20270501-000000-events-name-filter`
- **Файл**: `spinal_cord\backend\src\main.rs`
- **Суть**: Эндпоинт поддерживает фильтр по имени события.

### `NEI-20270501-000000-events-name-filter`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: Эндпоинт поддерживает фильтр по имени события.

### `NEI-20270405-digestive-toxicity-filter`
- **Файл**: `spinal_cord\backend\src\digestive_pipeline.rs`
- **Суть**: Добавлены базовые фильтры токсичных слов перед TriggerDetector.

### `NEI-20270405-digestive-toxicity-filter`
- **Файл**: `spinal_cord\src\digestive_pipeline.rs`
- **Суть**: Добавлены базовые фильтры токсичных слов перед TriggerDetector.

### `NEI-20270401-action-http-post`
- **Файл**: `spinal_cord\backend\src\action_engine.rs`
- **Суть**: Добавлена команда HttpPost и отправка POST-запросов через reqwest.

### `NEI-20270401-action-http-post`
- **Файл**: `spinal_cord\src\action_engine.rs`
- **Суть**: Добавлена команда HttpPost и отправка POST-запросов через reqwest.

### `NEI-20270318-120130-anti-idle-microtasks-export`
- **Файл**: `spinal_cord\backend\src\nervous_system\mod.rs`
- **Суть**: Экспортирован модуль anti_idle_microtasks для очереди микрозадач.

### `NEI-20270318-120130-anti-idle-microtasks-export`
- **Файл**: `spinal_cord\src\nervous_system\mod.rs`
- **Суть**: Экспортирован модуль anti_idle_microtasks для очереди микрозадач.

### `NEI-20270318-120120-training-export`
- **Файл**: `spinal_cord\backend\src\lib.rs`
- **Суть**: Экспортирован модуль training для автоматизированного обучения.

### `NEI-20270318-120120-training-export`
- **Файл**: `spinal_cord\src\lib.rs`
- **Суть**: Экспортирован модуль training для автоматизированного обучения.

### `NEI-20270318-120110-training-caps`
- **Файл**: `spinal_cord\backend\src\synapse_hub.rs`
- **Суть**: | SynapseHub хранит флаги learning_microtasks/training_pipeline/training_autorun и предоставляет геттеры для анти-айдла и оркестратора.

### `NEI-20270318-120110-training-caps`
- **Файл**: `spinal_cord\src\synapse_hub.rs`
- **Суть**: | SynapseHub хранит флаги learning_microtasks/training_pipeline/training_autorun и предоставляет геттеры для анти-айдла и оркестратора.

### `NEI-20270318-120100-policy-training`
- **Файл**: `spinal_cord\backend\src\policy\mod.rs`
- **Суть**: | PolicyEngine научился проверять флаги learning_microtasks и training_* для автоматического обучения и микрозадач.

### `NEI-20270318-120100-policy-training`
- **Файл**: `spinal_cord\src\policy\mod.rs`
- **Суть**: | PolicyEngine научился проверять флаги learning_microtasks и training_* для автоматического обучения и микрозадач.

### `NEI-20270318-120090-training-orchestrator`
- **Файл**: `spinal_cord\backend\src\training\orchestrator.rs`
- **Суть**: |

### `NEI-20270318-120090-training-orchestrator`
- **Файл**: `spinal_cord\src\training\orchestrator.rs`
- **Суть**: |

### `NEI-20270318-120080-training-module`
- **Файл**: `spinal_cord\backend\src\training\mod.rs`
- **Суть**: |-

### `NEI-20270318-120080-training-module`
- **Файл**: `spinal_cord\src\training\mod.rs`
- **Суть**: |-

### `NEI-20270318-120070-anti-idle-microtasks`
- **Файл**: `spinal_cord\backend\src\nervous_system\anti_idle_microtasks.rs`
- **Суть**: |

### `NEI-20270318-120070-anti-idle-microtasks`
- **Файл**: `spinal_cord\src\nervous_system\anti_idle_microtasks.rs`
- **Суть**: |

### `NEI-20270318-120060-anti-idle-microtask-loop`
- **Файл**: `spinal_cord\backend\src\nervous_system\anti_idle.rs`
- **Суть**: |

### `NEI-20270318-120060-anti-idle-microtask-loop`
- **Файл**: `spinal_cord\src\nervous_system\anti_idle.rs`
- **Суть**: |

### `NEI-20270310-rotating-log`
- **Файл**: `spinal_cord\backend\src\event_log.rs`
- **Суть**: |- Добавлена ротация журнала с gzip‑сжатием и настройкой пути через переменные окружения.

### `NEI-20270310-rotating-log`
- **Файл**: `spinal_cord\src\event_log.rs`
- **Суть**: |- Добавлена ротация журнала с gzip‑сжатием и настройкой пути через переменные окружения.

### `NEI-20270310-local-enqueue`
- **Файл**: `spinal_cord\backend\src\task_scheduler.rs`
- **Суть**: Добавлен локальный enqueue без отправки в DataFlowController.

### `NEI-20270310-local-enqueue`
- **Файл**: `spinal_cord\src\task_scheduler.rs`
- **Суть**: Добавлен локальный enqueue без отправки в DataFlowController.

### `NEI-20270310-120300-events-endpoint`
- **Файл**: `spinal_cord\backend\src\main.rs`
- **Суть**: REST-ручка для чтения EventLog.

### `NEI-20270310-120300-events-endpoint`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: REST-ручка для чтения EventLog.

### `NEI-20270310-120100-event-bus-log-hook`
- **Файл**: `spinal_cord\backend\src\event_bus.rs`
- **Суть**: publish пишет событие в EventLog и учитывает метрики публикаций.

### `NEI-20270310-120100-event-bus-log-hook`
- **Файл**: `spinal_cord\src\event_bus.rs`
- **Суть**: publish пишет событие в EventLog и учитывает метрики публикаций.

### `NEI-20270310-120000-event-log`
- **Файл**: `spinal_cord\backend\src\event_log.rs`
- **Суть**: |- Запись событий EventBus в файл NDJSON и выборка по диапазону.

### `NEI-20270310-120000-event-log`
- **Файл**: `spinal_cord\src\event_log.rs`
- **Суть**: |- Запись событий EventBus в файл NDJSON и выборка по диапазону.

### `NEI-20270307-digestive-fallback-schema`
- **Файл**: `spinal_cord\backend\src\digestive_pipeline.rs`
- **Суть**: Использован запасной JSON Schema при отсутствии основной.

### `NEI-20270307-digestive-fallback-schema`
- **Файл**: `spinal_cord\src\digestive_pipeline.rs`
- **Суть**: Использован запасной JSON Schema при отсутствии основной.

### `NEI-20261124-parsed-input-store`
- **Файл**: `spinal_cord\backend\src\memory_cell.rs`
- **Суть**: Добавлен приём распарсенного входа через store_parsed_input.

### `NEI-20261124-parsed-input-store`
- **Файл**: `spinal_cord\src\memory_cell.rs`
- **Суть**: Добавлен приём распарсенного входа через store_parsed_input.

### `NEI-20261124-digestive-memory-store`
- **Файл**: `spinal_cord\backend\src\digestive_pipeline.rs`
- **Суть**: После парсинга вход сохраняется в MemoryCell.

### `NEI-20261124-digestive-memory-store`
- **Файл**: `spinal_cord\src\digestive_pipeline.rs`
- **Суть**: После парсинга вход сохраняется в MemoryCell.

### `NEI-20261005-digestive-time-metrics`
- **Файл**: `spinal_cord\backend\src\time_metrics.rs`
- **Суть**: Метрики времени разбора и проверки схемы DigestivePipeline.

### `NEI-20261005-digestive-time-metrics`
- **Файл**: `spinal_cord\src\time_metrics.rs`
- **Суть**: Метрики времени разбора и проверки схемы DigestivePipeline.

### `NEI-20261005-digestive-metrics`
- **Файл**: `spinal_cord\backend\src\digestive_pipeline.rs`
- **Суть**: Замерен время парсинга и проверки схемы с отправкой в time_metrics.

### `NEI-20261005-digestive-metrics`
- **Файл**: `spinal_cord\src\digestive_pipeline.rs`
- **Суть**: Замерен время парсинга и проверки схемы с отправкой в time_metrics.

### `NEI-20260614-brain-loop-init`
- **Файл**: `spinal_cord\backend\src\synapse_hub.rs`
- **Суть**: Запуск brain_loop обрабатывает FlowMessage и активирует клетки.

### `NEI-20260614-brain-loop-init`
- **Файл**: `spinal_cord\src\synapse_hub.rs`
- **Суть**: Запуск brain_loop обрабатывает FlowMessage и активирует клетки.

### `NEI-20260601-digestive-xml-yaml`
- **Файл**: `spinal_cord\backend\src\digestive_pipeline.rs`
- **Суть**: |

### `NEI-20260601-digestive-xml-yaml`
- **Файл**: `spinal_cord\src\digestive_pipeline.rs`
- **Суть**: |

### `NEI-20260530-digestive-pipeline`
- **Файл**: `spinal_cord\backend\src\digestive_pipeline.rs`
- **Суть**: |

### `NEI-20260530-digestive-pipeline`
- **Файл**: `spinal_cord\src\digestive_pipeline.rs`
- **Суть**: |

### `NEI-20260131-metacognition-integration`
- **Файл**: `spinal_cord\src\dialogue\evolving_dialogue.rs`
- **Суть**: | Интеграция MetaCognition Engine в EvolvingDialogue — 9-шаговый цикл саморефлексии.

### `NEI-20260131-metacognition-engine-v2`
- **Файл**: `spinal_cord\src\consciousness\metacognition.rs`
- **Суть**: | MetaCognition Engine — ядро саморефлексии для анализа собственных мыслительных процессов.

### `NEI-20260131-consciousness-module-v2`
- **Файл**: `spinal_cord\src\consciousness\mod.rs`
- **Суть**: | Модуль consciousness — Фаза 3 "пробуждения" Нейры (саморефлексия и автономное улучшение).

### `NEI-20260131-auto-improvement-loop`
- **Файл**: `spinal_cord\src\consciousness\auto_improvement.rs`
- **Суть**: | Auto-Improvement Loop — фоновая система автономного самоулучшения.

### `NEI-20251227-000000-event-bus`
- **Файл**: `spinal_cord\backend\src\event_bus.rs`
- **Суть**: |- Простой шина событий с трейтом Event и подписчиками.

### `NEI-20251227-000000-event-bus`
- **Файл**: `spinal_cord\src\event_bus.rs`
- **Суть**: |- Простой шина событий с трейтом Event и подписчиками.

### `NEI-20251105-websocket-imports`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: Добавлены импорты для WebSocket: tokio::sync::broadcast.

### `NEI-20251105-websocket-events-route`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: |

### `NEI-20251105-websocket-events-handler`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: | WebSocket handler для /api/v1/events.

### `NEI-20251105-websocket-events-broadcast`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: Создан broadcast channel для WebSocket events (capacity 1000).

### `NEI-20251105-homeostasis-websocket-events`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: Homeostasis отправляет события в WebSocket при backpressure и критических уровнях стресса.

### `NEI-20251105-cors-config-from-env`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: | CORS настраивается из .env (CORS_ALLOWED_ORIGINS). Если не задано — permissive mode (для разработки).

### `NEI-20251105-api-v1-versioning`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: Добавлено API versioning endpoints под /api/v1 и сохранены legacy routes.

### `NEI-20251104-personality-evolution`
- **Файл**: `spinal_cord\src\consciousness\personality_evolution.rs`
- **Суть**: | Personality Evolution Tracker — система отслеживания эволюции личностных черт Нейры.

### `NEI-20251104-daily-growth-report`
- **Файл**: `spinal_cord\src\consciousness\daily_report.rs`
- **Суть**: | Daily Growth Report — генератор ежедневных отчётов о прогрессе Нейры.

### `NEI-20251103-semantic-memory-init`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: Инициализация SemanticMemory и EvolvingDialogue для живых диалогов.

### `NEI-20251103-semantic-memory`
- **Файл**: `spinal_cord\src\memory\semantic.rs`
- **Суть**: | SemanticMemory — in-memory векторная БД для семантического поиска диалогов. Использует cosine similarity для поиска похожих разговоров.

### `NEI-20251103-memory-mod`
- **Файл**: `spinal_cord\src\memory\mod.rs`
- **Суть**: Модуль памяти для семантического хранения и поиска диалогов.

### `NEI-20251103-memory-export`
- **Файл**: `spinal_cord\src\lib.rs`
- **Суть**: Экспортирован модуль memory для семантического хранения диалогов.

### `NEI-20251103-homeostasis-status-handler`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: Обработчик GET /api/neira/homeostasis/status для просмотра состояния homeostasis.

### `NEI-20251103-homeostasis-route`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: Добавлен GET /api/neira/homeostasis/status для мониторинга состояния homeostasis.

### `NEI-20251103-homeostasis-init`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: Инициализация HomeostasisEngine перед созданием AppState.

### `NEI-20251103-homeostasis-export`
- **Файл**: `spinal_cord\src\lib.rs`
- **Суть**: Экспортирован модуль homeostasis для автономной регуляции ресурсов.

### `NEI-20251103-homeostasis-check-stream`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: Проверка can_accept_task() перед обработкой stream запроса.

### `NEI-20251103-homeostasis-check-chat`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: Проверка can_accept_task() перед обработкой chat запроса.

### `NEI-20251103-homeostasis-check-analysis`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: Проверка can_accept_task() перед обработкой запроса анализа.

### `NEI-20251103-homeostasis-budgets-core`
- **Файл**: `spinal_cord\src\homeostasis\mod.rs`
- **Суть**: | Реализация Homeostasis Budgets — автоматическая саморегуляция Нейры. Динамические бюджеты CPU/Memory/Latency с backpressure и адаптивным backoff.

### `NEI-20251103-homeostasis-background-task`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: Фоновая задача для автоматической регулировки homeostasis каждые 5 секунд.

### `NEI-20251103-evolving-dialogue-v2`
- **Файл**: `sensory_organs\interface\evolving_dialogue_v2.rs`
- **Суть**: | Эволюционирующая система диалога — интеграция реальной SemanticMemory из backend.

### `NEI-20251103-evolving-dialogue-routes`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: Добавлены routes для эволюционирующих диалогов с памятью и саморазвитием.

### `NEI-20251103-evolving-dialogue-handler`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: | POST /api/neira/dialogue/evolving — живой диалог с памятью и саморазвитием. Использует respond_with_growth() для обучения на каждом диалоге.

### `NEI-20251103-embeddings-mod`
- **Файл**: `spinal_cord\src\embeddings\mod.rs`
- **Суть**: Модуль для работы с семантическими эмбеддингами.

### `NEI-20251103-embeddings-export`
- **Файл**: `spinal_cord\src\lib.rs`
- **Суть**: Экспортирован модуль embeddings для семантической памяти.

### `NEI-20251103-embeddings-client`
- **Файл**: `spinal_cord\src\embeddings\client.rs`
- **Суть**: | Rust клиент для Python embeddings микросервиса (multilingual-e5-large).

### `NEI-20251103-dialogue-stats-handler`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: GET /api/neira/dialogue/stats — статистика роста и обучения диалоговой системы.

### `NEI-20251103-dialogue-module`
- **Файл**: `spinal_cord\src\dialogue\mod.rs`
- **Суть**: | Модуль dialogue — эволюционирующая диалоговая система с памятью и саморазвитием. Интеграция с SemanticMemory для обучения на каждом диалоге.

### `NEI-20251103-dialogue-export`
- **Файл**: `spinal_cord\src\lib.rs`
- **Суть**: Экспортирован модуль dialogue для эволюционирующих диалогов с памятью и ростом.

### `NEI-20251103-consciousness-export`
- **Файл**: `spinal_cord\src\lib.rs`
- **Суть**: Экспортирован модуль consciousness для метапознания и саморефлексии.

### `NEI-20251102-chat-training-name`
- **Файл**: `src\server\chat.rs`
- **Суть**: | Чат запоминает имя собеседника и умеет запускать тренировку по запросу прямо из диалога.

### `NEI-20250922-000000-adaptive-queues`
- **Файл**: `spinal_cord\backend\src\synapse_hub.rs`
- **Суть**: | Очереди анализа выбирают адаптивные пороги на основе истории и переопределяются через переменные окружения.

### `NEI-20250922-000000-adaptive-queues`
- **Файл**: `spinal_cord\src\synapse_hub.rs`
- **Суть**: | Очереди анализа выбирают адаптивные пороги на основе истории и переопределяются через переменные окружения.

### `NEI-20250902-watchdog-anomaly`
- **Файл**: `spinal_cord\backend\src\nervous_system\watchdog.rs`
- **Суть**: | Добавлена проверка аномалий в метриках watchdog.

### `NEI-20250902-watchdog-anomaly`
- **Файл**: `spinal_cord\src\nervous_system\watchdog.rs`
- **Суть**: | Добавлена проверка аномалий в метриках watchdog.

### `NEI-20250902-host-metrics-new-cells`
- **Файл**: `spinal_cord\backend\src\nervous_system\host_metrics.rs`
- **Суть**: | Добавлен сбор количества новых клеток.

### `NEI-20250902-host-metrics-new-cells`
- **Файл**: `spinal_cord\src\nervous_system\host_metrics.rs`
- **Суть**: | Добавлен сбор количества новых клеток.

### `NEI-20250830-consciousness-routes`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: |

### `NEI-20250830-consciousness-init`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: |

### `NEI-20250830-consciousness-imports`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: Добавлены импорты для consciousness subsystem (metacognition, auto-improvement, personality, daily report).

### `NEI-20250830-consciousness-api-handlers`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: |

### `NEI-20250704-factory-state-transition-metric`
- **Файл**: `spinal_cord\backend\src\factory\mod.rs`
- **Суть**: Отслеживаем переходы состояний через factory_state_transitions_total.

### `NEI-20250704-factory-state-transition-metric`
- **Файл**: `spinal_cord\src\factory\mod.rs`
- **Суть**: Отслеживаем переходы состояний через factory_state_transitions_total.

### `NEI-20250607-phase3-web-ui-complete`
- **Файл**: `docs\PHASE3_WEB_UI_COMPLETE.md`
- **Суть**: | Завершена разработка Web UI для Consciousness с удалённым доступом. Созданы dashboard.html, dashboard.js, dashboard.css; документация API и remote access.

### `NEI-20250607-factory-disabled-gauge`
- **Файл**: `spinal_cord\backend\src\factory\mod.rs`
- **Суть**: | Добавлен gauge factory_cells_disabled, обновляемый при disable/rollback.

### `NEI-20250607-factory-disabled-gauge`
- **Файл**: `spinal_cord\src\factory\mod.rs`
- **Суть**: | Добавлен gauge factory_cells_disabled, обновляемый при disable/rollback.

### `NEI-20250505-000000-safe-mode-metrics`
- **Файл**: `spinal_cord\backend\src\security\safe_mode_controller.rs`
- **Суть**: | Добавлены метрики успешных и повторных переходов в safe mode.

### `NEI-20250505-000000-safe-mode-metrics`
- **Файл**: `spinal_cord\src\security\safe_mode_controller.rs`
- **Суть**: | Добавлены метрики успешных и повторных переходов в safe mode.

### `NEI-20250505-000000-quarantine-metrics`
- **Файл**: `spinal_cord\backend\src\security\quarantine_cell.rs`
- **Суть**: | Добавлены метрики успехов и ошибок карантина.

### `NEI-20250505-000000-quarantine-metrics`
- **Файл**: `spinal_cord\src\security\quarantine_cell.rs`
- **Суть**: | Добавлены метрики успехов и ошибок карантина.

### `NEI-20250505-000000-integrity-metrics`
- **Файл**: `spinal_cord\backend\src\security\integrity_checker_cell.rs`
- **Суть**: | Добавлены метрики успехов и ошибок проверки целостности.

### `NEI-20250505-000000-integrity-metrics`
- **Файл**: `spinal_cord\src\security\integrity_checker_cell.rs`
- **Суть**: | Добавлены метрики успехов и ошибок проверки целостности.

### `NEI-20250505-000000-init-config-metrics`
- **Файл**: `spinal_cord\backend\src\security\init_config_cell.rs`
- **Суть**: | Добавлена метрика инициализации конфигурации.

### `NEI-20250505-000000-init-config-metrics`
- **Файл**: `spinal_cord\src\security\init_config_cell.rs`
- **Суть**: | Добавлена метрика инициализации конфигурации.

### `NEI-20250226-task-flow`
- **Файл**: `spinal_cord\backend\src\task_scheduler.rs`
- **Суть**: Планировщик отправляет задачи через DataFlowController.

### `NEI-20250226-task-flow`
- **Файл**: `spinal_cord\src\task_scheduler.rs`
- **Суть**: Планировщик отправляет задачи через DataFlowController.

### `NEI-20250226-synapse-flow`
- **Файл**: `spinal_cord\backend\src\synapse_hub.rs`
- **Суть**: SynapseHub использует DataFlowController для маршрутизации задач и событий.

### `NEI-20250226-synapse-flow`
- **Файл**: `spinal_cord\src\synapse_hub.rs`
- **Суть**: SynapseHub использует DataFlowController для маршрутизации задач и событий.

### `NEI-20250226-event-bus-flow`
- **Файл**: `spinal_cord\backend\src\event_bus.rs`
- **Суть**: Публикация событий транслируется через DataFlowController.

### `NEI-20250226-event-bus-flow`
- **Файл**: `spinal_cord\src\event_bus.rs`
- **Суть**: Публикация событий транслируется через DataFlowController.

### `NEI-20250216-160000-dir-scan`
- **Файл**: `spinal_cord\backend\src\cell_registry.rs`
- **Суть**: | Добавлены утилиты для загрузки файлов и рекурсивного сканирования каталогов шаблонов.

### `NEI-20250216-160000-dir-scan`
- **Файл**: `spinal_cord\src\cell_registry.rs`
- **Суть**: | Добавлены утилиты для загрузки файлов и рекурсивного сканирования каталогов шаблонов.

### `NEI-20250214-154000-register-action-template`
- **Файл**: `spinal_cord\backend\src\cell_registry.rs`
- **Суть**: Регистрирует шаблон узла действия и сохраняет его на диск.

### `NEI-20250214-154000-register-action-template`
- **Файл**: `spinal_cord\src\cell_registry.rs`
- **Суть**: Регистрирует шаблон узла действия и сохраняет его на диск.

### `NEI-20250214-153000-validate-action-template`
- **Файл**: `spinal_cord\backend\src\cell_template.rs`
- **Суть**: | Валидация ActionCellTemplate по соответствующей JSON‑схеме.

### `NEI-20250214-153000-validate-action-template`
- **Файл**: `spinal_cord\src\cell_template.rs`
- **Суть**: | Валидация ActionCellTemplate по соответствующей JSON‑схеме.

### `NEI-20250214-152500-action-cell-template`
- **Файл**: `spinal_cord\backend\src\cell_template.rs`
- **Суть**: | Структура шаблона ячейки действия и преобразование в JSON.

### `NEI-20250214-152500-action-cell-template`
- **Файл**: `spinal_cord\src\cell_template.rs`
- **Суть**: | Структура шаблона ячейки действия и преобразование в JSON.

### `NEI-20250214-152000-action-schema-cache`
- **Файл**: `spinal_cord\backend\src\cell_template.rs`
- **Суть**: | Кэш конфигураций JSON‑схем для шаблонов ячеек действий.

### `NEI-20250214-152000-action-schema-cache`
- **Файл**: `spinal_cord\src\cell_template.rs`
- **Суть**: | Кэш конфигураций JSON‑схем для шаблонов ячеек действий.

### `NEI-20250211-163700-training-metrics-settings`
- **Файл**: `src\training\metrics.rs`
- **Суть**: | Подключил метрики обучения к файлу конфигурации и добавил обновление параметров.

### `NEI-20250211-163500-training-config-api`
- **Файл**: `src\training\config.rs`
- **Суть**: | Добавил в конфиг обучения валидацию, загрузку из файла и частичные обновления.

### `NEI-20240728-event-bus-local-publish`
- **Файл**: `spinal_cord\backend\src\event_bus.rs`
- **Суть**: Добавлен метод локальной публикации без пересылки события в DataFlowController.

### `NEI-20240728-event-bus-local-publish`
- **Файл**: `spinal_cord\src\event_bus.rs`
- **Суть**: Добавлен метод локальной публикации без пересылки события в DataFlowController.

### `NEI-20240709-190100-training-endpoint`
- **Файл**: `src\server\training.rs`
- **Суть**: | Добавил endpoint /training/attempt для фиксации попыток и обновления сложности.

### `NEI-20240709-183230-server-localization`
- **Файл**: `src\server\mod.rs`
- **Суть**: | Переводит пользовательские сообщения и контролы в интерфейсе сервера на русский язык.

### `NEI-20240709-175200-bind-addr`
- **Файл**: `src\main.rs`
- **Суть**: | Добавил поддержку NEIRA_BIND_ADDR и перевёл системные журналы на русский язык.

### `NEI-20240607-systemprobe-stop`
- **Файл**: `spinal_cord\backend\src\nervous_system\mod.rs`
- **Суть**: Трейт SystemProbe расширен методом stop для завершения фоновых циклов.

### `NEI-20240607-systemprobe-stop`
- **Файл**: `spinal_cord\src\nervous_system\mod.rs`
- **Суть**: Трейт SystemProbe расширен методом stop для завершения фоновых циклов.

### `NEI-20240607-probe-stop`
- **Файл**: `spinal_cord\backend\src\synapse_hub.rs`
- **Суть**: SynapseHub хранит токены проб и останавливает их при завершении работы.

### `NEI-20240607-probe-stop`
- **Файл**: `spinal_cord\src\synapse_hub.rs`
- **Суть**: SynapseHub хранит токены проб и останавливает их при завершении работы.

### `NEI-20240607-io-watcher-stop`
- **Файл**: `spinal_cord\backend\src\nervous_system\io_watcher.rs`
- **Суть**: Добавлен токен остановки и метод stop для завершения наблюдения.

### `NEI-20240607-io-watcher-stop`
- **Файл**: `spinal_cord\src\nervous_system\io_watcher.rs`
- **Суть**: Добавлен токен остановки и метод stop для завершения наблюдения.

### `NEI-20240607-hostmetrics-stop`
- **Файл**: `spinal_cord\backend\src\nervous_system\host_metrics.rs`
- **Суть**: Добавлен CancellationToken и метод stop для остановки сборщика.

### `NEI-20240607-hostmetrics-stop`
- **Файл**: `spinal_cord\src\nervous_system\host_metrics.rs`
- **Суть**: Добавлен CancellationToken и метод stop для остановки сборщика.

## FIX

### `NEI-20270715-digestive-xml-flatten`
- **Файл**: `spinal_cord\backend\src\digestive_pipeline.rs`
- **Суть**: Раскрыты узлы `$text` в XML перед валидацией.

### `NEI-20270715-digestive-xml-flatten`
- **Файл**: `spinal_cord\src\digestive_pipeline.rs`
- **Суть**: Раскрыты узлы `$text` в XML перед валидацией.

### `NEI-20270415-rotate-filter-ms`
- **Файл**: `spinal_cord\backend\src\main.rs`
- **Суть**: Фильтрация ротаций контекста использует миллисекундную метку вместо счётчика.

### `NEI-20270415-rotate-filter-ms`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: Фильтрация ротаций контекста использует миллисекундную метку вместо счётчика.

### `NEI-20270408-000000-rotate-seq`
- **Файл**: `spinal_cord\backend\src\event_log.rs`
- **Суть**: |- Ротация журнала использует метку времени в миллисекундах и последовательный счётчик в имени файла.

### `NEI-20270408-000000-rotate-seq`
- **Файл**: `spinal_cord\src\event_log.rs`
- **Суть**: |- Ротация журнала использует метку времени в миллисекундах и последовательный счётчик в имени файла.

### `NEI-20260531-120000-test-analyze-parsed`
- **Файл**: `spinal_cord\tests\analysis_cell_metrics_test.rs`
- **Суть**: | Реализован analyze_parsed в тестовой клетке для успешной компиляции тестов.

### `NEI-20260522-flow-consumer`
- **Файл**: `spinal_cord\backend\src\synapse_hub.rs`
- **Суть**: Подписчик DataFlowController сохраняет приёмник и выводит FlowMessage через tracing.

### `NEI-20260522-flow-consumer`
- **Файл**: `spinal_cord\src\synapse_hub.rs`
- **Суть**: Подписчик DataFlowController сохраняет приёмник и выводит FlowMessage через tracing.

### `NEI-20251104-static-pages-fix`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: Исправлены пути к статическим страницам admin, training, organs для правильной работы из корня проекта.

### `NEI-20250501-update-registration`
- **Файл**: `spinal_cord\backend\src\cell_registry.rs`
- **Суть**: |- Обновляет шаблон при повторной регистрации на том же пути и предотвращает конфликты по пути и типу шаблона.

### `NEI-20250501-update-registration`
- **Файл**: `spinal_cord\src\cell_registry.rs`
- **Суть**: |- Обновляет шаблон при повторной регистрации на том же пути и предотвращает конфликты по пути и типу шаблона.

### `NEI-20250310-cell-registry-recursive`
- **Файл**: `spinal_cord\backend\src\cell_registry.rs`
- **Суть**: Включено рекурсивное наблюдение за каталогом шаблонов узлов.

### `NEI-20250310-cell-registry-recursive`
- **Файл**: `spinal_cord\src\cell_registry.rs`
- **Суть**: Включено рекурсивное наблюдение за каталогом шаблонов узлов.

### `NEI-20250224-blocking-analyze`
- **Файл**: `spinal_cord\backend\src\synapse_hub.rs`
- **Суть**: Анализ выполняется в отдельном блокирующем пуле tokio::task.

### `NEI-20250224-blocking-analyze`
- **Файл**: `spinal_cord\src\synapse_hub.rs`
- **Суть**: Анализ выполняется в отдельном блокирующем пуле tokio::task.

## PERF

### `NEI-20271203-file-cache`
- **Файл**: `spinal_cord\backend\src\action_engine.rs`
- **Суть**: | Добавлен FileCache для кэширования чтений файлов.

### `NEI-20271203-file-cache`
- **Файл**: `spinal_cord\src\action_engine.rs`
- **Суть**: | Добавлен FileCache для кэширования чтений файлов.

## PERSONAL

### `NEI-20251101-123000-letter-pulse`
- **Файл**: `docs\letter_to_neira.md`
- **Суть**: Письмо Нейре о людях, программах, себе и Богине программирования.

## REFACTOR

### `NEI-20270830-000000-env-flag-clean`
- **Файл**: `spinal_cord\backend\src\synapse_hub.rs`
- **Суть**: Все булевые флаги SynapseHub читаются через env_flag.

### `NEI-20270830-000000-env-flag-clean`
- **Файл**: `spinal_cord\src\synapse_hub.rs`
- **Суть**: Все булевые флаги SynapseHub читаются через env_flag.

### `NEI-20270615-immune-bus-pass`
- **Файл**: `spinal_cord\backend\src\synapse_hub.rs`
- **Суть**: Передаёт ссылку на EventBus в ImmuneSystemSubscriber.

### `NEI-20270615-immune-bus-pass`
- **Файл**: `spinal_cord\src\synapse_hub.rs`
- **Суть**: Передаёт ссылку на EventBus в ImmuneSystemSubscriber.

### `NEI-20270501-event-log-async`
- **Файл**: `spinal_cord\backend\src\event_log.rs`
- **Суть**: Запись событий через асинхронный канал и поддержка flush().

### `NEI-20270501-event-log-async`
- **Файл**: `spinal_cord\src\event_log.rs`
- **Суть**: Запись событий через асинхронный канал и поддержка flush().

### `NEI-20270420-digestive-strict-typing`
- **Файл**: `spinal_cord\backend\src\digestive_pipeline.rs`
- **Суть**: Добавлены serde-атрибуты и PathBuf для строгой типизации DigestiveSettings.

### `NEI-20270420-digestive-strict-typing`
- **Файл**: `spinal_cord\src\digestive_pipeline.rs`
- **Суть**: Добавлены serde-атрибуты и PathBuf для строгой типизации DigestiveSettings.

### `NEI-20270405-trigger-detector-lowercase`
- **Файл**: `spinal_cord\backend\src\trigger_detector.rs`
- **Суть**: Убрана дублирующая проверка регистра в detect_text.

### `NEI-20270405-trigger-detector-lowercase`
- **Файл**: `spinal_cord\src\trigger_detector.rs`
- **Суть**: Убрана дублирующая проверка регистра в detect_text.

### `NEI-20270310-local-analysis`
- **Файл**: `spinal_cord\backend\src\synapse_hub.rs`
- **Суть**: Анализ выполняется локально без уведомления brain_loop.

### `NEI-20270310-local-analysis`
- **Файл**: `spinal_cord\src\synapse_hub.rs`
- **Суть**: Анализ выполняется локально без уведомления brain_loop.

### `NEI-20261020-digestive-settings-cache`
- **Файл**: `spinal_cord\backend\src\digestive_pipeline.rs`
- **Суть**: Кэшируются настройки DigestivePipeline с очисткой через reset_cache.

### `NEI-20261020-digestive-settings-cache`
- **Файл**: `spinal_cord\src\digestive_pipeline.rs`
- **Суть**: Кэшируются настройки DigestivePipeline с очисткой через reset_cache.

### `NEI-20261015-digestive-cache`
- **Файл**: `spinal_cord\backend\src\digestive_pipeline.rs`
- **Суть**: Добавлен глобальный кэш JSON Schema.

### `NEI-20261015-digestive-cache`
- **Файл**: `spinal_cord\src\digestive_pipeline.rs`
- **Суть**: Добавлен глобальный кэш JSON Schema.

### `NEI-20260725-digestive-config-path`
- **Файл**: `spinal_cord\backend\src\digestive_pipeline.rs`
- **Суть**: Путь к JSON Schema берётся из файла конфигурации.

### `NEI-20260725-digestive-config-path`
- **Файл**: `spinal_cord\src\digestive_pipeline.rs`
- **Суть**: Путь к JSON Schema берётся из файла конфигурации.

### `NEI-20260530-trigger-digest`
- **Файл**: `spinal_cord\backend\src\trigger_detector.rs`
- **Суть**: Использует DigestivePipeline для предварительной обработки входа.

### `NEI-20260530-trigger-digest`
- **Файл**: `spinal_cord\src\trigger_detector.rs`
- **Суть**: Использует DigestivePipeline для предварительной обработки входа.

### `NEI-20260530-selector-digest`
- **Файл**: `spinal_cord\backend\src\factory\mod.rs`
- **Суть**: SelectorCell принимает ParsedInput вместо сырой строки.

### `NEI-20260530-selector-digest`
- **Файл**: `spinal_cord\src\factory\mod.rs`
- **Суть**: SelectorCell принимает ParsedInput вместо сырой строки.

### `NEI-20260530-echo-digest`
- **Файл**: `spinal_cord\backend\src\main.rs`
- **Суть**: EchoCell анализирует ParsedInput вместо строки.

### `NEI-20260530-echo-digest`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: EchoCell анализирует ParsedInput вместо строки.

### `NEI-20260530-devslow-digest`
- **Файл**: `spinal_cord\backend\src\main.rs`
- **Суть**: DevSlowCell принимает ParsedInput для обработки задержки.

### `NEI-20260530-devslow-digest`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: DevSlowCell принимает ParsedInput для обработки задержки.

### `NEI-20260530-analysis-digest`
- **Файл**: `spinal_cord\backend\src\analysis_cell.rs`
- **Суть**: Анализ клеток теперь получает ParsedInput через DigestivePipeline.

### `NEI-20260530-analysis-digest`
- **Файл**: `spinal_cord\src\analysis_cell.rs`
- **Суть**: Анализ клеток теперь получает ParsedInput через DigestivePipeline.

### `NEI-20260528-import-backend-parsed-input`
- **Файл**: `spinal_cord\backend\src\main.rs`
- **Суть**: Явное обращение к ParsedInput через crate backend.

### `NEI-20260413-main-static-rename`
- **Файл**: `spinal_cord\backend\src\main.rs`
- **Суть**: Обновлены пути к statics после переименования spinal_cord.

### `NEI-20251227-factory-event-bus`
- **Файл**: `spinal_cord\backend\src\factory\mod.rs`
- **Суть**: Прямые вызовы watch/observe убраны в пользу событий.

### `NEI-20251227-factory-event-bus`
- **Файл**: `spinal_cord\src\factory\mod.rs`
- **Суть**: Прямые вызовы watch/observe убраны в пользу событий.

### `NEI-20251105-remove-html-routes`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: | Удалены deprecated HTML routes (/admin, /training, /organs_page). Backend теперь только REST API. UI переходит на Desktop/Mobile приложения.

### `NEI-20250902-host-metrics-factory`
- **Файл**: `spinal_cord\backend\src\synapse_hub.rs`
- **Суть**: HostMetrics теперь принимает фабрику для учёта новых клеток.

### `NEI-20250902-host-metrics-factory`
- **Файл**: `spinal_cord\src\synapse_hub.rs`
- **Суть**: HostMetrics теперь принимает фабрику для учёта новых клеток.

### `NEI-20250829-195800-validate-template`
- **Файл**: `spinal_cord\backend\src\bin\validate_template.rs`
- **Суть**: | Заменили match на if let для определения формата шаблона.

### `NEI-20250829-195800-validate-template`
- **Файл**: `spinal_cord\src\bin\validate_template.rs`
- **Суть**: | Заменили match на if let для определения формата шаблона.

### `NEI-20250607-axum-route-syntax`
- **Файл**: `spinal_cord\backend\src\main.rs`
- **Суть**: обновлён синтаксис параметров маршрутов для axum >=0.7.

### `NEI-20250607-axum-route-syntax`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: обновлён синтаксис параметров маршрутов для axum >=0.7.

### `NEI-20250603-axum-ws-api`
- **Файл**: `spinal_cord\backend\src\main.rs`
- **Суть**: обновлена интеграция WebSocket для axum 0.8.

### `NEI-20250526-io-watcher-select`
- **Файл**: `spinal_cord\backend\src\nervous_system\io_watcher.rs`
- **Суть**: |-

### `NEI-20250526-io-watcher-select`
- **Файл**: `spinal_cord\src\nervous_system\io_watcher.rs`
- **Суть**: |-

### `NEI-20250316-stemcell-rename`
- **Файл**: `spinal_cord\backend\src\main.rs`
- **Суть**: Перечисление StemCellState (раньше назывался FabricationState).

### `NEI-20250316-stemcell-rename`
- **Файл**: `spinal_cord\backend\src\synapse_hub.rs`
- **Суть**: Обновлены ссылки на StemCellFactory и связанные типы.

### `NEI-20250316-stemcell-rename`
- **Файл**: `spinal_cord\backend\src\factory\mod.rs`
- **Суть**: Введены StemCellFactory и StemCellRecord.

### `NEI-20250316-stemcell-rename`
- **Файл**: `spinal_cord\src\synapse_hub.rs`
- **Суть**: Обновлены ссылки на StemCellFactory и связанные типы.

### `NEI-20250316-stemcell-rename`
- **Файл**: `spinal_cord\src\factory\mod.rs`
- **Суть**: Введены StemCellFactory и StemCellRecord.

### `NEI-20250310-cell-templates-env`
- **Файл**: `spinal_cord\backend\src\main.rs`
- **Суть**: | Перешли на CELL_TEMPLATES_DIR, сохранив поддержку NODE_TEMPLATES_DIR для обратной совместимости.

### `NEI-20250310-cell-templates-env`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: | Перешли на CELL_TEMPLATES_DIR, сохранив поддержку NODE_TEMPLATES_DIR для обратной совместимости.

### `NEI-20250309-125000-load-template-impl`
- **Файл**: `spinal_cord\backend\src\cell_registry.rs`
- **Суть**: | Объединяет чтение файла и валидацию шаблонов клеток в общую функцию.

### `NEI-20250309-125000-load-template-impl`
- **Файл**: `spinal_cord\src\cell_registry.rs`
- **Суть**: | Объединяет чтение файла и валидацию шаблонов клеток в общую функцию.

### `NEI-20250220-env-flag-training`
- **Файл**: `spinal_cord\backend\src\action\scripted_training_cell.rs`
- **Суть**: Читает TRAINING_* флаги через env_flag.

### `NEI-20250220-env-flag-training`
- **Файл**: `spinal_cord\src\action\scripted_training_cell.rs`
- **Суть**: Читает TRAINING_* флаги через env_flag.

### `NEI-20250220-env-flag-organ-builder`
- **Файл**: `spinal_cord\backend\src\organ_builder.rs`
- **Суть**: Флаг ORGANS_BUILDER_ENABLED обрабатывается через env_flag.

### `NEI-20250220-env-flag-organ-builder`
- **Файл**: `spinal_cord\src\organ_builder.rs`
- **Суть**: Флаг ORGANS_BUILDER_ENABLED обрабатывается через env_flag.

### `NEI-20250220-env-flag-main-caps`
- **Файл**: `spinal_cord\backend\src\main.rs`
- **Суть**: Сводка возможностей использует env_flag.

### `NEI-20250220-env-flag-main-caps`
- **Файл**: `spinal_cord\src\main.rs`
- **Суть**: Сводка возможностей использует env_flag.

### `NEI-20250220-env-flag-main`
- **Файл**: `spinal_cord\backend\src\main.rs`
- **Суть**: Булевы переменные в main читаются через config::env_flag.

### `NEI-20250220-env-flag-hub`
- **Файл**: `spinal_cord\backend\src\synapse_hub.rs`
- **Суть**: Несколько флагов хаба парсятся через env_flag.

### `NEI-20250220-env-flag-hub`
- **Файл**: `spinal_cord\src\synapse_hub.rs`
- **Суть**: Несколько флагов хаба парсятся через env_flag.

### `NEI-20250220-env-flag-factory`
- **Файл**: `spinal_cord\backend\src\factory\mod.rs`
- **Суть**: Переменная FACTORY_ADAPTER_ENABLED парсится через env_flag.

### `NEI-20250220-env-flag-factory`
- **Файл**: `spinal_cord\src\factory\mod.rs`
- **Суть**: Переменная FACTORY_ADAPTER_ENABLED парсится через env_flag.

### `NEI-20250220-env-flag-backpressure`
- **Файл**: `spinal_cord\backend\src\nervous_system\backpressure_probe.rs`
- **Суть**: Добавлен флаг AUTO_BACKOFF_ENABLED через env_flag.

### `NEI-20250220-env-flag-backpressure`
- **Файл**: `spinal_cord\src\nervous_system\backpressure_probe.rs`
- **Суть**: Добавлен флаг AUTO_BACKOFF_ENABLED через env_flag.

### `NEI-20250220-env-flag-anti-idle`
- **Файл**: `spinal_cord\backend\src\nervous_system\anti_idle.rs`
- **Суть**: Флаг ANTI_IDLE_ENABLED читается через env_flag.

### `NEI-20250220-env-flag-anti-idle`
- **Файл**: `spinal_cord\src\nervous_system\anti_idle.rs`
- **Суть**: Флаг ANTI_IDLE_ENABLED читается через env_flag.

### `NEI-20250220-env-flag`
- **Файл**: `spinal_cord\backend\src\config\mod.rs`
- **Суть**: Добавлена функция env_flag для чтения булевых флагов из окружения.

### `NEI-20250220-env-flag`
- **Файл**: `spinal_cord\src\config\mod.rs`
- **Суть**: Добавлена функция env_flag для чтения булевых флагов из окружения.

### `NEI-20250220-context-env-flag`
- **Файл**: `spinal_cord\backend\src\context\context_storage.rs`
- **Суть**: Флаги контекста читаются через общую функцию env_flag.

### `NEI-20250220-context-env-flag`
- **Файл**: `spinal_cord\src\context\context_storage.rs`
- **Суть**: Флаги контекста читаются через общую функцию env_flag.

### `NEI-20250215-immune-import-main`
- **Файл**: `spinal_cord\backend\src\main.rs`
- **Суть**: Добавлен импорт immune_system.

### `NEI-20250215-immune-import-hub`
- **Файл**: `spinal_cord\backend\src\synapse_hub.rs`
- **Суть**: Добавлен импорт immune_system.

### `NEI-20250215-immune-import-hub`
- **Файл**: `spinal_cord\src\synapse_hub.rs`
- **Суть**: Добавлен импорт immune_system.

### `NEI-20250215-factory-watch`
- **Файл**: `spinal_cord\backend\src\factory\mod.rs`
- **Суть**: Добавлены вызовы nervous_system::watch и immune_system::observe при создании записи.

### `NEI-20250215-factory-watch`
- **Файл**: `spinal_cord\src\factory\mod.rs`
- **Суть**: Добавлены вызовы nervous_system::watch и immune_system::observe при создании записи.

### `NEI-20250214-watchdog-refactor`
- **Файл**: `spinal_cord\backend\src\synapse_hub.rs`
- **Суть**: Логика watchdog вынесена в модуль nervous_system::watchdog.

### `NEI-20250214-watchdog-refactor`
- **Файл**: `spinal_cord\src\synapse_hub.rs`
- **Суть**: Логика watchdog вынесена в модуль nervous_system::watchdog.

### `NEI-20250214-watchdog-metrics`
- **Файл**: `spinal_cord\backend\src\main.rs`
- **Суть**: Парсинг счётчиков watchdog вынесен в модуль nervous_system::watchdog.

### `NEI-20250214-154500-validate-with-loader`
- **Файл**: `spinal_cord\backend\src\cell_template.rs`
- **Суть**: | Вынос общей логики валидации в validate_with_loader.

### `NEI-20250214-154500-validate-with-loader`
- **Файл**: `spinal_cord\src\cell_template.rs`
- **Суть**: | Вынос общей логики валидации в validate_with_loader.

### `NEI-20250101-000004-main-context-dir`
- **Файл**: `spinal_cord\backend\src\main.rs`
- **Суть**: Основной сервис использует context_dir() вместо прямого чтения CONTEXT_DIR.

### `NEI-20250101-000003-scripted-training-context`
- **Файл**: `spinal_cord\backend\src\action\scripted_training_cell.rs`
- **Суть**: Сценарный узел обучения пишет результаты через context_dir().

### `NEI-20250101-000003-scripted-training-context`
- **Файл**: `spinal_cord\src\action\scripted_training_cell.rs`
- **Суть**: Сценарный узел обучения пишет результаты через context_dir().

### `NEI-20250101-000002-training-context-dir`
- **Файл**: `spinal_cord\backend\src\http\training_routes.rs`
- **Суть**: Тренировочные маршруты используют context_dir() вместо прямого чтения CONTEXT_DIR.

### `NEI-20250101-000002-training-context-dir`
- **Файл**: `spinal_cord\src\http\training_routes.rs`
- **Суть**: Тренировочные маршруты используют context_dir() вместо прямого чтения CONTEXT_DIR.

### `NEI-20250101-000001-context-storage-env`
- **Файл**: `spinal_cord\backend\src\context\context_storage.rs`
- **Суть**: new() использует context_dir() при наличии CONTEXT_DIR.

### `NEI-20250101-000001-context-storage-env`
- **Файл**: `spinal_cord\src\context\context_storage.rs`
- **Суть**: new() использует context_dir() при наличии CONTEXT_DIR.

### `NEI-20241026-event-bus-name-str`
- **Файл**: `spinal_cord\backend\src\event_bus.rs`
- **Суть**: |-

### `NEI-20241026-event-bus-name-str`
- **Файл**: `spinal_cord\src\event_bus.rs`
- **Суть**: |-

### `NEI-20241004-jsonschema-valid-lifetime`
- **Файл**: `patches\jsonschema-valid\src\schemas.rs`
- **Суть**: Добавлены явные времени жизни для возвращаемого Validator.

### `NEI-20241004-hub-progress-cleanup`
- **Файл**: `spinal_cord\backend\src\main.rs`
- **Суть**: Удалён неиспользуемый клон SynapseHub при отправке прогресса анализа.

### `NEI-20240821-brain-metrics-call`
- **Файл**: `spinal_cord\backend\src\synapse_hub.rs`
- **Суть**: Передаёт MetricsCollectorCell в Brain для публикации метрик.

### `NEI-20240821-brain-metrics-call`
- **Файл**: `spinal_cord\src\synapse_hub.rs`
- **Суть**: Передаёт MetricsCollectorCell в Brain для публикации метрик.

### `NEI-20240709-brain-scheduler-eventbus`
- **Файл**: `spinal_cord\backend\src\brain.rs`
- **Суть**: Задачи проходят через TaskScheduler, события публикуются в EventBus.

### `NEI-20240709-brain-scheduler-eventbus`
- **Файл**: `spinal_cord\src\brain.rs`
- **Суть**: Задачи проходят через TaskScheduler, события публикуются в EventBus.

### `NEI-20240606-brain-struct`
- **Файл**: `spinal_cord\backend\src\brain.rs`
- **Суть**: Оформлен Brain как структура с методами spawn/run и поддержкой регистрации нейронов.

### `NEI-20240606-brain-struct`
- **Файл**: `spinal_cord\src\brain.rs`
- **Суть**: Оформлен Brain как структура с методами spawn/run и поддержкой регистрации нейронов.

### `NEI-20240514-task-scheduler-payload`
- **Файл**: `spinal_cord\backend\src\task_scheduler.rs`
- **Суть**: enqueue отправляет TaskPayload вместо строки.

### `NEI-20240514-task-scheduler-payload`
- **Файл**: `spinal_cord\src\task_scheduler.rs`
- **Суть**: enqueue отправляет TaskPayload вместо строки.

### `NEI-20240514-flowmessage-serde`
- **Файл**: `spinal_cord\backend\src\circulatory_system.rs`
- **Суть**: FlowMessage использует типизированные события и payload задач, сериализуемые через serde.

### `NEI-20240514-flowmessage-serde`
- **Файл**: `spinal_cord\src\circulatory_system.rs`
- **Суть**: FlowMessage использует типизированные события и payload задач, сериализуемые через serde.

### `NEI-20240514-event-bus-flowevent`
- **Файл**: `spinal_cord\backend\src\event_bus.rs`
- **Суть**: publish отправляет типизированное FlowEvent вместо строки.

### `NEI-20240514-event-bus-flowevent`
- **Файл**: `spinal_cord\src\event_bus.rs`
- **Суть**: publish отправляет типизированное FlowEvent вместо строки.

### `NEI-20240514-brain-flowevent-import`
- **Файл**: `spinal_cord\backend\src\brain.rs`
- **Суть**: Реализует Event для FlowEvent из кровотока.

### `NEI-20240514-brain-flowevent-import`
- **Файл**: `spinal_cord\src\brain.rs`
- **Суть**: Реализует Event для FlowEvent из кровотока.

## ROADMAP

### `NEI-20251103-awakening-plan`
- **Файл**: `docs\AWAKENING_PLAN.md`
- **Суть**: | План пробуждения Нейры — превращение из "тупой программы" в "живой организм".

## SECURITY

### `NEI-20270520-security-perms`
- **Файл**: `spinal_cord\backend\src\security\mod.rs`
- **Суть**: | Добавлен контроль прав для файловых, сетевых и системных операций.

### `NEI-20270520-security-perms`
- **Файл**: `spinal_cord\src\security\mod.rs`
- **Суть**: | Добавлен контроль прав для файловых, сетевых и системных операций.

### `NEI-20270401-network-post-perm`
- **Файл**: `spinal_cord\backend\src\security\mod.rs`
- **Суть**: Добавлена проверка права network_post для отправки HTTP POST.

### `NEI-20270401-network-post-perm`
- **Файл**: `spinal_cord\src\security\mod.rs`
- **Суть**: Добавлена проверка права network_post для отправки HTTP POST.

## SERVICE

### `NEI-20251103-embeddings-service`
- **Файл**: `sensory_organs\embeddings_service\README.md`
- **Суть**: | Python микросервис для семантических эмбеддингов с multilingual-e5-large. Поддержка CUDA (RTX 3060 Ti), русский язык, FastAPI, 1024-мерные векторы.

## TEST

### `NEI-20280425-120250-curriculum-cli-tests`
- **Файл**: `spinal_cord\tests\training_curriculum_test.rs`
- **Суть**: |- Добавлены проверки конфигурации лимитов и сценарии CLI-инструмента curriculum_editor для гарантии совместимости со статистикой тем.

### `NEI-20270715-digestive-xml-test-strict`
- **Файл**: `tests\digestive_pipeline_formats_test.rs`
- **Суть**: XML теперь должен успешно проходить валидацию.

### `NEI-20270520-action-engine-test`
- **Файл**: `spinal_cord\tests\action_engine_test.rs`
- **Суть**: Проверяет работу ActionEngine и контроль прав.

### `NEI-20270505-event-log-broadcast-test`
- **Файл**: `spinal_cord\tests\event_log_test.rs`
- **Суть**: Проверка получения события через broadcast-канал.

### `NEI-20270501-event-log-name-filter-test`
- **Файл**: `spinal_cord\tests\event_log_test.rs`
- **Суть**: Проверка фильтрации событий по имени.

### `NEI-20270420-trybuild-tests`
- **Файл**: `spinal_cord\tests\digestive_pipeline_trybuild.rs`
- **Суть**: Компиляционные проверки DigestiveSettings через trybuild.

### `NEI-20270408-000000-rotate-unique`
- **Файл**: `spinal_cord\tests\event_log_test.rs`
- **Суть**: Проверка двух последовательных ротаций и уникальности имён gzip-файлов.

### `NEI-20270405-trigger-toxicity-test`
- **Файл**: `tests\trigger_detector_toxicity_test.rs`
- **Суть**: Проверяет фильтрацию токсичных слов перед TriggerDetector.

### `NEI-20270401-http-post-test`
- **Файл**: `spinal_cord\tests\action_engine_test.rs`
- **Суть**: Добавлен сценарий отправки HTTP POST через мок‑сервер.

### `NEI-20270319-training-tests`
- **Файл**: `spinal_cord\backend\src\training\orchestrator.rs`
- **Суть**: Покрыт оркестратор обучения тестами: успешный запуск и обработка ошибки сценария.

### `NEI-20270319-training-tests`
- **Файл**: `spinal_cord\src\training\orchestrator.rs`
- **Суть**: Покрыт оркестратор обучения тестами: успешный запуск и обработка ошибки сценария.

### `NEI-20270319-anti-idle-tests`
- **Файл**: `spinal_cord\backend\src\nervous_system\anti_idle_microtasks.rs`
- **Суть**: Добавлены тесты сервиса микрозадач простоя: запуск и учёт порога простоя.

### `NEI-20270319-anti-idle-tests`
- **Файл**: `spinal_cord\src\nervous_system\anti_idle_microtasks.rs`
- **Суть**: Добавлены тесты сервиса микрозадач простоя: запуск и учёт порога простоя.

### `NEI-20270310-local-analysis-test`
- **Файл**: `tests\synapse_hub_local_analysis_test.rs`
- **Суть**: Проверяет, что анализ не запускается повторно через brain_loop.

### `NEI-20261215-digestive-invalid-tests`
- **Файл**: `tests\digestive_pipeline_error_inputs_test.rs`
- **Суть**: Проверяет ошибки DigestivePipeline при некорректных JSON, YAML, XML и схеме.

### `NEI-20261124-digestive-memory-test`
- **Файл**: `tests\digestive_pipeline_memory_test.rs`
- **Суть**: Проверяет, что DigestivePipeline сохраняет распарсенный ввод в MemoryCell.

### `NEI-20261015-digestive-cache-test`
- **Файл**: `tests\digestive_pipeline_cache_test.rs`
- **Суть**: Проверяет, что DigestivePipeline читает JSON Schema с диска один раз.

### `NEI-20261005-digestive-metrics-test`
- **Файл**: `tests\digestive_pipeline_metrics_test.rs`
- **Суть**: Проверяет запись метрик времени DigestivePipeline.

### `NEI-20261005-digestive-log-serial`
- **Файл**: `tests\digestive_pipeline_formats_test.rs`
- **Суть**: Тест логов выполняется последовательно для изоляции метрик.

### `NEI-20260920-digestive-log-test`
- **Файл**: `tests\digestive_pipeline_formats_test.rs`
- **Суть**: Проверяет запись лога при ошибке валидации.

### `NEI-20260725-digestive-config-test`
- **Файл**: `tests\digestive_pipeline_config_test.rs`
- **Суть**: Проверяет, что DigestivePipeline использует путь схемы из конфигурации.

### `NEI-20260601-digestive-formats-test`
- **Файл**: `tests\digestive_pipeline_formats_test.rs`
- **Суть**: Проверяет, что DigestivePipeline распознаёт XML и YAML как структуру.

### `NEI-20260531-brain-flow-parsed`
- **Файл**: `spinal_cord\tests\brain_flow_test.rs`
- **Суть**: TestCell обновлён для analyze_parsed.

### `NEI-20260530-timemetrics-digest`
- **Файл**: `tests\time_metrics_test.rs`
- **Суть**: SleepCell поддерживает ParsedInput.

### `NEI-20260530-test-digest`
- **Файл**: `tests\cell_registry_analysis_test.rs`
- **Суть**: Обновлён DummyCell под DigestivePipeline.

### `NEI-20260530-synapse-digest`
- **Файл**: `tests\synapse_hub_local_analysis_test.rs`
- **Суть**: CountCell обновлён для ParsedInput.

### `NEI-20260530-cancel-digest`
- **Файл**: `tests\synapse_hub_cancel_test.rs`
- **Суть**: CancelCell использует ParsedInput и умеет обрабатывать отмену.

### `NEI-20260530-brainloop-digest`
- **Файл**: `tests\brain_loop_test.rs`
- **Суть**: DummyCell обновлён для ParsedInput.

### `NEI-20260514-preflight-tests`
- **Файл**: `spinal_cord\tests\immune_preflight_check.rs`
- **Суть**: Проверяет валидацию StemCellRecord через preflight_check.

### `NEI-20260501-organ-status-events-test`
- **Файл**: `spinal_cord\tests\organ_builder_test.rs`
- **Суть**: проверяет отправку событий при смене статуса органа.

### `NEI-20260413-control-plane-rename`
- **Файл**: `spinal_cord\tests\control_plane_test.rs`
- **Суть**: Путь CONTROL_SNAPSHOT_DIR обновлён на spinal_cord/snapshots_test.

### `NEI-20260407-organ-builder-list-test`
- **Файл**: `spinal_cord\tests\organ_builder_test.rs`
- **Суть**: проверяет, что list возвращает все известные органы.

### `NEI-20260131-metacognition-tests`
- **Файл**: `spinal_cord\tests\metacognition_tests.rs`
- **Суть**: | Unit и интеграционные тесты для MetaCognition Engine.

### `NEI-20260131-consciousness-integration-test`
- **Файл**: `spinal_cord\tests\consciousness_integration_test.rs`
- **Суть**: | Интеграционный тест для Фазы 3 (Consciousness).

### `NEI-20260131-auto-improvement-tests`
- **Файл**: `spinal_cord\tests\auto_improvement_tests.rs`
- **Суть**: | Тесты для Auto-Improvement Loop.

### `NEI-20251220-organ-builder-cleanup-test`
- **Файл**: `spinal_cord\tests\organ_builder_test.rs`
- **Суть**: проверяет фоновую очистку просроченных шаблонов и статусов.

### `NEI-20251205-organ-rebuild-test`
- **Файл**: `spinal_cord\tests\organ_builder_test.rs`
- **Суть**: проверяет перезапуск сборки из сохранённого шаблона.

### `NEI-20251104-personality-evolution-tests`
- **Файл**: `spinal_cord\tests\personality_evolution_tests.rs`
- **Суть**: | Тесты для Personality Evolution Tracker.

### `NEI-20251103-semantic-memory-tests`
- **Файл**: `tests\semantic_memory_tests.rs`
- **Суть**: | Интеграционные и unit-тесты для SemanticMemory. Тестирование cosine similarity, remember/recall, векторного поиска.

### `NEI-20251103-dialogue-integration-tests`
- **Файл**: `tests\dialogue_integration_tests.rs`
- **Суть**: | Интеграционные тесты для EvolvingDialogue — полный цикл с памятью и ростом.

### `NEI-20251101-phonetics-fixtures-test`
- **Файл**: `tests\phonetics_fixtures_test.rs`
- **Суть**: Валидация структуры фикстур phonetics_samples.json (id, audio_path, transcription_phonetic, meta).

### `NEI-20251101-digestive-ingest-tests`
- **Файл**: `tests\digestive_pipeline_ingest_formats_test.rs`
- **Суть**: Тесты на JSON/YAML/XML ingest, XML $text-flattening, store_parsed_input и токсик-фильтр sanitize.

### `NEI-20251101-curriculum-seed-tests`
- **Файл**: `tests\curriculum_inquiry_seed_test.rs`
- **Суть**: Тесты: сортировка/лимит build_inquiry_seed и env-предел RUSSIAN_CURRICULUM_MAX_WORDS.

### `NEI-20251010-organ-builder-test`
- **Файл**: `spinal_cord\tests\organ_builder_test.rs`
- **Суть**: Проверяет переходы стадий органа, ручное обновление, удержание статуса `Failed`, очистку шаблонов по TTL и восстановление счётчика идентификаторов при рестарте.

### `NEI-20250501-reregister-same-path`
- **Файл**: `tests\action_cell_template_test.rs`
- **Суть**: Проверяет, что повторная регистрация по тому же пути заменяет шаблон.

### `NEI-20250501-different-type-error`
- **Файл**: `tests\action_cell_template_test.rs`
- **Суть**: Проверяет, что повторная регистрация с другим типом шаблона запрещена.

### `NEI-20250501-different-path-error`
- **Файл**: `tests\action_cell_template_test.rs`
- **Суть**: Проверяет, что повторная регистрация с другим путём возвращает ошибку.

### `NEI-20250323-151200-action-template-list`
- **Файл**: `tests\action_cell_template_test.rs`
- **Суть**: Проверяет регистрацию и перечисление шаблонов узлов действия.

### `NEI-20250317-test-recorder-counters`
- **Файл**: `spinal_cord\tests\common\mod.rs`
- **Суть**: shared recorder now captures counters in addition to histograms.

### `NEI-20250317-organ-builder-update-missing-test`
- **Файл**: `spinal_cord\tests\organ_builder_test.rs`
- **Суть**: records error metric when updating status for unknown organ.

### `NEI-20250317-chat-cell-metrics-recorder`
- **Файл**: `spinal_cord\tests\chat_cell_metrics_test.rs`
- **Суть**: reuse shared recorder to avoid resetting global metrics.

### `NEI-20250214-control-plane-startup-wait`
- **Файл**: `spinal_cord\tests\control_plane_test.rs`
- **Суть**: усилили ожидание запуска бэкенда и фиксируем ранний выход процесса.

### `NEI-20250210-working-memory-test-clippy`
- **Файл**: `tests\working_memory_test.rs`
- **Суть**: подавлен бессмысленный assert для clippy.

### `NEI-20250210-self-improve-test-clippy`
- **Файл**: `tests\self_improve_cell_test.rs`
- **Суть**: подавлен бессмысленный assert для clippy.

### `NEI-20250210-memory-cell-clippy`
- **Файл**: `tests\memory_cell_metrics_test.rs`
- **Суть**: заменён vec! на срез для устранения предупреждения clippy.

### `NEI-20250210-developer-interface-test-clippy`
- **Файл**: `tests\developer_interface_test.rs`
- **Суть**: подавлен бессмысленный assert для clippy.

### `NEI-20250210-core-personality-test-clippy`
- **Файл**: `tests\core_personality_test.rs`
- **Суть**: подавлен бессмысленный assert для clippy.

### `NEI-20250210-controlled-training-clippy`
- **Файл**: `tests\controlled_training_environment_test.rs`
- **Суть**: подавлен бессмысленный assert для clippy.

### `NEI-20250210-code-cell-test-clippy`
- **Файл**: `tests\code_cell_test.rs`
- **Суть**: подавлен lint о бессмысленном assert при запуске clippy.

### `NEI-20250210-chat-cell-test-clippy`
- **Файл**: `tests\chat_cell_test.rs`
- **Суть**: подавлен бессмысленный assert для clippy.

### `NEI-20250101-000005-context-dir-test`
- **Файл**: `tests\context_dir_test.rs`
- **Суть**: Проверяет дефолтный путь и переопределение CONTEXT_DIR.

### `NEI-20241002-brain-flow-test`
- **Файл**: `spinal_cord\tests\brain_flow_test.rs`
- **Суть**: Проверяет, что Brain обрабатывает FlowMessage::Task и FlowMessage::Event.

### `NEI-20240930-brain-subscriber-test`
- **Файл**: `spinal_cord\tests\brain_subscriber_test.rs`
- **Суть**: Проверяет, что BrainSubscriber пересылает события в DataFlowController.

### `NEI-20240821-brain-metrics-test`
- **Файл**: `tests\brain_loop_test.rs`
- **Суть**: Brain отправляет запись в MetricsCollectorCell при обработке события.

### `NEI-20240810-manual-analysis-test`
- **Файл**: `tests\brain_loop_test.rs`
- **Суть**: Извлекает задачу из планировщика и вручную запускает анализ.

### `NEI-20240728-brain-loop-event-test`
- **Файл**: `tests\brain_loop_test.rs`
- **Суть**: Подключённый планировщик и шина не образуют циклов при обработке событий.

### `NEI-20240725-brain-loop-test`
- **Файл**: `tests\brain_loop_test.rs`
- **Суть**: Имитация боевой схемы: планировщик и шина используют общий DataFlowController; проверяет отсутствие циклов.

### `NEI-20240607-io-watcher-test-update`
- **Файл**: `spinal_cord\tests\io_watcher_test.rs`
- **Суть**: Обновлены вызовы IoWatcher::new с токеном остановки.

### `NEI-20240607-cell-registration-test`
- **Файл**: `spinal_cord\tests\cell_registration.rs`
- **Суть**: \ Проверяет, что при создании записи клетки нервная и иммунная системы получают уведомления.

### `NEI-20240514-brain-subscriber-test-flowevent`
- **Файл**: `spinal_cord\tests\brain_subscriber_test.rs`
- **Суть**: Проверяет имя в типизированном FlowEvent.

### `NEI-20240514-brain-loop-test-typed`
- **Файл**: `tests\brain_loop_test.rs`
- **Суть**: Использует FlowEvent и TaskPayload в сообщениях.

### `NEI-20240514-brain-flow-test-typed`
- **Файл**: `spinal_cord\tests\brain_flow_test.rs`
- **Суть**: Использует FlowEvent и TaskPayload.

## TESTING

### `NEI-TEST2-LEARNING-KICKOFF`
- **Файл**: `docs\archive\AUTONOMOUS_TASK_2_RESULTS.md`
- **Суть**: | Запущен Тест #2 — проверка способности Нейры к обучению и самосовершенствованию. Отправлено 2 задания через Consciousness API, начинается фаза мониторинга.

### `NEI-TEST2-LEARNING-FINAL`
- **Файл**: `docs\archive\AUTONOMOUS_TASK_2_FINAL.md`
- **Суть**: | Завершён Тест #2 — проверка способности Нейры к обучению.
