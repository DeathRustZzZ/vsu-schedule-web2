# Руководство разработчика

## Требования
- **Java 17** (для Spring Boot сервисов).
- **Gradle** (для `schedule-service`, если запуск без Docker; в `api-gateway` и `eurekaserver` есть Gradle Wrapper).
- **Docker + Docker Compose** (для локального окружения с базами).
- **Rust** (для `telegram-service-rust`, если запуск без Docker).
- **PostgreSQL** (если запуск без Docker).

## Переменные окружения
### Telegram-бот
- `BOT_TOKEN` — токен Telegram-бота.
- `DATABASE_URL` — строка подключения к `students-db`, например: `postgresql://admin:admin@localhost:5434/students-db`.
- `RUST_LOG` — уровень логирования, например `info`.

Локально переменные удобно хранить в `.env` (файл уже используется кодом через `dotenvy`).

## Запуск через Docker (рекомендуется)
### 1) Все сервисы + базы
В корне проекта:
```bash
docker compose -f compose-env.yaml up --build
```
Это поднимет:
- `eurekaserver`
- `api-gateway`
- `db` (PostgreSQL, `schedule-db`, порт `5432`)
- `students-db` (PostgreSQL, `students-db`, порт `5434`)
- `schedule-service`
- `telegram-service-rust`

## Локальный запуск без Docker
### PostgreSQL
Нужно поднять две базы:
- `schedule-db` на `localhost:5432`
- `students-db` на `localhost:5434`

### schedule-service
1) Укажи параметры подключения в `schedule-service/src/main/resources/application.yaml`.
2) Запуск (если есть установленный Gradle):
```bash
(cd schedule-service && gradle bootRun)
```

### eurekaserver
```bash
(cd eurekaserver && ./gradlew bootRun)
```

### api-gateway
```bash
(cd api-gateway && ./gradlew bootRun)
```

### telegram-service-rust
1) Подготовь `.env` с `BOT_TOKEN`, `DATABASE_URL`, `RUST_LOG`.
2) Запуск:
```bash
(cd telegram-service-rust && cargo run)
```

## Миграции и схема данных
### schedule-service
Flyway миграции находятся в:
- `schedule-service/src/main/resources/db/migration`

### telegram-service-rust
SQL-миграции:
- `telegram-service-rust/migrations`

При запуске через Docker также используются init-скрипты:
- `init-scripts-students/*.sql`

## Маршруты API Gateway
Файл маршрутов:
- `api-gateway/src/main/resources/routes/services-routes.yaml`

Основные маршруты (примеры):
- `/api/v1/groups/**` -> `schedule-service`
- `/api/v1/teachers/**` -> `schedule-service`
- `/api/v1/lessons/**` -> `schedule-service`
- `/api/v1/schedule/uploadFile` -> `schedule-service`

## Отладка и полезные заметки
- Сервисы регистрируются в Eureka; при запуске убедись, что `eurekaserver` поднят первым.
- В `api-gateway` настроен маршрут `/bots/index.php` на `telegram-service`, но текущая реализация бота работает через polling и не поднимает HTTP-сервер.
- Порты по умолчанию: `8761` (Eureka), `8765` (Gateway), `9898` (schedule-service), `5432` и `5434` (PostgreSQL).
