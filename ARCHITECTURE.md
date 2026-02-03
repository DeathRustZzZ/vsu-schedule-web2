# Архитектура проекта

## Контекст
Проект состоит из набора сервисов, связанных через Eureka (service discovery) и API Gateway. Основные бизнес-операции выполняет `schedule-service`, Telegram-бот (`telegram-service-rust`) обслуживает пользователей в Telegram и хранит состояния/данные в отдельной базе.

## Логическая схема
```
Пользователь/Клиент
        |
        v
  API Gateway (8765) ----> Eureka (8761)
        |
        v
 schedule-service (9898) ----> PostgreSQL schedule-db (5432)

Telegram API <--> telegram-service-rust ----> PostgreSQL students-db (5434)
```

## Сервисы
### api-gateway
- Spring Cloud Gateway.
- Читает маршруты из `api-gateway/src/main/resources/routes/services-routes.yaml`.
- Маршрутизирует запросы к `schedule-service` и (по конфигурации) к `telegram-service`.

### eurekaserver
- Spring Cloud Netflix Eureka Server.
- Порт `8761`.

### schedule-service
- Spring Boot 3.0.5.
- Основные REST-эндпоинты:
  - `/api/v1/groups/**`
  - `/api/v1/teachers/**`
  - `/api/v1/lessons/**`
  - `/api/v1/schedule/uploadFile` (загрузка расписания)
- Web-админка: `/schedule/admin`.
- БД: PostgreSQL `schedule-db`.
- Миграции: `schedule-service/src/main/resources/db/migration` (Flyway).

### telegram-service-rust
- Rust + `teloxide`.
- Запускается как Telegram-бот и обрабатывает сообщения/колбэки через dispatcher.
- Хранит пользователей и состояния регистрации в PostgreSQL `students-db`.
- Схема данных задаётся скриптами в `telegram-service-rust/migrations` и `init-scripts-students`.

## Данные и базы
- `schedule-db` — таблицы расписания, преподавателей, групп.
- `students-db` — таблицы `students`, `user_states`.

## Сетевые настройки (локально)
- `schedule-service` публикуется на `9898`.
- `api-gateway` публикуется на `8765`.
- `eurekaserver` публикуется на `8761`.
- PostgreSQL: `schedule-db` на `5432`, `students-db` на `5434`.

Примечание: в gateway настроен маршрут `/bots/index.php` на `telegram-service`, но в текущем коде Telegram-бот работает в режиме polling и не поднимает HTTP-сервер.
