# Telegram сервис (Rust)

Telegram-бот для работы с расписанием. Реализован на Rust с использованием `teloxide` и хранит данные в PostgreSQL.

## Ключевые возможности
- Регистрация пользователя через меню бота.
- Хранение информации о студенте и состоянии регистрации.
- Обработка сообщений и callback-кнопок через dispatcher.

## Стек
- Rust 2024
- `teloxide` (Telegram Bot API)
- `sqlx` (PostgreSQL)
- `tokio`

## Переменные окружения
- `BOT_TOKEN` — токен Telegram-бота.
- `DATABASE_URL` — строка подключения к PostgreSQL.
- `RUST_LOG` — уровень логирования.

Пример (локально):
```bash
BOT_TOKEN=***
DATABASE_URL=postgresql://admin:admin@localhost:5434/students-db
RUST_LOG=info
```

## Запуск
### Через Docker
Сервис собирается и запускается из корня проекта:
```bash
docker compose -f ../compose-env.yaml up --build
```

### Локально
```bash
cargo run
```

## База данных
- База: `students-db`
- Таблицы: `students`, `user_states`, `schedule_lessons`
- Миграции: `migrations/*.sql`

## Полезные файлы
- `src/main.rs` — точка входа.
- `src/config.rs` — загрузка конфигурации.
- `src/services/telegram` — обработчики сообщений/меню.
- `src/db` — доступ к БД.
