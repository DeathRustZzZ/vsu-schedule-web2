# 📖 Руководство разработчика

## Как запустить проект

### Требования
- Rust 1.70+
- PostgreSQL 13+
- Telegram Bot Token

### Шаги установки

1. **Клонируйте репозиторий и установите зависимости:**
```bash
cd tg_bot_vsu
cargo build
```

2. **Создайте файл `.env`:**
```env
BOT_TOKEN=your_telegram_bot_token
DATABASE_URL=postgresql://user:password@localhost/tg_bot_vsu
RUST_LOG=info
```

3. **Инициализируйте БД:**
```bash
# Установите Diesel CLI если ещё не установлен
cargo install diesel_cli --no-default-features --features postgres

# Запустите миграции
diesel migration run
```

4. **Запустите бота:**
```bash
cargo run
```

## Основные компоненты

### 1. Application Facade
Главная точка входа в приложение.

**Файл:** `src/app/facade.rs`

```rust
pub async fn run_app(config: AppConfig, pool: PgPool) {
    let db_facade = DbFacade::new(pool);
    let bot = teloxide::Bot::new(config.bot_token.clone());
    run_telegram_bot(bot, db_facade).await;
}
```

### 2. Database Facade
Единая точка доступа к БД.

**Файл:** `src/db/facade.rs`

```rust
// Поиск студента
let student = db.find_student(telegram_id).await?;

// Регистрация студента
let student = db.register_student(
    telegram_id,
    "МИТ",
    "ИСИТ",
    "Очная"
).await?;

// Управление состоянием
let state = db.get_user_state(telegram_id).await?;
db.set_user_faculty(telegram_id, "МИТ").await?;
```

### 3. Message Handlers
Обработчики сообщений от пользователей.

**Файл:** `src/services/telegram/handlers.rs`

```rust
pub async fn handle_message(
    bot: Bot,
    msg: Message,
    db: Arc<DbFacade>,
) -> Result<(), teloxide::RequestError> {
    // Логика обработки сообщения
}
```

### 4. Callback Router
Маршрутизатор для callback-запросов.

**Файл:** `src/services/telegram/callback_router.rs`

```rust
pub async fn route_callback(
    bot: Bot,
    q: CallbackQuery,
    db: Arc<DbFacade>,
) -> Result<(), teloxide::RequestError> {
    if let Some(data) = q.data.clone() {
        match CallbackData::from_str(&data) {
            Ok(callback) => handle_callback_data(bot, q, db, callback).await?,
            Err(e) => warn!("Ошибка парсинга: {}", e),
        }
    }
    Ok(())
}
```

### 5. Registration Handlers
Обработчики этапов регистрации.

**Файл:** `src/services/telegram/registration/handlers.rs`

```rust
pub async fn handle_faculty_choice(
    bot: Bot,
    q: CallbackQuery,
    faculty: Faculty,
) -> Result<(), teloxide::RequestError> {
    // Обработка выбора факультета
    bot.send_message(q.from.id, "Факультет выбран!").await?;
    Ok(())
}
```

## Добавление новых функций

### Добавить новую команду

1. **Добавьте в `domain/buttons.rs`:**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TechButton {
    Register,
    MySchedule,
    NewCommand,  // Новая команда
}

impl TechButton {
    pub fn title(&self) -> &'static str {
        match self {
            // ...
            TechButton::NewCommand => "Новая команда",
        }
    }

    pub fn callback(&self) -> &'static str {
        match self {
            // ...
            TechButton::NewCommand => "new_command",
        }
    }
}

impl FromStr for TechButton {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // ...
            "new_command" => Ok(TechButton::NewCommand),
            _ => Err(()),
        }
    }
}
```

2. **Добавьте обработчик в `services/telegram/registration/handlers.rs`:**
```rust
pub async fn handle_new_command(
    bot: Bot,
    q: CallbackQuery,
) -> Result<(), teloxide::RequestError> {
    bot.send_message(q.from.id, "Обработка новой команды!").await?;
    Ok(())
}
```

3. **Обновите `callback_router.rs`:**
```rust
TechButton(btn) => {
    registration::handlers::handle_tech_button(bot, q, btn).await?;
}
```

### Добавить новый enum (Факультет, Курс и т.д.)

1. **Создайте новый файл в `domain/`:**
```rust
// domain/new_enum.rs
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NewEnum {
    Option1,
    Option2,
}

impl NewEnum {
    pub fn title(&self) -> &'static str {
        match self {
            NewEnum::Option1 => "Опция 1",
            NewEnum::Option2 => "Опция 2",
        }
    }

    pub fn callback(&self) -> &'static str {
        match self {
            NewEnum::Option1 => "new_enum_option1",
            NewEnum::Option2 => "new_enum_option2",
        }
    }
}

impl FromStr for NewEnum {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "new_enum_option1" => Ok(NewEnum::Option1),
            "new_enum_option2" => Ok(NewEnum::Option2),
            _ => Err(()),
        }
    }
}
```

2. **Добавьте в `domain/callbacks.rs`:**
```rust
pub enum CallbackData {
    // ... существующие варианты
    NewEnum(NewEnum),  // Новый вариант
}

impl FromStr for CallbackData {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // ... существующие проверки
        if let Ok(new_enum) = NewEnum::from_str(s) {
            return Ok(CallbackData::NewEnum(new_enum));
        }
        Err(format!("Неизвестный callback: {}", s))
    }
}
```

3. **Обновите `callback_router.rs`:**
```rust
NewEnum(new_enum) => {
    // Обработка нового enum
}
```

### Добавить операцию с БД

1. **Добавьте функцию в репозиторий:**
```rust
// db/new_repo.rs
pub async fn my_operation(pool: &PgPool, param: &str) -> anyhow::Result<Data> {
    let result = sqlx::query_as::<_, Data>(
        "SELECT * FROM my_table WHERE column = $1"
    )
    .bind(param)
    .fetch_one(pool)
    .await?;

    Ok(result)
}
```

2. **Добавьте в `db/mod.rs`:**
```rust
pub mod new_repo;
```

3. **Добавьте в `db/facade.rs`:**
```rust
pub async fn my_operation(&self, param: &str) -> anyhow::Result<Data> {
    new_repo::my_operation(&self.pool, param).await
}
```

## Тестирование

### Запуск тестов
```bash
cargo test
```

### Написание теста
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        // Ваш тест
    }
}
```

## Логирование

Используется `log` crate. Уровни:
- `debug!()` - Детальная информация для отладки
- `info!()` - Основные события
- `warn!()` - Предупреждения
- `error!()` - Ошибки

```rust
use log::{debug, info, warn, error};

debug!("Отладочная информация: {:?}", value);
info!("Пользователь зарегистрирован: {}", user_id);
warn!("Неполные данные: {:?}", data);
error!("Ошибка при регистрации: {:?}", err);
```

## Обработка ошибок

Используется `anyhow::Result` для гибкой обработки ошибок:

```rust
pub async fn risky_operation() -> anyhow::Result<Data> {
    let data = sqlx::query("SELECT * FROM table")
        .fetch_one(&pool)
        .await?;
    Ok(data)
}
```

## Советы по производительности

1. **Используйте `Arc::clone()` для дешёвого клонирования**
```rust
let db = Arc::clone(&db);
```

2. **Параметризуйте SQL-запросы**
```rust
sqlx::query("SELECT * FROM users WHERE id = $1")
    .bind(user_id)
    .fetch_one(&pool)
    .await?
```

3. **Используйте async/await для избежания блокировок**
```rust
pub async fn handle_request(bot: Bot) -> Result<(), Error> {
    let result = db.query().await?;
    Ok(())
}
```

## Полезные команды

```bash
# Запуск с логами
RUST_LOG=debug cargo run

# Проверка кода без компиляции
cargo check

# Форматирование кода
cargo fmt

# Проверка линтером
cargo clippy

# Запуск тестов с выводом
cargo test -- --nocapture

# Создание новой миграции
diesel migration generate name_of_migration

# Откат последней миграции
diesel migration revert
```

---

Для дополнительной информации смотрите `ARCHITECTURE.md`

