# schedule-service-rust

Rust сервис для расписания бота с выбором даты. Это первый шаг миграции backend на Rust по модели strangler: новые эндпоинты добавляются в отдельный сервис и подключаются через gateway, старые Java эндпоинты остаются рабочими.

## Архитектура

Слои:
- `http`: обработчики HTTP и валидация запроса.
- `service`: бизнес‑логика выборки расписания.
- `db`: подключение к PostgreSQL.
- `model`: DTO ответа.
- `util`: утилиты (например, парсинг даты).

Данные берутся из существующей базы `schedule-db` (таблицы `lessons`, `teachers`).

## Эндпоинт

`GET /api/v1/bot/v2/schedule`

Query параметры:
- `faculty` — факультет (например, `ФМиИТ`)
- `group` — id группы
- `subgroup` — id подгруппы (может быть пустым)
- `date` — дата запроса

Форматы даты принимаются:
- `YYYY-MM-DD`
- `DD.MM.YYYY`
- `DD/MM/YY`
- и другие варианты с `.` или `/`

Сервис нормализует дату и подбирает набор вариантов для совпадения со строкой даты в БД.

Пример:
```
GET /api/v1/bot/v2/schedule?faculty=ФМиИТ&group=23ПИ1&subgroup=23ПИ1-1&date=2026-02-12
```

`GET /api/v1/bot/v2/schedule/week`

Query параметры:
- `faculty` — факультет
- `group` — id группы
- `subgroup` — id подгруппы
- `start` — дата начала (обязательная)
- `end` — дата конца (необязательная). Если не указано — берётся диапазон `MAX_RANGE_DAYS` дней.

Пример:
```
GET /api/v1/bot/v2/schedule/week?faculty=ФМиИТ&group=23ПИ1&subgroup=23ПИ1-1&start=2026-02-10&end=2026-02-16
```

## Запуск в Docker Compose

В корне репозитория уже есть `compose-env.yaml` с подключенным сервисом.

### 1) Собрать и запустить всё
```
docker compose -f compose-env.yaml up --build
```

### 2) Проверка здоровья
```
curl http://localhost:9900/health
```

### 3) Проверка запроса
```
curl "http://localhost:8765/api/v1/bot/v2/schedule?faculty=ФМиИТ&group=23ПИ1&subgroup=23ПИ1-1&date=2026-02-12"
```

```
curl "http://localhost:8765/api/v1/bot/v2/schedule/week?faculty=ФМиИТ&group=23ПИ1&subgroup=23ПИ1-1&start=2026-02-10&end=2026-02-16"
```

## Переменные окружения

- `DATABASE_URL` — строка подключения к Postgres
- `BIND_ADDR` — адрес и порт, по умолчанию `0.0.0.0:9900`
- `MAX_DATE_VARIANTS` — лимит вариантов даты (по умолчанию `16`)
- `MAX_RANGE_DAYS` — максимальная длина диапазона (по умолчанию `7`)

## Дальнейшие шаги

- Подключить Telegram‑бот к `v2` эндпоинту и добавить UX выбора даты.
- Добавить эндпоинт диапазона (неделя) после стабилизации.
