# Архитектура системы меню

## Обзор

Система меню представляет собой полностью переработанный модуль навигации Telegram бота с использованием современного подхода к организации кода.

## Компоненты системы

### 1. Domain Layer (`domain/menu.rs`)

**Ответственность:** Определение типов данных и бизнес-логики меню

```
MenuCommand enum
├── MainMenu
├── Register
├── MyProfile
├── MySchedule
├── ChooseGroup
├── Back
└── Help
```

**Методы:**
- `text()` - текст для отображения пользователю
- `callback()` - уникальный идентификатор для callback-обработки
- `FromStr` - парсинг строк в enum

```
MenuState enum
├── Main
├── Registration
├── UserMenu
└── Schedule
```

### 2. Services Layer

#### 2.1 Factories (`services/telegram/menu.rs`)

**Ответственность:** Создание Inline клавиатур различных типов

```rust
Functions:
├── main_menu_inline_keyboard()      // Меню для новых пользователей
├── user_menu_inline_keyboard()      // Меню для зарегистрированных
├── registration_inline_keyboard()   // Регистрация
├── menu_inline_keyboard()          // Простое меню
├── menu_two_column_keyboard()      // Двухколоночное меню
├── menu_with_back_keyboard()       // С кнопкой "Назад"
└── simple_inline_keyboard()        // Динамическое меню
```

#### 2.2 Dispatcher (`services/telegram/menu_dispatcher.rs`)

**Ответственность:** Обработка команд меню и формирование ответов

```rust
MenuDispatcher::dispatch()
├── MainMenu → show_main_menu()
├── Register → show_registration_menu()
├── MyProfile → show_profile()
├── MySchedule → show_schedule()
├── ChooseGroup → show_group_selection()
├── Back → show_main_menu()
└── Help → show_help()
```

**Особенности:**
- Автоматическое логирование команд
- Уведомление пользователя об обработке
- Форматирование MarkdownV2

#### 2.3 State Manager (`services/telegram/menu_state_manager.rs`)

**Ответственность:** Управление состоянием пользователя в меню

```rust
Methods:
├── get_state(user_id)          // Получить текущее состояние
├── set_state(user_id, state)   // Установить новое состояние
├── reset_to_main(user_id)      // Вернуться в главное меню
├── clear_state(user_id)        // Удалить состояние
└── cache_size()                // Размер кэша
```

#### 2.4 Callback Router (`services/telegram/callback_router.rs`)

**Ответственность:** Маршрутизация callback-запросов

```
route_callback()
├── MenuCommand? 
│   └── MenuDispatcher::dispatch()
└── CallbackData?
    └── handle_callback_data()
```

### 3. Handlers (`services/telegram/handlers.rs`)

**Ответственность:** Обработка сообщений и начальная маршрутизация

```
handle_message()
├── Зарегистрирован?
│   └── user_menu_inline_keyboard()
└── Не зарегистрирован?
    └── main_menu_inline_keyboard()

handle_callback()
└── callback_router::route_callback()
```

## Поток данных

```
Пользователь пишет сообщение
        ↓
  handle_message()
        ↓
    Проверка БД
    ↙         ↘
Зарегистр.  Не зарегистр.
    ↓             ↓
  user_menu      main_menu
    ↓             ↓
Нажимает кнопку (callback)
        ↓
  handle_callback()
        ↓
  route_callback()
    ↙         ↘
MenuCommand  CallbackData
    ↓             ↓
MenuDispatcher  registration
    ↓             handlers
 dispatch()       ↓
    ↓             ↓
Показать меню  Обработка регистрации
```

## Примеры интеграции

### Добавление новой команды

```rust
// 1. domain/menu.rs
pub enum MenuCommand {
    // ...
    NewCommand,
}

// 2. Реализовать методы
impl MenuCommand {
    pub fn text(&self) -> &'static str {
        match self {
            // ...
            MenuCommand::NewCommand => "🆕 Новая команда",
        }
    }

    pub fn callback(&self) -> &'static str {
        match self {
            // ...
            MenuCommand::NewCommand => "menu_new",
        }
    }
}

// 3. FromStr
impl FromStr for MenuCommand {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // ...
            "menu_new" => Ok(MenuCommand::NewCommand),
        }
    }
}

// 4. menu_dispatcher.rs
async fn show_new_command(context: &MenuCommandContext) 
    -> Result<(), Box<dyn std::error::Error>> 
{
    context.bot
        .edit_message_text(...)
        .reply_markup(menu::menu_inline_keyboard(&[MenuCommand::Back]))
        .await?;
    Ok(())
}

// 5. MenuDispatcher::dispatch()
NewCommand => {
    info!("🆕 Новая команда от {}", context.query.from.id);
    Self::show_new_command(context).await?;
}
```

## Преимущества архитектуры

### 1. **Разделение ответственности**
- Domain: типы данных
- Services: логика обработки
- Handlers: интеграция

### 2. **Масштабируемость**
- Легко добавлять новые команды
- Возможность создания подменю
- Модульная структура

### 3. **Тестируемость**
```rust
#[test]
fn test_menu_command() {
    assert_eq!(MenuCommand::MainMenu.callback(), "menu_main");
}
```

### 4. **Типобезопасность**
- Enum вместо строк
- Compile-time проверка
- Нет runtime ошибок

### 5. **Производительность**
- Кэширование состояния (HashMap)
- Оптимальная маршрутизация
- Минимальные аллокации

### 6. **Удобство разработки**
- Централизованное управление
- Понятная структура
- Хорошо задокументировано

## Структура папок

```
src/
├── domain/
│   └── menu.rs                      # 🎯 Определения типов
├── services/
│   └── telegram/
│       ├── menu.rs                  # 🎨 Фабрика клавиатур
│       ├── menu_dispatcher.rs        # 🔄 Обработчик команд
│       ├── menu_state_manager.rs     # 💾 Управление состоянием
│       ├── menu_examples.rs          # 📚 Примеры использования
│       ├── callback_router.rs        # 🛣️ Маршрутизатор (обновлён)
│       └── handlers.rs               # 📬 Обработчики (обновлены)
└── ...
```

## Возможности расширения

1. **Database Integration**
   - Сохранение предпочтений пользователя
   - Интеграция MenuState с БД

2. **Advanced Routing**
   - Условная навигация в зависимости от состояния
   - Dynamic меню на основе данных

3. **Caching**
   - Кэширование часто используемых менюкомбинаций
   - Оптимизация памяти

4. **Localization**
   - Многоязычная поддержка
   - Локализация текстов команд

5. **Analytics**
   - Отслеживание популярных команд
   - Метрики использования меню

## Миграция из старой системы

Если в вашем коде были старые обработчики:

**Было:**
```rust
// Множество if let блоков
if let TechButton::Register = callback {
    // обработка
}
```

**Стало:**
```rust
// Централизованный dispatcher
MenuDispatcher::dispatch(&context, MenuCommand::Register).await?;
```

## Заключение

Новая система меню обеспечивает:
- ✅ Чистую архитектуру
- ✅ Легкую поддержку
- ✅ Быструю разработку
- ✅ Надёжность
- ✅ Масштабируемость

Система готова к production использованию и может быть легко адаптирована под специфические требования проекта.

