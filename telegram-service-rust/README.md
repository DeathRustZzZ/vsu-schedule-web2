# 📱 VSU Telegram Bot

Telegram бот для студентов ВГУ имени Петра Первого для управления расписанием и регистрации.

## ✨ Возможности

- 📚 **Управление расписанием** - Просмотр расписания занятий
- ✍️ **Регистрация студентов** - Быстрая регистрация в системе
- 👥 **Управление группами** - Выбор факультета, формы обучения и группы
- 🔐 **Безопасное хранение данных** - PostgreSQL БД с валидными запросами
- 🚀 **Высокопроизводительный** - Async/await для параллельной обработки
- 🏗️ **Чистая архитектура** - Модульный и масштабируемый код

## 🚀 Быстрый старт

### Требования
- Rust 1.70+
- PostgreSQL 13+
- Telegram Bot Token (от [@BotFather](https://t.me/BotFather))

### Установка

1. **Клонируйте репозиторий:**
```bash
git clone https://github.com/yourusername/tg_bot_vsu.git
cd tg_bot_vsu
```

2. **Создайте файл `.env`:**
```env
BOT_TOKEN=your_telegram_bot_token_here
DATABASE_URL=postgresql://user:password@localhost/tg_bot_vsu
RUST_LOG=info
```

3. **Инициализируйте БД:**
```bash
# Установите Diesel CLI
cargo install diesel_cli --no-default-features --features postgres

# Запустите миграции
diesel migration run
```

4. **Запустите бота:**
```bash
cargo run
```

## 📖 Документация

- **[ARCHITECTURE.md](./ARCHITECTURE.md)** - Описание архитектуры приложения
- **[DEVELOPER_GUIDE.md](./DEVELOPER_GUIDE.md)** - Руководство разработчика
- **[EXAMPLES.md](./EXAMPLES.md)** - Примеры использования API

## 🏗️ Структура проекта

```
src/
├── main.rs                    # Точка входа
├── config.rs                  # Конфигурация
├── errors.rs                  # Типы ошибок
├── app/                       # Слой приложения
├── db/                        # Доступ к БД
├── domain/                    # Бизнес-логика
└── services/                  # Интеграции
    └── telegram/              # Telegram бот
```

Подробнее в [ARCHITECTURE.md](./ARCHITECTURE.md)

## 🎯 Основные улучшения

### ✅ Единый парсер callbacks
Вместо множества `if let` блоков используется единый `CallbackData` enum:

```rust
// Раньше
if let Ok(btn) = TechButton::from_str(&data) { ... }
if let Ok(fac) = Faculty::from_str(&data) { ... }
// ... много if let блоков

// Теперь
match CallbackData::from_str(&data) {
    Ok(callback) => handle_callback_data(bot, q, db, callback).await?
}
```

### ✅ Централизованный маршрутизатор
Все callbacks обрабатываются через один `callback_router`, делая код более масштабируемым.

### ✅ Состояние пользователя
Отслеживание этапа регистрации и сохранение выбранных значений:
- Факультет
- Форма обучения  
- Курс

### ✅ Чистое разделение ответственности
- **Domain** - бизнес-логика, независима от фреймворков
- **Services** - обработка Telegram-событий
- **DB** - доступ к данным через репозитории
- **App** - инициализация и координация

## 📚 Типичный поток

```
Telegram API
    ↓
Message/Callback Handler
    ↓
Callback Router
    ↓
Registration Handlers
    ↓
DB Facade (DbFacade)
    ↓
Repositories
    ↓
PostgreSQL
```

## 🔧 Добавление новой функции

### Добавить новую кнопку

1. Добавьте в `domain/buttons.rs`
2. Добавьте обработчик в `registration/handlers.rs`
3. Обновите `callback_router.rs`

Подробнее в [DEVELOPER_GUIDE.md](./DEVELOPER_GUIDE.md#добавление-новых-функций)

## 📊 База данных

### Таблица `students`
Хранит информацию о зарегистрированных студентах

### Таблица `user_states`
Хранит состояние пользователя во время регистрации

### Таблица `schedule_lessons`
Хранит расписание занятий

## 🧪 Тестирование

```bash
# Запуск всех тестов
cargo test

# Запуск с выводом
cargo test -- --nocapture

# Запуск конкретного теста
cargo test test_name
```

## 📝 Логирование

Проект использует `log` crate с разными уровнями:

```bash
# Запуск с debug логами
RUST_LOG=debug cargo run

# Или в .env
RUST_LOG=info
```

## 🚨 Обработка ошибок

Используется `anyhow::Result` для гибкой обработки ошибок:

```rust
pub async fn risky_operation() -> anyhow::Result<Data> {
    let data = db.query().await?;
    Ok(data)
}
```

## 🔐 Безопасность

- ✅ Параметризованные SQL-запросы (защита от SQL injection)
- ✅ Типизированный Rust (prevention of type errors)
- ✅ Async/await (prevention of blocking)
- ✅ Arc для безопасного шаринга данных

## 📦 Зависимости

- **teloxide** - Telegram Bot API
- **tokio** - Async runtime
- **sqlx** - Работа с БД
- **diesel** - Миграции
- **log** - Логирование
- **anyhow** - Обработка ошибок

## 🤝 Contributon

Contributions приветствуются! Пожалуйста:

1. Fork репозиторий
2. Создайте feature branch
3. Commit изменения
4. Push в branch
5. Создайте Pull Request

## 📄 Лицензия

Этот проект распределяется под лицензией MIT. Смотрите [LICENSE](./LICENSE) для деталей.

## 👨‍💻 Автор

**deathcrush** - Создатель и основной разработчик

## 📞 Контакты

- Telegram: [@botusername](https://t.me/botusername)
- Email: your.email@example.com
- Issues: [GitHub Issues](https://github.com/yourusername/tg_bot_vsu/issues)

## 🙏 Благодарности

Спасибо за помощь:
- [teloxide](https://github.com/teloxide/teloxide) - отличная библиотека для Telegram бота
- Сообществу Rust за excellent tooling

## 📈 Roadmap

- [ ] Интеграция с календарём
- [ ] Push-уведомления о изменениях расписания
- [ ] Экспорт расписания в ICS
- [ ] Поддержка других факультетов
- [ ] Многоязычная поддержка
- [ ] Мобильное приложение

---

**Версия**: 1.0
**Дата**: Январь 2026

Для более подробной информации смотрите [документацию](./ARCHITECTURE.md).

