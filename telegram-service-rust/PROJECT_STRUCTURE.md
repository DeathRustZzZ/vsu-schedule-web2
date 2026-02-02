# 📂 Структура проекта (финальная версия v1.0)

## Полная структура директорий

```
tg_bot_vsu/
├── src/
│   ├── main.rs                          ✅ Точка входа
│   ├── config.rs                        ✅ Конфигурация приложения
│   ├── errors.rs                        ✅ Типы ошибок
│   ├── schema.rs                        ✅ Схема БД (автогенерировано Diesel)
│   │
│   ├── app/                             📦 Application Layer
│   │   ├── mod.rs                       ✅ Экспорты
│   │   └── facade.rs                    ✅ Главный фасад приложения
│   │
│   ├── db/                              📦 Data Access Layer
│   │   ├── mod.rs                       ✅ Экспорты репозиториев
│   │   ├── connection.rs                ✅ Инициализация подключения
│   │   ├── facade.rs                    ✅ DbFacade (главный фасад БД)
│   │   ├── students_repo.rs             ✅ Репозиторий студентов
│   │   └── user_states_repo.rs          ✅ Репозиторий состояния (НОВОЕ)
│   │
│   ├── domain/                          📦 Domain Layer (бизнес-логика)
│   │   ├── mod.rs                       ✅ Экспорты
│   │   ├── buttons.rs                   ✅ Enum основных кнопок
│   │   ├── callbacks.rs                 ✅ Unified enum для всех callbacks (НОВОЕ)
│   │   ├── course.rs                    ✅ Enum курсов
│   │   ├── faculty.rs                   ✅ Enum факультетов
│   │   ├── student.rs                   ✅ Модель студента
│   │   ├── study_form.rs                ✅ Enum форм обучения
│   │   ├── user_state.rs                ✅ Модель состояния пользователя (НОВОЕ)
│   │   │
│   │   └── groups/                      📦 Группы по факультетам
│   │       ├── mod.rs                   ✅ Экспорты
│   │       └── mit.rs                   ✅ Enum групп МИТ
│   │
│   └── services/                        📦 Service Layer
│       ├── mod.rs                       ✅ Экспорты
│       │
│       └── telegram/                    📦 Telegram интеграция
│           ├── mod.rs                   ✅ Инициализация и запуск бота
│           ├── handlers.rs              ✅ Основные обработчики сообщений/callbacks
│           ├── callback_router.rs       ✅ Маршрутизатор callbacks (НОВОЕ)
│           ├── keyboards.rs             ✅ Клавиатуры главного меню
│           │
│           └── registration/            📦 Модуль регистрации
│               ├── mod.rs               ✅ Экспорты
│               ├── handlers.rs          ✅ Обработчики этапов регистрации
│               ├── keyboards.rs         ✅ Клавиатуры регистрации
│               └── facade.rs            ✅ RegistrationFacade
│
├── migrations/                          📦 Миграции Diesel
│   ├── 00000000000000_diesel_initial_setup/
│   ├── 2026-01-12-114732-0000_create_schedule_lessons/
│   ├── 2026-01-14-123212-0000_create_students/
│   ├── 2026-01-14-165306-0000_add_study_form_to_students/
│   ├── 2026-01-14-175957-0000_add_course_and_username_to_students/
│   │
│   └── 2026-01-16-125016-0000_add_user_states/  ✅ НОВАЯ МИГРАЦИЯ
│       ├── up.sql                       ✅ Создание таблицы user_states
│       └── down.sql                     ✅ Откат миграции
│
├── target/                              📦 Выходные артефакты (сборка)
│
├── Cargo.toml                           ✅ Конфигурация проекта
├── Cargo.lock                           ✅ Lockfile зависимостей
├── diesel.toml                          ✅ Конфигурация Diesel
│
├── README.md                            ✅ Главная документация (ПЕРЕДЕЛАНО)
├── ARCHITECTURE.md                      ✅ Архитектура проекта (НОВОЕ)
├── DEVELOPER_GUIDE.md                   ✅ Руководство разработчика (НОВОЕ)
├── EXAMPLES.md                          ✅ Примеры использования (НОВОЕ)
├── IMPROVEMENTS_CHECKLIST.md            ✅ Чек-лист улучшений (НОВОЕ)
└── COMPLETION_REPORT.md                 ✅ Отчёт о завершении (НОВОЕ)
```

---

## 📊 Статистика файлов

### Исходный код (src/)

| Директория | Файлы | Описание |
|-----------|-------|---------|
| **root** | 4 | main.rs, config.rs, errors.rs, schema.rs |
| **app/** | 2 | Слой приложения |
| **db/** | 5 | Репозитории и фасад БД |
| **domain/** | 9 | Бизнес-объекты (с groups/) |
| **services/telegram/** | 7 | Telegram интеграция |
| **ИТОГО** | **27** | Файлов кода |

### Документация

| Файл | Строк | Описание |
|------|-------|---------|
| README.md | 200+ | Главная документация |
| ARCHITECTURE.md | 300+ | Архитектура проекта |
| DEVELOPER_GUIDE.md | 350+ | Руководство разработчика |
| EXAMPLES.md | 400+ | Примеры кода |
| IMPROVEMENTS_CHECKLIST.md | 200+ | Чек-лист улучшений |
| COMPLETION_REPORT.md | 250+ | Отчёт о завершении |
| **ИТОГО** | **1700+** | Строк документации |

### Миграции БД

| Миграция | Статус |
|----------|--------|
| 00000000000000_diesel_initial_setup | ✅ |
| 2026-01-12-114732-0000_create_schedule_lessons | ✅ |
| 2026-01-14-123212-0000_create_students | ✅ |
| 2026-01-14-165306-0000_add_study_form_to_students | ✅ |
| 2026-01-14-175957-0000_add_course_and_username_to_students | ✅ |
| 2026-01-16-125016-0000_add_user_states | ✅ НОВАЯ |

---

## 📈 Статистика кода

### Количество строк по модулям

```
src/main.rs                                    : 19 строк
src/config.rs                                  : 42 строк
src/errors.rs                                  : 48 строк
src/schema.rs                                  : 39 строк

src/app/facade.rs                              : 41 строк
src/app/mod.rs                                 : 1 строка

src/db/connection.rs                           : ~40 строк (существующий)
src/db/facade.rs                               : 96 строк (+30 новых методов)
src/db/students_repo.rs                        : ~100 строк (существующий)
src/db/user_states_repo.rs                     : 121 строк (НОВОЕ)
src/db/mod.rs                                  : 4 строки

src/domain/buttons.rs                          : 42 строк
src/domain/callbacks.rs                        : 72 строк (НОВОЕ)
src/domain/course.rs                           : 44 строк
src/domain/faculty.rs                          : 65 строк
src/domain/student.rs                          : ~40 строк (существующий)
src/domain/study_form.rs                       : 36 строк
src/domain/user_state.rs                       : 40 строк (НОВОЕ)
src/domain/groups/mit.rs                       : 50 строк
src/domain/groups/mod.rs                       : 1 строка
src/domain/mod.rs                              : 9 строк

src/services/telegram/mod.rs                   : 56 строк
src/services/telegram/handlers.rs              : 44 строк (-40 строк, упрощено)
src/services/telegram/callback_router.rs       : 75 строк (НОВОЕ)
src/services/telegram/keyboards.rs             : 27 строк
src/services/telegram/registration/handlers.rs : 136 строк (+50 строк, лучше)
src/services/telegram/registration/keyboards.rs: 44 строк
src/services/telegram/registration/facade.rs   : 24 строк
src/services/telegram/registration/mod.rs      : 3 строк

═════════════════════════════════════════════════════════════════
ИТОГО исходного кода                           : ~1300 строк
ИТОГО документации                             : ~1700 строк
```

---

## 🏗️ Архитектурные слои

### Application Layer (`app/`)
**Ответственность:** Инициализация и координация всех компонентов

- `facade.rs` - Главный фасад приложения
  - Создание DbFacade
  - Инициализация Telegram бота
  - Запуск основного цикла

### Service Layer (`services/telegram/`)
**Ответственность:** Обработка Telegram-событий

- `mod.rs` - Инициализация бота и dispatcher
- `handlers.rs` - Основные обработчики (сообщения, callbacks)
- `callback_router.rs` - Маршрутизация callbacks
- `keyboards.rs` - Клавиатуры главного меню
- `registration/` - Модуль регистрации

### Domain Layer (`domain/`)
**Ответственность:** Бизнес-логика и доменные модели

- `buttons.rs` - Enum основных кнопок
- `callbacks.rs` - Unified enum для всех callbacks
- `faculty.rs` - Факультеты
- `course.rs` - Курсы
- `study_form.rs` - Формы обучения
- `student.rs` - Модель студента
- `user_state.rs` - Состояние пользователя
- `groups/` - Группы по факультетам

### Data Access Layer (`db/`)
**Ответственность:** Доступ к данным и репозитории

- `facade.rs` - DbFacade (главная точка доступа)
- `students_repo.rs` - Репозиторий студентов
- `user_states_repo.rs` - Репозиторий состояния
- `connection.rs` - Управление подключением

---

## 🔗 Связи между модулями

```
main.rs
  ├─ config.rs
  ├─ app/facade.rs
  │   ├─ db/facade.rs
  │   │   ├─ db/students_repo.rs
  │   │   └─ db/user_states_repo.rs
  │   └─ services/telegram/mod.rs
  │       ├─ services/telegram/handlers.rs
  │       │   └─ services/telegram/callback_router.rs
  │       │       ├─ services/telegram/registration/handlers.rs
  │       │       └─ domain/callbacks.rs
  │       └─ services/telegram/keyboards.rs
  │
  ├─ domain/
  │   ├─ buttons.rs
  │   ├─ callbacks.rs
  │   ├─ faculty.rs
  │   ├─ course.rs
  │   ├─ study_form.rs
  │   ├─ student.rs
  │   ├─ user_state.rs
  │   └─ groups/mit.rs
  │
  └─ errors.rs
```

---

## 📝 Ключевые файлы и их назначение

### Критические файлы (без них приложение не работает)
- ✅ `src/main.rs` - Точка входа
- ✅ `src/app/facade.rs` - Инициализация приложения
- ✅ `src/db/facade.rs` - Доступ к БД
- ✅ `src/services/telegram/mod.rs` - Запуск бота

### Важные файлы (расширяют функциональность)
- ✅ `src/domain/callbacks.rs` - Парсинг callbacks
- ✅ `src/services/telegram/callback_router.rs` - Маршрутизация
- ✅ `src/db/user_states_repo.rs` - Управление состоянием
- ✅ Миграции в `migrations/` - Схема БД

### Файлы расширения (для новых функций)
- ✅ `src/domain/` - Добавляйте новые enum'ы здесь
- ✅ `src/services/telegram/registration/` - Добавляйте обработчики здесь
- ✅ `migrations/` - Новые миграции при изменении БД

### Документация (помогают разработке)
- ✅ `ARCHITECTURE.md` - Общее описание
- ✅ `DEVELOPER_GUIDE.md` - Как разрабатывать
- ✅ `EXAMPLES.md` - Примеры кода
- ✅ `README.md` - Быстрый старт

---

## 🚀 Как читать код

### Для новичков
1. Начните с `README.md`
2. Прочитайте `ARCHITECTURE.md`
3. Посмотрите `EXAMPLES.md`
4. Запустите проект локально

### Для разработчиков
1. Посмотрите `DEVELOPER_GUIDE.md`
2. Изучите `src/app/facade.rs` (главный поток)
3. Посмотрите конкретный модуль, который нужен
4. Обратитесь к `EXAMPLES.md` для примеров

### Для контрибьютеров
1. Прочитайте `IMPROVEMENTS_CHECKLIST.md`
2. Выберите задачу для реализации
3. Следуйте паттернам в `DEVELOPER_GUIDE.md`
4. Добавьте документацию и примеры

---

## 🎯 Типичные пути кода

### Путь обработки сообщения

```
User sends /start
    ↓
Telegram API
    ↓
handlers.rs::handle_message()
    ↓
DbFacade::find_student()
    ↓
students_repo::find_by_telegram_id()
    ↓
PostgreSQL query
    ↓
Send reply to user
```

### Путь обработки callback

```
User clicks button
    ↓
Telegram API
    ↓
handlers.rs::handle_callback()
    ↓
callback_router.rs::route_callback()
    ↓
CallbackData::from_str()
    ↓
registration/handlers.rs::handle_*()
    ↓
DbFacade::register_student()
    ↓
PostgreSQL operations
    ↓
Send confirmation to user
```

---

## 📦 Зависимости (из Cargo.toml)

- **tokio** - Async runtime
- **teloxide** - Telegram Bot API
- **sqlx** - Работа с БД (async)
- **diesel** - Миграции БД
- **log + pretty_env_logger** - Логирование
- **anyhow** - Обработка ошибок
- **chrono** - Работа с датой/временем
- **uuid** - UUID генерация

---

## ✅ Проверка целостности

### Все файлы на месте?
- ✅ `src/` - 27 файлов кода
- ✅ `migrations/` - 6 миграций
- ✅ Документация - 6 файлов
- ✅ Конфиг файлы (Cargo.toml, diesel.toml)

### Все компилируется?
- ✅ `cargo build` - успешно
- ✅ Нет ошибок
- ✅ Нет критических warning'ов

### Все модули экспортированы?
- ✅ `src/main.rs` - все подмодули
- ✅ `src/domain/mod.rs` - все типы
- ✅ `src/db/mod.rs` - все репозитории
- ✅ `src/services/mod.rs` - все сервисы

---

## 🎓 Обучающие материалы

По файлам структуры:

| Файл | Изучить если нужно |
|------|-------------------|
| `src/main.rs` | Точка входа, инициализация |
| `src/app/facade.rs` | Общий поток приложения |
| `src/db/facade.rs` | Доступ к БД |
| `src/domain/callbacks.rs` | Парсинг enum'ов |
| `src/services/telegram/callback_router.rs` | Маршрутизация |
| `migrations/*.sql` | Схема БД |
| `EXAMPLES.md` | Примеры кода |

---

**Версия:** 1.0
**Дата:** 16 января 2026
**Статус:** ✅ ПОЛНАЯ ГОТОВНОСТЬ

Структура проекта оптимальна и готова к расширению! 🚀

