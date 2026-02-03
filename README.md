# VSU Schedule Web

Проект расписания ВГУ на базе микросервисной архитектуры. Состоит из сервисов на Spring Boot и Telegram-бота на Rust.

## Состав проекта
- **eurekaserver** — сервис обнаружения (Eureka), порт `8761`.
- **api-gateway** — API Gateway (Spring Cloud Gateway), порт `8765`.
- **schedule-service** — основной сервис расписаний, порт `9898`, PostgreSQL `schedule-db`.
- **telegram-service-rust** — Telegram-бот (teloxide), PostgreSQL `students-db`.

## Быстрый старт (локально через Docker)
1) Запусти базы и сервисы, которые описаны в `compose-env.yaml`:
   ```bash
   docker compose -f compose-env.yaml up --build
   
   docker compose -f compose-env.yaml up -d --build eurekaserver api-gateway schedule-service db students-db redis
   ```
   Это поднимет:
   - `eurekaserver`
   - `api-gateway`
   - `db` (PostgreSQL, `schedule-db`, порт `5432`)
   - `students-db` (PostgreSQL, `students-db`, порт `5434`)
   - `schedule-service`
   - `telegram-service-rust`

## Полезные порты
- Eureka: `http://localhost:8761`
- API Gateway: `http://localhost:8765`
- Schedule Service: `http://localhost:9898`
- PostgreSQL `schedule-db`: `localhost:5432`
- PostgreSQL `students-db`: `localhost:5434`

## Конфигурация
- `compose-env.yaml` — docker-compose окружение.
- `schedule-service/src/main/resources/application.yaml` — настройки schedule-service.
- `api-gateway/src/main/resources/application.yaml` и `routes/services-routes.yaml` — маршруты gateway.
- `.env` — переменные Telegram-бота (локально).

Подробности для разработки смотри в `DEVELOPER_GUIDE.md`.
