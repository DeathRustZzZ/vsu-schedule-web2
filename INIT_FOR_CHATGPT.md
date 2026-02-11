# VSU Schedule Web — ChatGPT Init

## Коротко
Микросервисная система для расписания ВГУ. Основные сервисы на Spring Boot + API Gateway + Eureka, отдельный Telegram-бот на Rust (teloxide). Две PostgreSQL базы: `schedule-db` и `students-db`. Основная бизнес-логика в `schedule-service`.

## Состав и порты (локально)
- `eurekaserver` (Eureka): `8761`
- `api-gateway` (Spring Cloud Gateway): `8765`
- `schedule-service` (Spring Boot): `9898`
- PostgreSQL `schedule-db`: `5432`
- PostgreSQL `students-db`: `5434`

## Архитектура (упрощённо)
Клиент -> API Gateway -> schedule-service -> schedule-db
Telegram API -> telegram-service-rust -> students-db
Eureka используется для service discovery.

## Структура репозитория
- `api-gateway/` — маршрутизация запросов (Spring Cloud Gateway)
- `eurekaserver/` — Eureka Server
- `schedule-service/` — основной сервис расписаний
- `telegram-service-rust/` — Telegram-бот на Rust
- `init-scripts-db/` — init-скрипты для `schedule-db`
- `init-scripts-students/` — init-скрипты для `students-db`
- `compose-env.yaml` — Docker Compose окружение

## Основные API (schedule-service)
- `/api/v1/groups/**`
- `/api/v1/teachers/**`
- `/api/v1/lessons/**`
- `/api/v1/schedule/uploadFile` — загрузка расписания
- `/api/v1/groups/available/{faculty}` — доступные группы
- `/api/v1/bot/schedule` — расписание для бота
- Админка: `/schedule/admin`

## Миграции и БД
- `schedule-service/src/main/resources/db/migration` (Flyway)
- `telegram-service-rust/migrations` и `init-scripts-students/*.sql`

## Запуск (Docker)
```bash
docker compose -f compose-env.yaml up --build
```
Для отдельных сервисов:
```bash
docker compose -f compose-env.yaml up -d --build eurekaserver api-gateway schedule-service db students-db redis
```

## Локальный запуск без Docker
- Java 17 + Gradle (или wrapper) для Java-сервисов
- Rust toolchain (cargo) для `telegram-service-rust`

Примеры:
```bash
cd schedule-service
./gradlew bootRun

cd telegram-service-rust
cargo run
```

## Конфигурация
- `schedule-service/src/main/resources/application.yaml`
- `api-gateway/src/main/resources/application.yaml`
- `api-gateway/src/main/resources/routes/services-routes.yaml`
- `.env` для Telegram-бота (локально)

## Переменные окружения Telegram-бота
- `BOT_TOKEN`
- `DATABASE_URL`
- `RUST_LOG`
- `SCHEDULE_API_BASE` (по умолчанию `http://api-gateway:8765`)
- `SCHEDULE_TZ_OFFSET` (например `+03:00`)

## Где искать логику
- Парсер расписаний: `schedule-service/src/main/java/com/vsuscheduleweb/parser/Parser.java`
- UI админки: `schedule-service/src/main/resources/templates/` + `static/schedule/js/`
- Бот: `telegram-service-rust/src/bot`

## Важно
Telegram-бот работает в режиме polling и не поднимает HTTP-сервер. В gateway может быть настроен маршрут `/bots/index.php`, но в текущем коде бот не слушает HTTP.
