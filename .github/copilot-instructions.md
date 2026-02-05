# Copilot / AI agent instructions — VSU Schedule Web

Коротко, практично и ориентировано на этот репозиторий. Цель — дать AI-кодинг-агенту всё, что нужно, чтобы быстро править код и тестировать изменения.

1) Большая картина
- Моно-репо с микросервисами: `api-gateway`, `eurekaserver`, `schedule-service`, `telegram-service-rust`.
- Сервис обнаружения: `eurekaserver` (порт 8761). Gateway: `api-gateway` (8765). Основной API: `schedule-service` (9898).
- Telegram-бот написан на Rust и работает polling'ом: `telegram-service-rust` (код в `src/bot`, БД — `migrations` и `init-scripts-students`).

2) Куда смотреть за архитектурой и правилами
- Архитектура: ARCHITECTURE.md
- Быстрый старт и workflow: DEVELOPER_GUIDE.md
- Основной README: README.md

3) Билд / запуск / тесты (конкретные команды)
- Запустить весь стек (рекомендуемый):
  - `docker compose -f compose-env.yaml up --build`
- Запуск отдельных Java-сервисов (пример `schedule-service`):
  - `cd schedule-service && ./gradlew bootRun`
  - Тесты: `./gradlew test`
- Rust-бот:
  - `cd telegram-service-rust && cargo build` — собрать
  - `cd telegram-service-rust && cargo run` — запустить локально (нужны env: BOT_TOKEN, DATABASE_URL и т.п.)

4) Важные проектные конвенции
- Каждый Java сервис использует Gradle Wrapper в своей папке; не запускайте глобальный Gradle.
- Конфиги находятся в `src/main/resources/application.yaml` внутри каждого сервиса.
- Миграции:
  - `schedule-service` — Flyway в `schedule-service/src/main/resources/db/migration`
  - `telegram-service-rust` — SQL-миграции в `telegram-service-rust/migrations` и `init-scripts-students` для Docker
- Telegram-бот хранит состояние пользователей в отдельной БД `students-db` (порт 5434 в compose).

5) Интеграционные точки и паттерны коммуникации
- Gateway маршруты: `api-gateway/src/main/resources/routes` — изменения маршрутов надо синхронизировать с `schedule-service` API.
- `telegram-service-rust` запрашивает расписание через API Gateway (переменная `SCHEDULE_API_BASE` / `api-gateway:8765`). Посмотрите `telegram-service-rust/src/bot/schedule_api.rs`.
- Сервис регистрации/состояний: `telegram-service-rust/src/db` (facade/repo) — там ключевые CRUD-операции для бота.

6) Что важно при изменениях
- Для правок в Java-сервисах: сначала локально запустите `./gradlew bootRun` и выполните API-ручные проверки или unit-тесты.
- Для правок в Rust-боте: запускайте `cargo run` и проверяйте через Telegram (используйте тестовый токен). Локальные тесты: `cargo test`.
- При изменении схемы БД обновите миграции и `init-scripts-*` (Docker использует init-скрипты при поднятии контейнера).

7) Примеры быстрого поиска и референсы (частые места правок)
- Роуты gateway: `api-gateway/src/main/resources/routes/services-routes.yaml`
- REST API расписаний: `schedule-service/src/main/java/.../controller` (искать `api/v1`)
- Telegram UX/колбэки: `telegram-service-rust/src/bot/callbacks.rs` и `router.rs`
- DB-фасад бота: `telegram-service-rust/src/db/facade.rs`

8) Ограничения и заметки для агента
- Не меняйте глобальные сборки/CI без явного указания; фокус — отдельный сервис.
- Проверяйте `compose-env.yaml` при изменении сетевых портов/имён сервисов.
- Логирование Rust использует `RUST_LOG` — для отладки увеличьте уровень.

9) Запросы к человеку (если нужно)
- Если отсутствуют env vars (BOT_TOKEN, DATABASE_URL) или доступ к тестовой Telegram-учётной записи — запросите у автора.
- Перед массовыми рефакторами предупредите про миграции БД и возможный откат.

Если нужно, могу расширить инструкцию примерами кода (частые паттерны в `src/bot`), или добавить CI/PR рекомендации.
