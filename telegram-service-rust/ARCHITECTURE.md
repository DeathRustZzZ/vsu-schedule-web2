# Архитектура Telegram-сервиса

## Поток обработки
```
Telegram API
    |
    v
Dispatcher (teloxide)
    |-- messages -> handlers::handle_message
    |-- callbacks -> handlers::handle_callback -> callback_router
    v
DbFacade -> students_repo / user_states_repo
    v
PostgreSQL (students-db)
```

## Основные модули
- `src/main.rs` — точка входа, инициализация конфигурации и пула БД.
- `src/config.rs` — чтение `BOT_TOKEN`, `DATABASE_URL`, `RUST_LOG`.
- `src/app/facade.rs` — склейка конфигурации, БД и запуска бота.
- `src/services/telegram` — обработчики сообщений, меню, маршрутизация callback.
- `src/db` — слой доступа к БД (пул + репозитории + фасад).
- `src/domain` — доменные модели, меню, callback-данные, состояния.

## Доступ к данным
- `DbFacade` скрывает работу с репозиториями.
- `students_repo` — CRUD по таблице `students`.
- `user_states_repo` — управление состояниями регистрации.

## База данных
- `students` — зарегистрированные пользователи.
- `user_states` — текущее состояние регистрации пользователя.

Схема БД задаётся SQL-миграциями в `migrations/*.sql`.
