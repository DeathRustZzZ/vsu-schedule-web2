# Быстрый старт с системой меню

## Что было добавлено?

Полностью реработана система навигации бота с использованием удобных Inline меню (клавиатур).

### Новые модули:

1. **`domain/menu.rs`** - Определение команд и состояния меню
2. **`services/telegram/menu.rs`** - Фабрика для создания клавиатур
3. **`services/telegram/menu_dispatcher.rs`** - Диспетчер для обработки команд меню
4. **`services/telegram/menu_state_manager.rs`** - Менеджер состояния пользователей в меню

## Структура команд меню

```
🏠 Главное меню (menu_main)
├── 📝 Зарегистрироваться (menu_register)
├── 👤 Мой профиль (menu_profile)
├── 📅 Моё расписание (menu_schedule)
├── 👥 Выбрать группу (menu_choose_group)
├── ❓ Справка (menu_help)
└── ◀️ Назад (menu_back)
```

## Примеры использования

### 1. Основное меню в handlers.rs

```rust
use crate::services::telegram::menu;

// Для незарегистрированного пользователя
bot.send_message(
    msg.chat.id,
    "👋 Привет! Ты ещё не зарегистрирован.\n\n📝 Пройди регистрацию!"
)
.reply_markup(menu::main_menu_inline_keyboard())
.await?;

// Для зарегистрированного пользователя
bot.send_message(
    msg.chat.id,
    "👋 Привет! Выбери действие из меню:"
)
.reply_markup(menu::user_menu_inline_keyboard())
.await?;
```

### 2. Использование диспетчера в callback_router.rs

Диспетчер автоматически обрабатывает все команды меню:

```rust
use crate::services::telegram::menu_dispatcher::{MenuDispatcher, MenuCommandContext};

let context = MenuCommandContext {
    bot: bot.clone(),
    query: q,
};

MenuDispatcher::dispatch(&context, menu_cmd).await?;
```

### 3. Добавление новой команды в меню

Чтобы добавить новую команду:

**Шаг 1:** Добавьте в `domain/menu.rs`:
```rust
pub enum MenuCommand {
    // ... existing commands ...
    Settings,  // Новая команда
}
```

**Шаг 2:** Реализуйте методы:
```rust
impl MenuCommand {
    pub fn text(&self) -> &'static str {
        match self {
            // ... existing ...
            MenuCommand::Settings => "⚙️ Настройки",
        }
    }

    pub fn callback(&self) -> &'static str {
        match self {
            // ... existing ...
            MenuCommand::Settings => "menu_settings",
        }
    }
}
```

**Шаг 3:** Добавьте парсинг:
```rust
impl FromStr for MenuCommand {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // ... existing ...
            "menu_settings" => Ok(MenuCommand::Settings),
            // ... existing ...
        }
    }
}
```

**Шаг 4:** Добавьте обработчик в `menu_dispatcher.rs`:
```rust
async fn show_settings(context: &MenuCommandContext) -> Result<(), Box<dyn std::error::Error>> {
    context
        .bot
        .edit_message_text(
            context.query.from.id,
            context.query.message.as_ref().unwrap().id(),
            "⚙️ *Настройки*\n\nВыберите опцию:",
        )
        .reply_markup(menu::menu_inline_keyboard(&[MenuCommand::Back]))
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .await?;
    Ok(())
}
```

**Шаг 5:** Добавьте обработку в `MenuDispatcher::dispatch`:
```rust
MenuCommand::Settings => {
    info!("⚙️ Открыты настройки пользователем {}", context.query.from.id);
    Self::show_settings(context).await?;
}
```

### 4. Управление состоянием пользователя

```rust
use crate::services::telegram::menu_state_manager::MenuStateManager;
use crate::domain::menu::MenuState;

let manager = MenuStateManager::new();

// Получить текущее состояние
let state = manager.get_state(user_id).await;

// Установить новое состояние
manager.set_state(user_id, MenuState::UserMenu).await;

// Вернуться в главное меню
manager.reset_to_main(user_id).await;
```

## Преимущества новой системы

✅ **Type-Safe** - все команды определены в enum, нет строковых ошибок  
✅ **Централизованная маршрутизация** - все обработчики в одном месте  
✅ **Легко расширяемо** - просто добавьте новую команду в enum  
✅ **Логирование** - каждая команда автоматически логируется  
✅ **Красивый UI** - поддержка эмодзи и MarkdownV2 форматирования  
✅ **Иерархические меню** - кнопка "Назад" для навигации  
✅ **Асинхронная обработка** - полная поддержка async/await  

## Структура файлов

```
src/
├── domain/
│   └── menu.rs                 # Определения команд и состояний
├── services/
│   └── telegram/
│       ├── menu.rs             # Фабрика клавиатур
│       ├── menu_dispatcher.rs   # Обработчик команд
│       ├── menu_state_manager.rs # Управление состоянием
│       └── callback_router.rs    # Маршрутизатор (обновлён)
```

## Полный пример использования

```rust
// В handlers.rs
pub async fn handle_message(
    bot: Bot,
    msg: Message,
    db: Arc<DbFacade>,
) -> Result<(), teloxide::RequestError> {
    let telegram_id = msg.from.map(|u| u.id.0).unwrap_or(0);

    match db.find_student(telegram_id.try_into().unwrap()).await {
        Ok(Some(student)) => {
            // Зарегистрированный пользователь
            bot.send_message(
                msg.chat.id,
                format!("👋 Привет, {}!\n\n📚 Выбери действие:", student.group_name)
            )
            .reply_markup(menu::user_menu_inline_keyboard())
            .await?;
        }
        Ok(None) => {
            // Не зарегистрированный пользователь
            bot.send_message(
                msg.chat.id,
                "👋 Привет! Зарегистрируйся, чтобы начать."
            )
            .reply_markup(menu::main_menu_inline_keyboard())
            .await?;
        }
        Err(_) => {
            bot.send_message(msg.chat.id, "❌ Ошибка. Попробуй позже.")
                .await?;
        }
    }
    Ok(())
}
```

## Тестирование в Telegram

1. Напишите боту `/start`
2. Нажмите кнопку меню
3. Навигируйте по меню используя кнопки
4. Используйте кнопку "Назад" для возврата

## Дальнейшее развитие

Система готова к расширению. Вы можете:

- Добавить новые команды в enum `MenuCommand`
- Создать подменю для каждой команды
- Интегрировать состояние пользователя с базой данных
- Добавить кэширование часто используемых данных
- Реализовать inline поиск (для выбора группы, расписания и т.д.)


