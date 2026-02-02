 use std::sync::Arc;
use std::str::FromStr;

use teloxide::prelude::*;
use teloxide::types::MessageId;
use log::{debug, info, warn};

use crate::bot::callbacks::{Action, Callback};
use crate::bot::keyboards;
use crate::bot::ui::render_screen;
use crate::db::facade::DbFacade;
use crate::domain::registration_state::RegistrationState;

pub async fn run(bot: Bot, db: DbFacade) {
    info!("Запуск Telegram-бота...");
    let db = Arc::new(db);

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint({
            let db = Arc::clone(&db);
            move |bot: Bot, msg: Message| {
                let db = Arc::clone(&db);
                async move { handle_message(bot, msg, db).await }
            }
        }))
        .branch(Update::filter_callback_query().endpoint({
            let db = Arc::clone(&db);
            move |bot: Bot, q: CallbackQuery| {
                let db = Arc::clone(&db);
                async move { handle_callback(bot, q, db).await }
            }
        }));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn handle_message(
    bot: Bot,
    msg: Message,
    db: Arc<DbFacade>,
) -> Result<(), teloxide::RequestError> {
    let telegram_id = match msg.from {
        Some(ref user) => user.id.0 as i64,
        None => return Ok(()),
    };

    if let Some(text) = msg.text() {
        let cmd = text.trim();
        match cmd {
            "/start" | "/menu" => {
                show_main_menu(&bot, db.as_ref(), telegram_id, msg.chat.id, None).await?;
                return Ok(());
            }
            "/register" => {
                start_registration(&bot, db.as_ref(), telegram_id, msg.chat.id, None).await?;
                return Ok(());
            }
            "/help" => {
                show_help(&bot, db.as_ref(), telegram_id, msg.chat.id, None).await?;
                return Ok(());
            }
            _ => {}
        }
    }

    show_main_menu(&bot, db.as_ref(), telegram_id, msg.chat.id, None).await?;
    Ok(())
}

async fn handle_callback(
    bot: Bot,
    q: CallbackQuery,
    db: Arc<DbFacade>,
) -> Result<(), teloxide::RequestError> {
    let data = match q.data.clone() {
        Some(data) => data,
        None => return Ok(()),
    };

    debug!("Callback: {}", data);
    let callback = match Callback::from_str(&data) {
        Ok(cb) => cb,
        Err(err) => {
            warn!("Неизвестный callback '{}': {}", data, err);
            bot.answer_callback_query(q.id).await?;
            return Ok(());
        }
    };

    let telegram_id = q.from.id.0 as i64;
    let chat_id = q
        .message
        .as_ref()
        .map(|m| m.chat().id)
        .unwrap_or(q.from.id.into());
    let message_id = q.message.as_ref().map(|m| m.id());

    match callback {
        Callback::Action(action) => {
            handle_action(&bot, db.as_ref(), telegram_id, chat_id, message_id, action).await?;
        }
        Callback::Faculty(faculty) => {
            db.set_user_faculty(telegram_id, faculty.title(), RegistrationState::AwaitingStudyForm)
                .await
                .ok();
            render_screen(
                &bot,
                db.as_ref(),
                telegram_id,
                chat_id,
                message_id,
                &format!("✅ Факультет: {}\n\nТеперь выбери форму обучения:", faculty.title()),
                Some(keyboards::study_form_keyboard()),
            )
            .await?;
        }
        Callback::StudyForm(form) => {
            db.set_user_study_form(telegram_id, form.title(), RegistrationState::AwaitingCourse)
                .await
                .ok();
            render_screen(
                &bot,
                db.as_ref(),
                telegram_id,
                chat_id,
                message_id,
                &format!("✅ Форма обучения: {}\n\nТеперь выбери курс:", form.title()),
                Some(keyboards::course_keyboard()),
            )
            .await?;
        }
        Callback::Course(course) => {
            db.set_user_course(telegram_id, course.title(), RegistrationState::AwaitingGroup)
                .await
                .ok();
            render_screen(
                &bot,
                db.as_ref(),
                telegram_id,
                chat_id,
                message_id,
                &format!("✅ Курс: {}\n\nТеперь выбери группу:", course.title()),
                Some(keyboards::mit_group_keyboard()),
            )
            .await?;
        }
        Callback::MitGroup(group) => {
            let username = q.from.username.clone();
            complete_registration(
                &bot,
                db.as_ref(),
                telegram_id,
                chat_id,
                message_id,
                group,
                username,
            )
            .await?;
        }
    }

    bot.answer_callback_query(q.id).await?;
    Ok(())
}

async fn handle_action(
    bot: &Bot,
    db: &DbFacade,
    telegram_id: i64,
    chat_id: ChatId,
    message_id: Option<MessageId>,
    action: Action,
) -> Result<(), teloxide::RequestError> {
    match action {
        Action::MainMenu | Action::Back => {
            show_main_menu(bot, db, telegram_id, chat_id, message_id).await?;
        }
        Action::Register => {
            start_registration(bot, db, telegram_id, chat_id, message_id).await?;
        }
        Action::MyProfile => {
            show_profile(bot, db, telegram_id, chat_id, message_id).await?;
        }
        Action::MySchedule => {
            show_schedule(bot, db, telegram_id, chat_id, message_id).await?;
        }
        Action::ChooseGroup => {
            start_registration(bot, db, telegram_id, chat_id, message_id).await?;
        }
        Action::Help => {
            show_help(bot, db, telegram_id, chat_id, message_id).await?;
        }
    }
    Ok(())
}

async fn show_main_menu(
    bot: &Bot,
    db: &DbFacade,
    telegram_id: i64,
    chat_id: ChatId,
    message_id: Option<MessageId>,
) -> Result<(), teloxide::RequestError> {
    let is_registered = db.find_student(telegram_id).await.ok().flatten().is_some();
    let text = if is_registered {
        "👋 Привет! Чем займёмся?"
    } else {
        "👋 Привет! Ты ещё не зарегистрирован."
    };
    render_screen(
        bot,
        db,
        telegram_id,
        chat_id,
        message_id,
        text,
        Some(keyboards::main_menu(is_registered)),
    )
    .await
}

async fn show_help(
    bot: &Bot,
    db: &DbFacade,
    telegram_id: i64,
    chat_id: ChatId,
    message_id: Option<MessageId>,
) -> Result<(), teloxide::RequestError> {
    let text = "ℹ️ Справка\n\nДоступные команды:\n/start — главное меню\n/register — регистрация\n/help — помощь";
    render_screen(
        bot,
        db,
        telegram_id,
        chat_id,
        message_id,
        text,
        Some(keyboards::back_menu()),
    )
    .await
}

async fn start_registration(
    bot: &Bot,
    db: &DbFacade,
    telegram_id: i64,
    chat_id: ChatId,
    message_id: Option<MessageId>,
) -> Result<(), teloxide::RequestError> {
    db.reset_registration(telegram_id, RegistrationState::AwaitingFaculty)
        .await
        .ok();
    render_screen(
        bot,
        db,
        telegram_id,
        chat_id,
        message_id,
        "✨ Начнём регистрацию. Выбери факультет:",
        Some(keyboards::faculty_keyboard()),
    )
    .await
}

// group selection is part of the registration flow now

async fn complete_registration(
    bot: &Bot,
    db: &DbFacade,
    telegram_id: i64,
    chat_id: ChatId,
    message_id: Option<MessageId>,
    group: crate::domain::groups::mit::MitGroup,
    username: Option<String>,
) -> Result<(), teloxide::RequestError> {
    let state = match db.get_user_state(telegram_id).await {
        Ok(state) => state,
        Err(err) => {
            warn!("Не удалось получить состояние пользователя {}: {:?}", telegram_id, err);
            render_screen(
                bot,
                db,
                telegram_id,
                chat_id,
                message_id,
                "❌ Не удалось завершить регистрацию. Попробуй ещё раз.",
                Some(keyboards::main_menu(false)),
            )
            .await?;
            return Ok(());
        }
    };

    let student = db.find_student(telegram_id).await.ok().flatten();

    if state.is_none() && student.is_none() {
        start_registration(bot, db, telegram_id, chat_id, message_id).await?;
        return Ok(());
    }

    if let Some(ref current_state) = state {
        if current_state.faculty().is_none() || current_state.study_form().is_none() {
            start_registration(bot, db, telegram_id, chat_id, message_id).await?;
            return Ok(());
        }
    }

    let faculty = state
        .as_ref()
        .and_then(|s| s.faculty())
        .or_else(|| student.as_ref().map(|s| s.faculty.as_str()))
        .unwrap_or("МИТ");
    let study_form = state
        .as_ref()
        .and_then(|s| s.study_form())
        .or_else(|| student.as_ref().map(|s| s.study_form.as_str()))
        .unwrap_or("Очная форма");
    let course = state
        .as_ref()
        .and_then(|s| s.course())
        .or_else(|| student.as_ref().and_then(|s| s.course.as_deref()));
    match db
        .register_student(telegram_id, faculty, group.title(), study_form, course, username.as_deref())
        .await
    {
        Ok(student) => {
            db.reset_registration(telegram_id, RegistrationState::Idle).await.ok();
            let text = format!(
                "🎉 Регистрация завершена!\n\nФакультет: {}\nФорма: {}\nКурс: {}\nГруппа: {}",
                student.faculty,
                student.study_form,
                student.course.as_deref().unwrap_or("—"),
                student.group_name,
            );
            render_screen(
                bot,
                db,
                telegram_id,
                chat_id,
                message_id,
                &text,
                Some(keyboards::main_menu(true)),
            )
            .await?;
        }
        Err(err) => {
            warn!("Ошибка регистрации пользователя {}: {:?}", telegram_id, err);
            render_screen(
                bot,
                db,
                telegram_id,
                chat_id,
                message_id,
                "❌ Не удалось завершить регистрацию. Попробуй ещё раз.",
                Some(keyboards::main_menu(false)),
            )
            .await?;
        }
    }

    Ok(())
}

async fn show_profile(
    bot: &Bot,
    db: &DbFacade,
    telegram_id: i64,
    chat_id: ChatId,
    message_id: Option<MessageId>,
) -> Result<(), teloxide::RequestError> {
    let text = match db.find_student(telegram_id).await {
        Ok(Some(student)) => format!(
            "👤 Профиль\n\nФакультет: {}\nФорма: {}\nКурс: {}\nГруппа: {}",
            student.faculty,
            student.study_form,
            student.course.as_deref().unwrap_or("—"),
            student.group_name,
        ),
        Ok(None) => "Профиль не найден. Сначала зарегистрируйся.".to_string(),
        Err(_) => "❌ Ошибка загрузки профиля.".to_string(),
    };

    render_screen(
        bot,
        db,
        telegram_id,
        chat_id,
        message_id,
        &text,
        Some(keyboards::back_menu()),
    )
    .await
}

async fn show_schedule(
    bot: &Bot,
    db: &DbFacade,
    telegram_id: i64,
    chat_id: ChatId,
    message_id: Option<MessageId>,
) -> Result<(), teloxide::RequestError> {
    let text = "📅 Расписание\n\nФункция в разработке.";
    render_screen(
        bot,
        db,
        telegram_id,
        chat_id,
        message_id,
        text,
        Some(keyboards::back_menu()),
    )
    .await
}
