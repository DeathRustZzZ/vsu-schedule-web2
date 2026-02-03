# Developer Guide — VSU Schedule Web

Полный ориентир по проекту: архитектура, запуск, окружение, данные, отладка.

---

## 1. Коротко о проекте

VSU Schedule Web — микросервисная система для работы с расписанием ВГУ.
Состоит из:
- **Eureka Server** — сервис обнаружения.
- **API Gateway** — маршрутизация запросов.
- **Schedule Service** — основной сервис расписаний (Spring Boot).
- **Telegram Bot** — бот на Rust (teloxide).

Схема (упрощённо):
```
Клиент
  -> API Gateway (8765)
       -> Schedule Service (9898) -> PostgreSQL (schedule-db)

Telegram API
  -> telegram-service-rust -> PostgreSQL (students-db)
```

---

## 2. Требования

### Рекомендуемый способ (Docker)
- Docker + Docker Compose

### Для локального запуска без Docker
- Java 17
- Gradle (или Gradle Wrapper для api-gateway/eurekaserver)
- Rust toolchain (cargo)
- PostgreSQL (2 базы)

---

## 3. Структура репозитория

```
/ (root)
  api-gateway/           # Spring Cloud Gateway
  eurekaserver/          # Eureka Server
  schedule-service/      # Основной сервис расписаний
  telegram-service-rust/ # Telegram-бот (Rust)
  init-scripts-db/       # init скрипты schedule-db
  init-scripts-students/ # init скрипты students-db
  compose-env.yaml       # docker-compose окружение
  ARCHITECTURE.md
  README.md
```

---

## 4. Запуск проекта

### 4.1. Быстрый старт (Docker, все сервисы)
```bash
docker compose -f compose-env.yaml up --build
```

### 4.2. Запуск отдельных сервисов
```bash
docker compose -f compose-env.yaml up -d --build schedule-service api-gateway eurekaserver db students-db redis
```

---

## 5. Порты

- Eureka Server: http://localhost:8761
- API Gateway: http://localhost:8765
- Schedule Service: http://localhost:9898
- PostgreSQL schedule-db: localhost:5432
- PostgreSQL students-db: localhost:5434

---

## 6. Базы данных и миграции

### schedule-service
- Flyway миграции: `schedule-service/src/main/resources/db/migration`

### telegram-service-rust
- Миграции: `telegram-service-rust/migrations`
- В docker используются `init-scripts-students/*.sql`

---

## 7. Основные REST API (Schedule Service)

- `/api/v1/groups/**`
- `/api/v1/teachers/**`
- `/api/v1/lessons/**`
- `/api/v1/schedule/uploadFile` — загрузка расписаний
- `/api/v1/groups/available/{faculty}` — список групп + подгрупп
- `/api/v1/bot/schedule` — расписание для бота

---

## 8. Админ-панель

- URL: `/schedule/admin`
- UI шаблоны: `schedule-service/src/main/resources/templates/`
- JS логика загрузки расписаний: `schedule-service/src/main/resources/static/schedule/js/`

---

## 9. Telegram Bot

### Переменные окружения
- `BOT_TOKEN`
- `DATABASE_URL`
- `RUST_LOG`
- `SCHEDULE_API_BASE` (по умолчанию `http://api-gateway:8765`)
- `SCHEDULE_TZ_OFFSET` (например `+03:00`)

### Запуск (локально)
```bash
cd telegram-service-rust
cargo run
```

### Основные модули
- `src/bot` — UX, маршрутизация callback
- `src/db` — работа с БД
- `src/domain` — доменные модели

---

## 10. Парсер расписаний

Парсер находится в:
- `schedule-service/src/main/java/com/vsuscheduleweb/parser/Parser.java`

Поддерживает:
- обычное расписание (дни недели + пары)
- зачёты/экзамены (по колонке "Дата")

---

## 11. Как загрузить расписание

1. Открыть `/schedule/admin`
2. Выбрать факультет
3. Загрузить Excel файл

В случае успеха данные сохраняются в `schedule-db`, кэш в Redis обновляется.

---

## 12. Частые проблемы

### ❌ "Не видно групп" в Telegram
Проверь:
- корректно ли загружены расписания в schedule-service
- работает ли `/api/v1/groups/available/{faculty}`
- корректная нормализация факультета

### ❌ Ошибка загрузки расписания
Чаще всего:
- неверный формат Excel
- таблица не соответствует ожидаемой структуре

---

## 13. Разработка

### Полезные команды

#### Локально (Java)
```bash
cd schedule-service
./gradlew bootRun
```

#### Локально (Rust)
```bash
cd telegram-service-rust
cargo run
```

---

## 14. Рекомендуемый workflow

1. Разработка изменений в сервисе
2. Локальная проверка
3. Пересборка контейнера нужного сервиса
4. Smoke-тест через API

---

## 15. Где искать

- Архитектура: `ARCHITECTURE.md`
- Быстрый старт: `README.md`
- Конфиги: `compose-env.yaml`

---

Если нужно добавить новые сервисы или расширять API — лучше фиксировать изменения в этом гайде.
