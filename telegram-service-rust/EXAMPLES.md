# 🎯 Примеры использования API

## Управление студентами

### Регистрация нового студента

```rust
use crate::db::facade::DbFacade;

async fn register_student_example(db: &DbFacade) {
    match db.register_student(
        123456789,           // telegram_id
        "МИТ",              // faculty
        "ИСИТ",             // group
        "Очная"             // study_form
    ).await {
        Ok(student) => {
            println!("Студент зарегистрирован: {:?}", student);
        }
        Err(e) => {
            eprintln!("Ошибка: {}", e);
        }
    }
}
```

### Поиск студента

```rust
async fn find_student_example(db: &DbFacade) {
    match db.find_student(123456789).await {
        Ok(Some(student)) => {
            println!("Найден студент: {}", student.group_name);
        }
        Ok(None) => {
            println!("Студент не найден");
        }
        Err(e) => {
            eprintln!("Ошибка: {}", e);
        }
    }
}
```

### Обновление данных студента

```rust
async fn update_student_example(db: &DbFacade) {
    match db.update_student(
        123456789,
        "Юридический факультет",
        "ЮР-1",
        "Заочная"
    ).await {
        Ok(Some(student)) => {
            println!("Данные обновлены: {:?}", student);
        }
        Ok(None) => {
            println!("Студент не найден");
        }
        Err(e) => {
            eprintln!("Ошибка: {}", e);
        }
    }
}
```

### Удаление студента

```rust
async fn delete_student_example(db: &DbFacade) {
    match db.delete_student(123456789).await {
        Ok(rows) => {
            println!("Удалено строк: {}", rows);
        }
        Err(e) => {
            eprintln!("Ошибка: {}", e);
        }
    }
}
```

## Управление состоянием пользователя

### Создание состояния при начале регистрации

```rust
async fn start_registration_example(db: &DbFacade, telegram_id: i64) {
    match db.create_user_state(telegram_id).await {
        Ok(state) => {
            println!("Состояние создано: {:?}", state);
        }
        Err(e) => {
            eprintln!("Ошибка: {}", e);
        }
    }
}
```

### Получение состояния пользователя

```rust
async fn get_user_state_example(db: &DbFacade, telegram_id: i64) {
    match db.get_user_state(telegram_id).await {
        Ok(Some(state)) => {
            if state.is_complete() {
                println!("Регистрация завершена");
            } else {
                println!("Факультет: {:?}", state.faculty());
                println!("Форма: {:?}", state.study_form());
                println!("Курс: {:?}", state.course());
            }
        }
        Ok(None) => {
            println!("Состояние не найдено");
        }
        Err(e) => {
            eprintln!("Ошибка: {}", e);
        }
    }
}
```

### Обновление факультета в состоянии

```rust
async fn set_faculty_example(db: &DbFacade, telegram_id: i64) {
    match db.set_user_faculty(telegram_id, "МИТ").await {
        Ok(state) => {
            println!("Факультет установлен: {:?}", state);
        }
        Err(e) => {
            eprintln!("Ошибка: {}", e);
        }
    }
}
```

### Обновление формы обучения в состоянии

```rust
async fn set_study_form_example(db: &DbFacade, telegram_id: i64) {
    match db.set_user_study_form(telegram_id, "Очная").await {
        Ok(state) => {
            println!("Форма установлена: {:?}", state);
        }
        Err(e) => {
            eprintln!("Ошибка: {}", e);
        }
    }
}
```

### Обновление курса в состоянии

```rust
async fn set_course_example(db: &DbFacade, telegram_id: i64) {
    match db.set_user_course(telegram_id, "2 курс").await {
        Ok(state) => {
            println!("Курс установлен: {:?}", state);
        }
        Err(e) => {
            eprintln!("Ошибка: {}", e);
        }
    }
}
```

## Работа с Telegram Bot API

### Отправка простого сообщения

```rust
use teloxide::prelude::*;

async fn send_message_example(bot: Bot, chat_id: ChatId) -> Result<(), teloxide::RequestError> {
    bot.send_message(chat_id, "Привет! 👋")
        .await?;
    Ok(())
}
```

### Отправка сообщения с клавиатурой

```rust
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

async fn send_keyboard_example(
    bot: Bot,
    chat_id: ChatId
) -> Result<(), teloxide::RequestError> {
    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback("Кнопка 1", "btn_1")],
        vec![InlineKeyboardButton::callback("Кнопка 2", "btn_2")],
    ]);

    bot.send_message(chat_id, "Выберите опцию:")
        .reply_markup(keyboard)
        .await?;
    Ok(())
}
```

### Обработка callback-запроса с ответом

```rust
async fn handle_callback_example(
    bot: Bot,
    q: CallbackQuery
) -> Result<(), teloxide::RequestError> {
    if let Some(callback_data) = q.data {
        println!("Callback data: {}", callback_data);
        
        // Показать уведомление пользователю
        bot.answer_callback_query(&q.id)
            .text("✅ Выбор принят!")
            .await?;

        // Отправить новое сообщение
        bot.send_message(q.from.id, "Спасибо за выбор!")
            .await?;
    }
    Ok(())
}
```

## Работа с Enum'ами

### Парсинг Faculty из строки

```rust
use crate::domain::faculty::Faculty;
use std::str::FromStr;

fn parse_faculty_example() {
    let faculty_str = "faculty_mit";
    match Faculty::from_str(faculty_str) {
        Ok(faculty) => {
            println!("Факультет: {}", faculty.title());
            println!("Callback: {}", faculty.callback());
        }
        Err(_) => {
            println!("Неизвестный факультет");
        }
    }
}
```

### Работа с Course

```rust
use crate::domain::course::Course;
use std::str::FromStr;

fn work_with_course_example() {
    let course = Course::Second;
    println!("Курс: {}", course.title());
    println!("Callback: {}", course.callback());
    
    // Парсинг из строки
    if let Ok(parsed) = Course::from_str("course_3") {
        println!("Распарсенный курс: {}", parsed.title());
    }
}
```

### Работа с MitGroup

```rust
use crate::domain::groups::mit::MitGroup;

fn work_with_group_example() {
    let group = MitGroup::ISIT;
    println!("Группа: {}", group.title());
    println!("Callback: {}", group.callback());
}
```

### Парсинг единого CallbackData

```rust
use crate::domain::callbacks::CallbackData;
use std::str::FromStr;

fn parse_callback_data_example() {
    let callbacks = vec!["register", "faculty_mit", "form_fulltime", "course_1", "mit_group_pi"];
    
    for callback_str in callbacks {
        match CallbackData::from_str(callback_str) {
            Ok(callback) => {
                match callback {
                    CallbackData::TechButton(btn) => println!("Кнопка: {:?}", btn),
                    CallbackData::Faculty(fac) => println!("Факультет: {}", fac.title()),
                    CallbackData::StudyForm(form) => println!("Форма: {}", form.title()),
                    CallbackData::Course(course) => println!("Курс: {}", course.title()),
                    CallbackData::MitGroup(group) => println!("Группа: {}", group.title()),
                }
            }
            Err(e) => println!("Ошибка парсинга: {}", e),
        }
    }
}
```

## Полный пример: Регистрация пользователя

```rust
use teloxide::prelude::*;
use crate::db::facade::DbFacade;
use std::sync::Arc;

async fn complete_registration_example(
    bot: Bot,
    user_id: UserId,
    db: Arc<DbFacade>,
    telegram_id: i64,
    faculty: &str,
    group: &str,
    study_form: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Создаём состояние пользователя
    db.create_user_state(telegram_id).await?;
    
    // 2. Обновляем состояние с выбранными данными
    db.set_user_faculty(telegram_id, faculty).await?;
    db.set_user_study_form(telegram_id, study_form).await?;
    
    // 3. Регистрируем студента
    let student = db.register_student(
        telegram_id,
        faculty,
        group,
        study_form
    ).await?;
    
    // 4. Отправляем подтверждение пользователю
    let message = format!(
        "🎉 Поздравляем! Вы зарегистрированы!\n\n\
        📚 Факультет: {}\n\
        👥 Группа: {}\n\
        📝 Форма обучения: {}",
        student.faculty,
        student.group_name,
        student.study_form
    );
    
    bot.send_message(user_id, message).await?;
    
    Ok(())
}
```

## Обработка ошибок

```rust
use anyhow::Result;
use log::error;

async fn error_handling_example(db: &DbFacade) -> Result<()> {
    match db.find_student(123456).await {
        Ok(Some(student)) => {
            println!("Найден: {}", student.group_name);
        }
        Ok(None) => {
            println!("Студент не найден");
        }
        Err(e) => {
            error!("Ошибка БД: {}", e);
            return Err(e);
        }
    }
    Ok(())
}

// С контекстом ошибки
async fn error_with_context(db: &DbFacade) -> Result<()> {
    db.find_student(123456)
        .await
        .map_err(|e| anyhow::anyhow!("Не удалось найти студента: {}", e))?;
    Ok(())
}
```

## Логирование

```rust
use log::{debug, info, warn, error};

async fn logging_example(telegram_id: i64) {
    debug!("Начинаем обработку запроса для пользователя {}", telegram_id);
    
    info!("Пользователь {} выбрал факультет МИТ", telegram_id);
    
    warn!("Данные пользователя {} неполные", telegram_id);
    
    error!("Ошибка при регистрации пользователя {}: DB connection failed", telegram_id);
}
```

---

Эти примеры демонстрируют основные паттерны использования API приложения. Для более сложных сценариев смотрите исходный код в `src/services/telegram/`.

