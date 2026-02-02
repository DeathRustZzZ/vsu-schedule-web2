# 🏗️ Архитектура проекта VSU Telegram Bot

## Обзор

Проект представляет собой Telegram бот для управления расписанием студентов ВГУ, построенный на чистой архитектуре с разделением ответственности.

## 📁 Структура проекта

```
src/
├── main.rs                    # Точка входа в приложение
├── config.rs                  # Конфигурация приложения
├── errors.rs                  # Типы ошибок приложения
├── app/                       # Слой приложения (Application Layer)
│   ├── mod.rs
│   └── facade.rs              # Главный фасад приложения
│
├── db/                        # Слой доступа к данным (Data Access Layer)
│   ├── connection.rs          # Инициализация подключения к БД
│   ├── facade.rs              # Фасад БД (основная точка доступа)
│   ├── students_repo.rs       # Репозиторий студентов
│   ├── user_states_repo.rs    # Репозиторий состояния пользователей
│   └── mod.rs
│
├── domain/                    # Бизнес-логика (Domain Layer)
│   ├── buttons.rs             # Enum основных кнопок
│   ├── callbacks.rs           # Unified enum для всех callbacks
│   ├── course.rs              # Enum курсов
│   ├── faculty.rs             # Enum факультетов
│   ├── student.rs             # Модель студента
│   ├── study_form.rs          # Enum формы обучения
│   ├── user_state.rs          # Модель состояния пользователя
│   ├── groups/
│   │   ├── mit.rs             # Enum групп факультета МИТ
│   │   └── mod.rs
│   └── mod.rs
│
└── services/                  # Слой сервисов (Service Layer)
    └── telegram/              # Telegram интеграция
        ├── mod.rs             # Инициализация бота
        ├── handlers.rs        # Основные обработчики сообщений и callbacks
        ├── callback_router.rs # Центральный маршрутизатор callbacks
        ├── keyboards.rs       # Клавиатуры главного меню
        └── registration/      # Модуль регистрации
            ├── mod.rs
            ├── handlers.rs    # Обработчики регистрационных этапов
            ├── keyboards.rs   # Клавиатуры для регистрации
            └── facade.rs      # Фасад регистрации
```

## 🏗️ Слои архитектуры

### 1. **Application Layer** (`app/`)
- Главный фасад приложения
- Инициализирует все компоненты
- Связывает все слои вместе

### 2. **Service Layer** (`services/`)
- Обработка Telegram-событий
- Маршрутизация callback-запросов
- Генерация клавиатур

### 3. **Domain Layer** (`domain/`)
- Бизнес-объекты (Faculty, Course, StudyForm, etc.)
- Трансформация данных между слоями
- **Не зависит** от фреймворков и деталей реализации

### 4. **Data Access Layer** (`db/`)
- Репозитории для доступа к данным
- Фасад БД для централизованного доступа
- Работа с sqlx

## 🔄 Поток данных

```
Telegram API
    ↓
handlers.rs (handle_message, handle_callback)
    ↓
callback_router.rs (маршрутизация)
    ↓
registration/handlers.rs (обработчики регистрации)
    ↓
db/facade.rs (доступ к БД)
    ↓
user_states_repo.rs / students_repo.rs (репозитории)
    ↓
PostgreSQL
```

## ✨ Ключевые улучшения

### 1. **Единый парсер callbacks** (`domain/callbacks.rs`)
Заменяет множество `if let` блоков на единый enum:

```rust
// Было
if let Ok(btn) = TechButton::from_str(&data) { ... }
if let Ok(fac) = Faculty::from_str(&data) { ... }
// ...много if let блоков

// Теперь
match CallbackData::from_str(&data) {
    Ok(callback) => handle_callback_data(bot, q, db, callback).await?
}
```

### 2. **Централизованный маршрутизатор** (`services/telegram/callback_router.rs`)
Все callbacks обрабатываются через один router, что делает код более масштабируемым.

### 3. **Состояние пользователя** (`domain/user_state.rs`, `db/user_states_repo.rs`)
Возможность отслеживать этап регистрации и сохранять выбранные значения:
- Факультет
- Форма обучения
- Курс

### 4. **Чистое разделение ответственности**
- Каждый модуль отвечает за одно
- Легко добавлять новые функции
- Легко тестировать

## 🚀 Как добавить новую функцию

### Пример 1: Добавить новую кнопку

1. **Добавить в `domain/buttons.rs`:**
```rust
pub enum TechButton {
    Register,
    MySchedule,
    ChooseGroup,
    NewFeature,  // Новая кнопка
}
```

2. **Добавить обработчик в `services/telegram/registration/handlers.rs`:**
```rust
pub async fn handle_new_feature(bot: Bot, q: CallbackQuery) -> Result<(), teloxide::RequestError> {
    // Логика обработки
}
```

3. **Обновить `callback_router.rs`** для маршрутизации.

### Пример 2: Добавить новый факультет

1. **Добавить в `domain/faculty.rs`:**
```rust
pub enum Faculty {
    Mit,
    NewFaculty,  // Новый факультет
    // ...
}
```

2. **Обновить методы `title()` и `callback()`**
3. **Обновить `FromStr` реализацию**
4. **Обновить клавиатуру в `registration/keyboards.rs`**

## 🗄️ База данных

### Таблица `students`
```sql
CREATE TABLE students (
    id UUID PRIMARY KEY,
    telegram_id BIGINT UNIQUE NOT NULL,
    faculty VARCHAR NOT NULL,
    group_name VARCHAR NOT NULL,
    study_form VARCHAR NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE,
    course VARCHAR
);
```

### Таблица `user_states`
```sql
CREATE TABLE user_states (
    id SERIAL PRIMARY KEY,
    telegram_id BIGINT UNIQUE NOT NULL,
    state VARCHAR DEFAULT 'idle',
    faculty VARCHAR,
    study_form VARCHAR,
    course VARCHAR,
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);
```

## 📝 Стиль кода

- **Документация**: Каждый публичный модуль и функция имеет doc-комментарий
- **Логирование**: Использование `log` crate с разными уровнями (debug, info, error)
- **Обработка ошибок**: Использование `anyhow::Result` для сложных операций
- **Именование**: Ясные, описательные имена переменных и функций

## 🔐 Безопасность

- Все параметры пользователя параметризуются при работе с БД (защита от SQL injection)
- Использование типизированного Rust для предотвращения ошибок типов
- Async/await для предотвращения блокировок

## 📈 Масштабируемость

Архитектура позволяет легко:
- ➕ Добавлять новые факультеты и группы
- ➕ Добавлять новые кнопки и команды
- ➕ Добавлять новые сервисы (не только Telegram)
- ➕ Переходить на другие базы данных
- ➕ Добавлять кеширование

## 🧪 Тестирование

Модульная архитектура упрощает написание тестов:
- Репозитории можно mock'ировать
- Бизнес-логика отделена от фреймворков
- Каждый слой независим

---

**Дата создания**: Январь 2026
**Версия**: 1.0
