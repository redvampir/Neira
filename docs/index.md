# Документация Neira — Стартовая страница

Добро пожаловать. Эта страница — карта проекта (DocMap) для быстрой навигации как для человека (владелец), так и для ассистента.

- Быстрый обзор API: docs/backend-api.md
- Переменные окружения (референс): docs/reference/env.md
- Метрики (реестр): docs/reference/metrics.md
- Способности и гейты: CAPABILITIES.md
- Режимы автономии и принципы: AGENTS.md (разделы Vision & Autonomy, Autonomy Modes)
- Рабочий процесс и мета‑комментарии: WORKFLOW.md, COMMENTING.md
- Роли и взаимодействие: TEAMWORK.md
- Код‑гайдлайны: CODING_GUIDELINES.md
- Решения (ADR‑индекс): DECISIONS.md
- Примеры запросов: docs/examples/curl.md
- Глоссарий терминов: docs/meta/glossary.md
- Справка по окружению бэкенда (историческая): backend/ENV.md

## DocMap — откуда истина
- API: docs/backend-api.md (истина), README — только ссылка.
- ENV: docs/reference/env.md (истина), backend/ENV.md — обзор и пример .env.
- Метрики: docs/reference/metrics.md (истина), код — источник конкретных инкрементов.
- Гейты способностей: CAPABILITIES.md (истина), ссылки из neira:meta.

## Быстрый старт
1) Посмотрите API и примеры curl.
2) Настройте .env (см. docs/reference/env.md и backend/ENV.md).
3) Запустите сервис и проверьте /metrics.

## Термины (кратко)
- Иммунная система: безопасный режим, карантин, интегрити.
- Нервная система: метрики/пробы/алерты.
- Гейт способности: переключатель locked/experimental/stable/deprecated.
- Режимы автономии: explore / perform / safe‑mode.
