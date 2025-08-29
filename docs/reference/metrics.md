# Реестр Метрик (Истина)

| Имя | Тип | Единицы | Где инкрементируется | Назначение |
|---|---|---|---|---|
| chat_requests_total | counter | req | InteractionHub | Входящие чат‑запросы |
| chat_errors_total | counter | err | InteractionHub | Ошибки авторизации/валидации/лимитов |
| chat_response_time_ms | histogram | ms | InteractionHub | Время ответа чат‑узла |
| analysis_requests_total | counter | req | InteractionHub | Входящие анализ‑запросы |
| analysis_errors_total | counter | err | InteractionHub | Ошибки анализа/тайм‑ауты/отмена |
| analysis_node_request_duration_ms | histogram | ms | InteractionHub | Длительность анализа (сред/квантили) |
| chat_node_requests_total | counter | req | EchoChatNode | Вызовы чат‑ноды |
| chat_node_errors_total | counter | err | EchoChatNode | Ошибки чат‑ноды |
| chat_node_request_duration_ms | histogram | ms | EchoChatNode | Длительность обработки узлом |
| messages_saved | counter | msg | FileContextStorage | Сохранённые сообщения |
| context_loads | counter | op | FileContextStorage | Загрузки контекста |
| context_misses | counter | op | FileContextStorage | Промахи загрузки |
| context_bytes_written | counter | bytes | FileContextStorage | Записанные байты контекста |
| gz_rotate_count | counter | ops | FileContextStorage | Архивные ротации gz |
| sessions_created_total | counter | ops | Hub/Session | Созданные сессии |
| sessions_deleted_total | counter | ops | Session delete | Удалённые сессии |
| sessions_closed_total | counter | ops | Session delete | Закрытия сессий (для отчётов) |
| sessions_active | gauge | count | Hub/Session init+ops | Активные сессии |
| sessions_autocreated_total | counter | ops | Hub (persist auto) | Автосозданные сессии |
| requests_idempotent_hits | counter | ops | Hub (LRU+file) | Кэш‑попадания идемпотентных ответов |
| index_compact_runs | counter | ops | Compaction job | Запуски компактера |
| sse_active | gauge | count | SSE stream | Активные SSE потоки |
| safe_mode | gauge | 0/1 | Hub | Статус безопасного режима |

Примечание: именование согласовано с кодом (backend/src). При добавлении новых метрик — обновляйте эту таблицу.

