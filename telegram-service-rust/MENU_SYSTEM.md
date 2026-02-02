# Система Меню - Документация

## Обзор

Система меню обеспечивает удобную навигацию для пользователей Telegram бота с использованием Inline клавиатур.

## Структура

### Модули

1. **`domain/menu.rs`** - Определение команд и состояния меню
   - `MenuCommand` - Enum для основных команд меню
   - `MenuState` - Enum для состояния меню

2. **`services/telegram/menu.rs`** - Фабрика клавиатур
   - Функции для создания различных типов меню
   - Поддержка одноколоночных и двухколоночных макетов

3. **`services/telegram/callback_router.rs`** - Маршрутизатор команд
   - Централизованная обработка callback-запросов
   - Разделение меню команд и регистрационных данных

## Команды Меню

| Команда | Текст | Callback |
|---------|-------|----------|
| `MainMenu` | 🏠 Главное меню | `menu_main` |
| `Register` | 📝 Зарегистрироваться | `menu_register` |
| `MyProfile` | 👤 Мой профиль | `menu_profile` |
| `MySchedule` | 📅 Моё расписание | `menu_schedule` |
| `ChooseGroup` | 👥 Выбрать группу | `menu_choose_group` |
| `Back` | ◀️ Назад | `menu_back` |
| `Help` | ❓ Справка | `menu_help` |

## Функции для создания клавиатур

### Готовые меню

```rust
// Главное меню для незарегистрированного пользователя
let keyboard = menu::main_menu_inline_keyboard();

// Меню зарегистрированного пользователя
let keyboard = menu::user_menu_inline_keyboard();
```

### Пользовательские меню

```rust
// Меню с двумя колонками
let commands = vec![MenuCommand::MyProfile, MenuCommand::MySchedule];
let keyboard = menu::menu_two_column_keyboard(&commands);

// Меню с кнопкой "Назад"
let commands = vec![MenuCommand::MyProfile];
let keyboard = menu::menu_with_back_keyboard(&commands);

// Простое меню
let commands = vec![MenuCommand::Help];
let keyboard = menu::menu_inline_keyboard(&commands);
```

## Использование в Handler'ах

### Отправка сообщения с меню

```rust
bot.send_message(chat_id, "Выберите действие:")
    .reply_markup(menu::user_menu_inline_keyboard())
    .await?;
```

### Редактирование сообщения с меню

```rust
bot.edit_message_text(chat_id, message_id, "Обновлённое сообщение")
    .reply_markup(menu::menu_inline_keyboard(&[MenuCommand::Back]))
    .await?;
```

## Обработка Callback'ов

Все callback'ы от меню автоматически маршрутизируются в функцию `handle_menu_command`:

```rust
pub async fn handle_menu_command(
    bot: Bot,
    q: CallbackQuery,
    command: MenuCommand,
) -> Result<(), Box<dyn std::error::Error>>
```

Каждая команда имеет свой обработчик в match выражении.

## Примеры использования

### Добавление нового пункта меню

1. Добавьте вариант в enum `MenuCommand` в `domain/menu.rs`:

```rust
pub enum MenuCommand {
    // ...existing...
    NewCommand,
}
```

2. Реализуйте методы `title()` и `callback()`:

```rust
impl MenuCommand {
    pub fn title(&self) -> &'static str {
        match self {
            // ...existing...
            MenuCommand::NewCommand => "🆕 Новая команда",
        }
    }

    pub fn callback(&self) -> &'static str {
        match self {
            // ...existing...
            MenuCommand::NewCommand => "menu_new_command",
        }
    }
}
```

3. Добавьте парсинг в `FromStr`:

```rust
impl FromStr for MenuCommand {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // ...existing...
            "menu_new_command" => Ok(MenuCommand::NewCommand),
            // ...existing...
        }
    }
}
```

4. Обработайте команду в `callback_router.rs`:

```rust
NewCommand => {
    // Ваша логика обработки
    bot.send_message(q.from.id, "Обработка новой команды...")
        .await?;
}
```

### Создание иерархического меню

```rust
// Главное меню
bot.send_message(chat_id, "Главное меню")
    .reply_markup(menu::menu_two_column_keyboard(&[
        MenuCommand::MyProfile,
        MenuCommand::MySchedule,
    ]))
    .await?;

// При клике - подменю
bot.edit_message_text(chat_id, msg_id, "Подменю")
    .reply_markup(menu::menu_with_back_keyboard(&[
        MenuCommand::MyProfile,
    ]))
    .await?;

// Кнопка Back возвращает в главное меню
```

## Преимущества текущей реализации

✅ **Централизованное управление** - все меню в одном месте  
✅ **Легко расширяемо** - просто добавляйте новые команды  
✅ **Type-safe** - использование enum вместо строк  
✅ **Удобная навигация** - иерархические меню с кнопкой "Назад"  
✅ **Отзывчивый дизайн** - меню автоматически форматируется  
✅ **Эмодзи поддержка** - красивые иконки для каждой команды  

