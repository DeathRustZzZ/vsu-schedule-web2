use std::str::FromStr;
use std::sync::Arc;

use chrono::{Datelike, FixedOffset, Utc};
use log::{debug, info, warn};
use teloxide::prelude::*;
use teloxide::types::MessageId;

use crate::bot::callbacks::{Action, Callback};
use crate::bot::keyboards;
use crate::bot::schedule_api::{GroupWithSubgroupsIds, ScheduleApi};
use crate::bot::ui::render_screen;
use crate::db::facade::DbFacade;
use crate::domain::registration_state::RegistrationState;

/// Запуск Telegram-бота + подключение внешнего Schedule API.
///
/// Архитектурная идея:
/// - `DbFacade` и `ScheduleApi` заворачиваем в `Arc`, потому что обработчики dptree будут клонироваться.
/// - сам `Bot` остаётся "лёгким" хэндлом, teloxide его копирует без проблем.
///
/// Диагностически важно логировать:
/// - старт бота
/// - факт инициализации внешнего API (base_url логируется внутри ScheduleApi::new)
pub async fn run(
    bot: Bot,
    db: DbFacade,
    schedule_api_base: String,
    schedule_tz_offset_seconds: i32,
) {
    info!("Запуск Telegram-бота...");

    let db = Arc::new(db);
    let schedule_api = Arc::new(ScheduleApi::new(schedule_api_base));

    let handler = dptree::entry()
        // Обработка обычных сообщений
        .branch(Update::filter_message().endpoint({
            let db = Arc::clone(&db);
            let schedule_api = Arc::clone(&schedule_api);
            move |bot: Bot, msg: Message| {
                let db = Arc::clone(&db);
                let schedule_api = Arc::clone(&schedule_api);
                async move {
                    // Здесь все правильно - handle_message_safe для Message
                    crate::bot::error_handler::handle_message_safe(
                        bot.clone(),
                        msg.clone(),
                        |b, m| async move {
                            handle_message(b, m, db, schedule_api).await
                        },
                    )
                        .await
                }
            }
        }))
        // Обработка callback (кнопок)
        .branch(Update::filter_callback_query().endpoint({
            let db = Arc::clone(&db);
            let schedule_api = Arc::clone(&schedule_api);
            let schedule_tz_offset_seconds = schedule_tz_offset_seconds;
            move |bot: Bot, q: CallbackQuery| {
                let db = Arc::clone(&db);
                let schedule_api = Arc::clone(&schedule_api);
                async move {
                    // ИСПРАВЛЯЕМ: используем handle_callback_safe для CallbackQuery
                    crate::bot::error_handler::handle_callback_safe(
                        bot.clone(),
                        q.clone(),
                        |b, query| async move {
                            handle_callback(
                                b,
                                query,
                                db,
                                schedule_api,
                                schedule_tz_offset_seconds,
                            )
                                .await
                        },
                    )
                        .await
                }
            }
        }));

    info!("Dispatcher initialized. Waiting for updates…");
    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    warn!("Dispatcher stopped. Bot runtime finished.");
}

/// Обработка обычных сообщений.
/// Сейчас schedule_api не используется (параметр оставлен для будущей функциональности).
///
/// Важно логировать:
/// - кто пишет
/// - что за команда
/// - какой fallback сработал
async fn handle_message(
    bot: Bot,
    msg: Message,
    db: Arc<DbFacade>,
    _schedule_api: Arc<ScheduleApi>,
) -> Result<(), teloxide::RequestError> {
    let telegram_id = match msg.from {
        Some(ref user) => user.id.0 as i64,
        None => {
            debug!("message without sender (msg_id={:?}) ignored", msg.id);
            return Ok(());
        }
    };

    debug!(
        "message received: user={} chat_id={} msg_id={:?}",
        telegram_id, msg.chat.id.0, msg.id
    );

    if let Some(text) = msg.text() {
        let cmd = text.trim();
        debug!("text message from user {}: '{}'", telegram_id, cmd);

        match cmd {
            "/start" | "/menu" => {
                info!("command /menu triggered by user {}", telegram_id);
                show_main_menu(&bot, db.as_ref(), telegram_id, msg.chat.id, None).await?;
                return Ok(());
            }
            "/register" => {
                info!("command /register triggered by user {}", telegram_id);
                start_registration(&bot, db.as_ref(), telegram_id, msg.chat.id, None).await?;
                return Ok(());
            }
            "/help" => {
                info!("command /help triggered by user {}", telegram_id);
                show_help(&bot, db.as_ref(), telegram_id, msg.chat.id, None).await?;
                return Ok(());
            }
            _ => {
                // Это не ошибка — пользователь мог написать что угодно.
                // Мы просто возвращаем в меню, чтобы UX был предсказуемым.
                debug!("unknown command/text '{}' from user {}", cmd, telegram_id);
            }
        }
    }

    // Fallback: любое нераспознанное сообщение возвращает главное меню.
    show_main_menu(&bot, db.as_ref(), telegram_id, msg.chat.id, None).await?;
    Ok(())
}

/// Обработка callback_query (нажатия inline-кнопок).
///
/// Это одна из самых критичных точек UX:
/// - callback_data должен распознаться
/// - всегда нужно `answer_callback_query`, чтобы убрать "часики"
/// - регистрационные шаги должны быть устойчивы к "старым кнопкам"
async fn handle_callback(
    bot: Bot,
    q: CallbackQuery,
    db: Arc<DbFacade>,
    schedule_api: Arc<ScheduleApi>,
    schedule_tz_offset_seconds: i32,
) -> Result<(), teloxide::RequestError> {
    let data = match q.data.clone() {
        Some(data) => data,
        None => {
            debug!("callback query without data (id={}) ignored", q.id);
            return Ok(());
        }
    };

    debug!("callback received: '{}' (q_id={})", data, q.id);

    let callback = match Callback::from_str(&data) {
        Ok(cb) => cb,
        Err(err) => {
            // warn оправдан: это ломает UX и указывает на устаревшие кнопки или баг генерации.
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

    debug!(
        "callback context: user={} chat_id={} message_id={:?}",
        telegram_id, chat_id.0, message_id.map(|m| m.0)
    );

    let user_state = match db.get_user_state(telegram_id).await {
        Ok(state) => state,
        Err(err) => {
            warn!(
                "failed to load user state for user {}: {:?}",
                telegram_id, err
            );
            render_screen(
                &bot,
                db.as_ref(),
                telegram_id,
                chat_id,
                message_id,
                "❌ Не удалось получить состояние регистрации. Попробуй ещё раз.",
                Some(keyboards::back_menu()),
            )
            .await?;
            bot.answer_callback_query(q.id).await?;
            return Ok(());
        }
    };

    match callback {
        Callback::Action(action) => {
            info!("action callback {:?} from user {}", action, telegram_id);
            handle_action(
                &bot,
                db.as_ref(),
                schedule_api.as_ref(),
                schedule_tz_offset_seconds,
                telegram_id,
                chat_id,
                message_id,
                action,
            )
                .await?;
        }

        Callback::Faculty(faculty) => {
            if !is_expected_state(&user_state, RegistrationState::AwaitingFaculty) {
                render_screen(
                    &bot,
                    db.as_ref(),
                    telegram_id,
                    chat_id,
                    message_id,
                    "⚠️ Этот шаг регистрации устарел. Начни регистрацию заново.",
                    Some(keyboards::main_menu(false)),
                )
                .await?;
                bot.answer_callback_query(q.id).await?;
                return Ok(());
            }
            info!("faculty selected: '{}' by user {}", faculty.title(), telegram_id);
            debug!("persisting faculty selection into DB (user={})", telegram_id);

            if db
                .set_user_faculty(
                telegram_id,
                faculty.title(),
                RegistrationState::AwaitingStudyForm,
            )
                .await
                .is_err()
            {
                render_screen(
                    &bot,
                    db.as_ref(),
                    telegram_id,
                    chat_id,
                    message_id,
                    "❌ Не удалось сохранить факультет. Попробуй ещё раз.",
                    Some(keyboards::main_menu(false)),
                )
                .await?;
                bot.answer_callback_query(q.id).await?;
                return Ok(());
            }

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
            if !is_expected_state(&user_state, RegistrationState::AwaitingStudyForm) {
                render_screen(
                    &bot,
                    db.as_ref(),
                    telegram_id,
                    chat_id,
                    message_id,
                    "⚠️ Этот шаг регистрации устарел. Начни регистрацию заново.",
                    Some(keyboards::main_menu(false)),
                )
                .await?;
                bot.answer_callback_query(q.id).await?;
                return Ok(());
            }
            info!("study form selected: '{}' by user {}", form.title(), telegram_id);
            debug!("persisting study form selection into DB (user={})", telegram_id);

            if db
                .set_user_study_form(
                telegram_id,
                form.title(),
                RegistrationState::AwaitingCourse,
            )
                .await
                .is_err()
            {
                render_screen(
                    &bot,
                    db.as_ref(),
                    telegram_id,
                    chat_id,
                    message_id,
                    "❌ Не удалось сохранить форму обучения. Попробуй ещё раз.",
                    Some(keyboards::main_menu(false)),
                )
                .await?;
                bot.answer_callback_query(q.id).await?;
                return Ok(());
            }

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
            if !is_expected_state(&user_state, RegistrationState::AwaitingCourse) {
                render_screen(
                    &bot,
                    db.as_ref(),
                    telegram_id,
                    chat_id,
                    message_id,
                    "⚠️ Этот шаг регистрации устарел. Начни регистрацию заново.",
                    Some(keyboards::main_menu(false)),
                )
                .await?;
                bot.answer_callback_query(q.id).await?;
                return Ok(());
            }
            info!("course selected: '{}' by user {}", course.title(), telegram_id);
            debug!("persisting course selection into DB (user={})", telegram_id);

            if db
                .set_user_course(telegram_id, course.title(), RegistrationState::AwaitingGroup)
                .await
                .is_err()
            {
                render_screen(
                    &bot,
                    db.as_ref(),
                    telegram_id,
                    chat_id,
                    message_id,
                    "❌ Не удалось сохранить курс. Попробуй ещё раз.",
                    Some(keyboards::main_menu(false)),
                )
                .await?;
                bot.answer_callback_query(q.id).await?;
                return Ok(());
            }

            let raw_faculty = user_state
                .as_ref()
                .and_then(|s| s.faculty())
                .unwrap_or("ФМиИТ");
            let faculty = normalize_faculty(raw_faculty).unwrap_or(raw_faculty);

            let available = match schedule_api.get_available_groups(faculty).await {
                Ok(list) => list.list,
                Err(err) => {
                    warn!("Failed to load groups list: {:?}", err);
                    render_screen(
                        &bot,
                        db.as_ref(),
                        telegram_id,
                        chat_id,
                        message_id,
                        "❌ Не удалось получить список групп.",
                        Some(keyboards::main_menu(false)),
                    )
                    .await?;
                    bot.answer_callback_query(q.id).await?;
                    return Ok(());
                }
            };

            let filtered = filter_groups_by_course(&available, course);
            if filtered.is_empty() {
                render_screen(
                    &bot,
                    db.as_ref(),
                    telegram_id,
                    chat_id,
                    message_id,
                    "❌ Не удалось подобрать группы для выбранного курса.",
                    Some(keyboards::main_menu(false)),
                )
                .await?;
                bot.answer_callback_query(q.id).await?;
                return Ok(());
            }

            render_screen(
                &bot,
                db.as_ref(),
                telegram_id,
                chat_id,
                message_id,
                &format!("✅ Курс: {}\n\nТеперь выбери группу:", course.title()),
                Some(keyboards::groups_keyboard(&filtered)),
            )
            .await?;
        }

        Callback::MitGroup(group) => {
            if !is_expected_state(&user_state, RegistrationState::AwaitingGroup) {
                render_screen(
                    &bot,
                    db.as_ref(),
                    telegram_id,
                    chat_id,
                    message_id,
                    "⚠️ Этот шаг регистрации устарел. Начни регистрацию заново.",
                    Some(keyboards::main_menu(false)),
                )
                .await?;
                bot.answer_callback_query(q.id).await?;
                return Ok(());
            }
            info!("group selected: '{}' by user {}", group.title(), telegram_id);

            let username = q.from.username.clone();
            complete_registration(
                &bot,
                db.as_ref(),
                telegram_id,
                chat_id,
                message_id,
                group.title().to_string(),
                username,
            )
            .await?;
        }

        Callback::GroupId(group_id) => {
            if !is_expected_state(&user_state, RegistrationState::AwaitingGroup) {
                render_screen(
                    &bot,
                    db.as_ref(),
                    telegram_id,
                    chat_id,
                    message_id,
                    "⚠️ Этот шаг регистрации устарел. Начни регистрацию заново.",
                    Some(keyboards::main_menu(false)),
                )
                .await?;
                bot.answer_callback_query(q.id).await?;
                return Ok(());
            }

            let raw_faculty = user_state
                .as_ref()
                .and_then(|s| s.faculty())
                .unwrap_or("ФМиИТ");
            let faculty = normalize_faculty(raw_faculty).unwrap_or(raw_faculty);

            let available = match schedule_api.get_available_groups(faculty).await {
                Ok(list) => list.list,
                Err(err) => {
                    warn!("Failed to load groups list: {:?}", err);
                    render_screen(
                        &bot,
                        db.as_ref(),
                        telegram_id,
                        chat_id,
                        message_id,
                        "❌ Не удалось получить список групп.",
                        Some(keyboards::main_menu(false)),
                    )
                    .await?;
                    bot.answer_callback_query(q.id).await?;
                    return Ok(());
                }
            };

            let selected = available.iter().find(|g| g.group_id == group_id);
            if let Some(selected) = selected {
                if selected.subgroup_ids.len() > 1 {
                    render_screen(
                        &bot,
                        db.as_ref(),
                        telegram_id,
                        chat_id,
                        message_id,
                        &format!("✅ Группа: {}\n\nТеперь выбери подгруппу:", selected.group_id),
                        Some(keyboards::subgroups_keyboard(&selected.subgroup_ids)),
                    )
                    .await?;
                } else if selected.subgroup_ids.len() == 1 {
                    let username = q.from.username.clone();
                    complete_registration(
                        &bot,
                        db.as_ref(),
                        telegram_id,
                        chat_id,
                        message_id,
                        selected.subgroup_ids[0].clone(),
                        username,
                    )
                    .await?;
                } else {
                    let username = q.from.username.clone();
                    complete_registration(
                        &bot,
                        db.as_ref(),
                        telegram_id,
                        chat_id,
                        message_id,
                        selected.group_id.clone(),
                        username,
                    )
                    .await?;
                }
            } else {
                render_screen(
                    &bot,
                    db.as_ref(),
                    telegram_id,
                    chat_id,
                    message_id,
                    "❌ Не удалось подобрать группу. Попробуй ещё раз.",
                    Some(keyboards::main_menu(false)),
                )
                .await?;
            }
        }

        Callback::SubgroupId(subgroup_id) => {
            if !is_expected_state(&user_state, RegistrationState::AwaitingGroup) {
                render_screen(
                    &bot,
                    db.as_ref(),
                    telegram_id,
                    chat_id,
                    message_id,
                    "⚠️ Этот шаг регистрации устарел. Начни регистрацию заново.",
                    Some(keyboards::main_menu(false)),
                )
                .await?;
                bot.answer_callback_query(q.id).await?;
                return Ok(());
            }
            let username = q.from.username.clone();
            complete_registration(
                &bot,
                db.as_ref(),
                telegram_id,
                chat_id,
                message_id,
                subgroup_id,
                username,
            )
            .await?;
        }
    }

    // Всегда "закрываем" callback, чтобы у пользователя не висели "часики".
    bot.answer_callback_query(q.id).await?;
    Ok(())
}

/// Обработка Action (навигация/меню).
///
/// Здесь концентрируется UI-навигация, а доменные данные выбора (Faculty/Group/…) живут отдельно.
/// Это снижает вероятность смешения "действий" и "данных".
async fn handle_action(
    bot: &Bot,
    db: &DbFacade,
    schedule_api: &ScheduleApi,
    schedule_tz_offset_seconds: i32,
    telegram_id: i64,
    chat_id: ChatId,
    message_id: Option<MessageId>,
    action: Action,
) -> Result<(), teloxide::RequestError> {
    debug!(
        "handle_action: user={} action={:?} chat_id={} message_id={:?}",
        telegram_id, action, chat_id.0, message_id.map(|m| m.0)
    );

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
            // Schedule — внешняя интеграция → самое ценное место для логов.
            show_schedule(
                bot,
                db,
                schedule_api,
                schedule_tz_offset_seconds,
                telegram_id,
                chat_id,
                message_id,
            )
            .await?;
        }
        Action::ChooseGroup => {
            // Сейчас выбор группы встроен в регистрацию.
            // Лог полезен: иначе непонятно, почему кнопка "Выбрать группу" запускает регистрацию.
            info!(
                "Action::ChooseGroup invoked by user {} — routed to registration flow",
                telegram_id
            );
            start_registration(bot, db, telegram_id, chat_id, message_id).await?;
        }
        Action::Help => {
            show_help(bot, db, telegram_id, chat_id, message_id).await?;
        }
    }
    Ok(())
}

/// Главное меню с ветвлением по состоянию регистрации.
async fn show_main_menu(
    bot: &Bot,
    db: &DbFacade,
    telegram_id: i64,
    chat_id: ChatId,
    message_id: Option<MessageId>,
) -> Result<(), teloxide::RequestError> {
    debug!("show_main_menu: user={} chat_id={}", telegram_id, chat_id.0);

    let is_registered = db.find_student(telegram_id).await.ok().flatten().is_some();
    debug!("is_registered={} for user={}", is_registered, telegram_id);

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

/// Справка должна быть доступна всегда.
async fn show_help(
    bot: &Bot,
    db: &DbFacade,
    telegram_id: i64,
    chat_id: ChatId,
    message_id: Option<MessageId>,
) -> Result<(), teloxide::RequestError> {
    debug!("show_help: user={} chat_id={}", telegram_id, chat_id.0);

    let text =
        "ℹ️ Справка\n\nДоступные команды:\n/start — главное меню\n/register — регистрация\n/help — помощь";
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

/// Запускаем регистрацию: сбрасываем состояние и просим выбрать факультет.
async fn start_registration(
    bot: &Bot,
    db: &DbFacade,
    telegram_id: i64,
    chat_id: ChatId,
    message_id: Option<MessageId>,
) -> Result<(), teloxide::RequestError> {
    info!("start_registration: user={} chat_id={}", telegram_id, chat_id.0);

    if db
        .reset_registration(telegram_id, RegistrationState::AwaitingFaculty)
        .await
        .is_err()
    {
        render_screen(
            bot,
            db,
            telegram_id,
            chat_id,
            message_id,
            "❌ Не удалось начать регистрацию. Попробуй ещё раз.",
            Some(keyboards::main_menu(false)),
        )
        .await?;
        return Ok(());
    }

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

/// Финальный шаг регистрации.
/// Тут много edge-case’ов (устаревшие кнопки, неполное состояние), поэтому warn/debug оправданы.
async fn complete_registration(
    bot: &Bot,
    db: &DbFacade,
    telegram_id: i64,
    chat_id: ChatId,
    message_id: Option<MessageId>,
    group_name: String,
    username: Option<String>,
) -> Result<(), teloxide::RequestError> {
    info!(
        "complete_registration: user={} group='{}' chat_id={}",
        telegram_id,
        group_name,
        chat_id.0
    );

    let state = match db.get_user_state(telegram_id).await {
        Ok(state) => {
            debug!("user_state loaded: user={} state={:?}", telegram_id, state);
            state
        }
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
    debug!("student record exists={} for user={}", student.is_some(), telegram_id);

    if state.is_none() && student.is_none() {
        debug!("no state and no student record → restarting registration");
        start_registration(bot, db, telegram_id, chat_id, message_id).await?;
        return Ok(());
    }

    if let Some(ref current_state) = state {
        if current_state.faculty().is_none() || current_state.study_form().is_none() {
            warn!(
                "incomplete registration state for user={} state={:?} → restarting registration",
                telegram_id, current_state
            );
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

    debug!(
        "register_student payload: user={} faculty='{}' study_form='{}' course={:?} group='{}' username={:?}",
        telegram_id,
        faculty,
        study_form,
        course,
        group_name,
        username
    );

    match db
        .register_student(
            telegram_id,
            faculty,
            &group_name,
            study_form,
            course,
            username.as_deref(),
        )
        .await
    {
        Ok(student) => {
            info!("registration successful: user={} student_id={}", telegram_id, student.id);

            if db
                .reset_registration(telegram_id, RegistrationState::Idle)
                .await
                .is_err()
            {
                warn!(
                    "failed to reset registration state for user {} after success",
                    telegram_id
                );
            }

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

/// Профиль пользователя.
async fn show_profile(
    bot: &Bot,
    db: &DbFacade,
    telegram_id: i64,
    chat_id: ChatId,
    message_id: Option<MessageId>,
) -> Result<(), teloxide::RequestError> {
    debug!("show_profile: user={} chat_id={}", telegram_id, chat_id.0);

    let text = match db.find_student(telegram_id).await {
        Ok(Some(student)) => {
            debug!("profile loaded for user={}", telegram_id);
            format!(
                "👤 Профиль\n\nФакультет: {}\nФорма: {}\nКурс: {}\nГруппа: {}",
                student.faculty,
                student.study_form,
                student.course.as_deref().unwrap_or("—"),
                student.group_name,
            )
        }
        Ok(None) => {
            info!("profile requested but user {} is not registered", telegram_id);
            "Профиль не найден. Сначала зарегистрируйся.".to_string()
        }
        Err(err) => {
            warn!("failed to load profile for user {}: {:?}", telegram_id, err);
            "❌ Ошибка загрузки профиля.".to_string()
        }
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

/// Показ расписания через внешний Schedule API.
///
/// Здесь самые важные зоны для логов:
/// - входные параметры (факультет/группа/weekday)
/// - этапы: загрузили профиль → загрузили список групп → подобрали id → загрузили расписание
/// - ошибки API (они не фатальные, но должны быть видимыми)
async fn show_schedule(
    bot: &Bot,
    db: &DbFacade,
    schedule_api: &ScheduleApi,
    schedule_tz_offset_seconds: i32,
    telegram_id: i64,
    chat_id: ChatId,
    message_id: Option<MessageId>,
) -> Result<(), teloxide::RequestError> {
    info!("show_schedule requested: user={} chat_id={}", telegram_id, chat_id.0);

    let student = match db.find_student(telegram_id).await {
        Ok(Some(student)) => student,
        Ok(None) => {
            info!("user {} requested schedule but not registered", telegram_id);
            return render_screen(
                bot,
                db,
                telegram_id,
                chat_id,
                message_id,
                "Сначала зарегистрируйся, чтобы смотреть расписание.",
                Some(keyboards::back_menu()),
            )
                .await;
        }
        Err(err) => {
            warn!("failed to load student profile for user {}: {:?}", telegram_id, err);
            return render_screen(
                bot,
                db,
                telegram_id,
                chat_id,
                message_id,
                "❌ Не удалось загрузить профиль.",
                Some(keyboards::back_menu()),
            )
                .await;
        }
    };

    // Приводим факультет из профиля к ожидаемому формату внешнего API.
    // Это "тонкое место": разные источники могут писать "ФМиИТ" по-разному.
    let faculty = normalize_faculty(&student.faculty);
    debug!(
        "faculty normalization: raw='{}' normalized={:?}",
        student.faculty, faculty
    );

    // Пока поддерживаем только ФМиИТ — явно сообщаем пользователю и логируем.
    if faculty.as_deref() != Some("ФМиИТ") {
        info!(
            "schedule requested for unsupported faculty: raw='{}' user={}",
            student.faculty, telegram_id
        );
        return render_screen(
            bot,
            db,
            telegram_id,
            chat_id,
            message_id,
            "Пока поддерживается только факультет ФМиИТ.",
            Some(keyboards::back_menu()),
        )
            .await;
    }

    // Текущий день недели: используется как параметр API.
    // Логируем, чтобы понимать, что именно запрашивали у API.
    let weekday = current_weekday_ru(schedule_tz_offset_seconds);
    debug!(
        "schedule request context: user={} faculty={:?} group_name='{}' weekday='{}'",
        telegram_id,
        faculty,
        student.group_name,
        weekday
    );

    // 1) Получаем список доступных групп.
    // Это нужно, чтобы подобрать (group_id, subgroup_id) под student.group_name.
    let available = match schedule_api
        .get_available_groups(faculty.as_deref().unwrap_or(&student.faculty))
        .await
    {
        Ok(list) => {
            debug!("available groups loaded: count={}", list.list.len());
            list.list
        }
        Err(err) => {
            warn!("Failed to load groups list: {:?}", err);
            return render_screen(
                bot,
                db,
                telegram_id,
                chat_id,
                message_id,
                "❌ Не удалось получить список групп.",
                Some(keyboards::back_menu()),
            )
                .await;
        }
    };

    // 2) Подбираем group_id / subgroup_id из списка.
    let (group_id, subgroup_id) = match pick_group_and_subgroup(&available, &student.group_name) {
        Some(pair) => {
            debug!(
                "picked group/subgroup: group_id='{}' subgroup_id='{}' for user={}",
                pair.0, pair.1, telegram_id
            );
            pair
        }
        None => {
            warn!(
                "failed to pick group/subgroup: group_name='{}' available_count={}",
                student.group_name,
                available.len()
            );
            return render_screen(
                bot,
                db,
                telegram_id,
                chat_id,
                message_id,
                "❌ Не удалось подобрать группу. Обратись к администратору.",
                Some(keyboards::back_menu()),
            )
                .await;
        }
    };

    // 3) Загружаем расписание.
    let schedule = match schedule_api
        .get_schedule(
            faculty.as_deref().unwrap_or(&student.faculty),
            &group_id,
            &subgroup_id,
            weekday,
        )
        .await
    {
        Ok(schedule) => {
            debug!("schedule loaded: lessons_count={}", schedule.lessons.len());
            schedule
        }
        Err(err) => {
            warn!("Failed to load schedule: {:?}", err);
            return render_screen(
                bot,
                db,
                telegram_id,
                chat_id,
                message_id,
                "❌ Не удалось получить расписание.",
                Some(keyboards::back_menu()),
            )
                .await;
        }
    };

    // 4) Формируем сообщение пользователю.
    // На этом этапе часто возникают "визуальные" баги, поэтому полезно логировать
    // количество занятий и выбранные id.
    debug!(
        "formatting schedule message: group_id='{}' subgroup_id='{}' lessons_count={}",
        group_id,
        subgroup_id,
        schedule.lessons.len()
    );

    let text = format_schedule_message(&group_id, &subgroup_id, schedule.lessons, weekday);

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

/// Подбор пары (group_id, subgroup_id) на основании:
/// - списка доступных групп от внешнего API
/// - `group_name`, который хранится у студента
///
/// Важный момент:
/// подбор идёт по `contains` на lower-case строках → это эвристика.
/// Она удобна, но при похожих названиях групп может дать "не ту" группу.
/// Поэтому в show_schedule выше мы логируем результат выбора.
fn pick_group_and_subgroup(
    available: &[GroupWithSubgroupsIds],
    group_name: &str,
) -> Option<(String, String)> {
    if available.is_empty() {
        // warn не ставим — это может быть валидный ответ API (например, факультет без групп).
        return None;
    }

    let needle = normalize_group_key(group_name);

    let exact_matches: Vec<&GroupWithSubgroupsIds> = available
        .iter()
        .filter(|g| normalize_group_key(&g.group_id) == needle)
        .collect();
    if exact_matches.len() == 1 {
        let selected = exact_matches[0].clone();
        let subgroup = pick_subgroup(&selected, &needle)?;
        return Some((selected.group_id.clone(), subgroup));
    }
    if exact_matches.len() > 1 {
        return None;
    }

    let contains_matches: Vec<&GroupWithSubgroupsIds> = available
        .iter()
        .filter(|g| normalize_group_key(&g.group_id).contains(&needle))
        .collect();
    if contains_matches.len() == 1 {
        let selected = contains_matches[0].clone();
        let subgroup = pick_subgroup(&selected, &needle)?;
        return Some((selected.group_id.clone(), subgroup));
    }

    let mut subgroup_hit: Option<(String, String)> = None;
    for group in available {
        for subgroup in &group.subgroup_ids {
            if normalize_group_key(subgroup) == needle {
                if subgroup_hit.is_some() {
                    return None;
                }
                subgroup_hit = Some((group.group_id.clone(), subgroup.clone()));
            }
        }
    }
    if let Some(pair) = subgroup_hit {
        return Some(pair);
    }

    let mut subgroup_contains: Option<(String, String)> = None;
    for group in available {
        for subgroup in &group.subgroup_ids {
            if normalize_group_key(subgroup).contains(&needle) {
                if subgroup_contains.is_some() {
                    return None;
                }
                subgroup_contains = Some((group.group_id.clone(), subgroup.clone()));
            }
        }
    }
    if let Some(pair) = subgroup_contains {
        return Some(pair);
    }

    None
}

/// Форматирование расписания в текст Telegram.
///
/// Почему сортировка здесь:
/// - API может вернуть занятия неотсортированными
/// - пользователю нужен человекочитаемый порядок по времени
fn format_schedule_message(
    group_id: &str,
    subgroup_id: &str,
    mut lessons: Vec<crate::bot::schedule_api::LessonResponse>,
    weekday: &str,
) -> String {
    lessons.sort_by(|a, b| a.start_time.cmp(&b.start_time));

    let header = if subgroup_id.is_empty() {
        format!("📅 Расписание на {weekday}\nГруппа: {group_id}\n")
    } else {
        format!(
            "📅 Расписание на {weekday}\nГруппа: {group_id}\nПодгруппа: {subgroup_id}\n"
        )
    };

    if lessons.is_empty() {
        return format!("{header}\nПар нет 🎉");
    }

    let mut lines = Vec::new();
    for (index, lesson) in lessons.iter().enumerate() {
        // Сборка строки максимально "бережно":
        // - trim у названия
        // - type и auditorium добавляем только если они заданы и не пустые
        let mut line = format!(
            "{}. {}-{} — {}",
            index + 1,
            lesson.start_time,
            lesson.end_time,
            lesson.name.trim()
        );

        if let Some(lesson_type) = lesson.lesson_type.as_ref() {
            if !lesson_type.is_empty() {
                line.push_str(" (");
                line.push_str(lesson_type);
                line.push(')');
            }
        }

        if let Some(auditorium) = lesson.auditorium.as_ref() {
            if !auditorium.is_empty() {
                line.push_str(" — ");
                line.push_str(auditorium);
            }
        }

        lines.push(line);
    }

    format!("{header}\n{}", lines.join("\n"))
}

/// Текущий день недели на русском.
///
/// Это прикладная функция для UI/параметров API.
/// Если API ожидает другой формат (например, "MONDAY"), это место нужно будет менять.
fn current_weekday_ru(offset_seconds: i32) -> &'static str {
    let offset = FixedOffset::east_opt(offset_seconds)
        .unwrap_or_else(|| FixedOffset::east_opt(0).unwrap());
    match Utc::now().with_timezone(&offset).weekday() {
        chrono::Weekday::Mon => "Понедельник",
        chrono::Weekday::Tue => "Вторник",
        chrono::Weekday::Wed => "Среда",
        chrono::Weekday::Thu => "Четверг",
        chrono::Weekday::Fri => "Пятница",
        chrono::Weekday::Sat => "Суббота",
        chrono::Weekday::Sun => "Воскресенье",
    }
}

fn normalize_group_key(value: &str) -> String {
    value
        .trim()
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect()
}

fn filter_groups_by_course(
    available: &[GroupWithSubgroupsIds],
    course: crate::domain::course::Course,
) -> Vec<GroupWithSubgroupsIds> {
    let course_number = match course {
        crate::domain::course::Course::First => 1,
        crate::domain::course::Course::Second => 2,
        crate::domain::course::Course::Third => 3,
        crate::domain::course::Course::Fourth => 4,
    };

    let year = Utc::now().year() % 100;
    let expected = year - course_number;

    let mut filtered: Vec<GroupWithSubgroupsIds> = available
        .iter()
        .cloned()
        .filter(|g| group_year_prefix(&g.group_id) == Some(expected))
        .collect();

    if filtered.is_empty() {
        filtered = available.to_vec();
    }

    filtered.sort_by(|a, b| a.group_id.cmp(&b.group_id));
    filtered
}

fn group_year_prefix(group_id: &str) -> Option<i32> {
    let digits: String = group_id.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.len() >= 2 {
        digits[..2].parse::<i32>().ok()
    } else {
        None
    }
}

fn pick_subgroup(
    selected: &GroupWithSubgroupsIds,
    needle: &str,
) -> Option<String> {
    if selected.subgroup_ids.is_empty() {
        return Some(String::new());
    }
    if normalize_group_key(&selected.group_id) == *needle && selected.subgroup_ids.len() > 1 {
        return None;
    }
    let exact: Vec<&String> = selected
        .subgroup_ids
        .iter()
        .filter(|id| normalize_group_key(id) == *needle)
        .collect();
    if exact.len() == 1 {
        return Some(exact[0].clone());
    }
    if exact.len() > 1 {
        return None;
    }

    let contains: Vec<&String> = selected
        .subgroup_ids
        .iter()
        .filter(|id| normalize_group_key(id).contains(needle))
        .collect();
    if contains.len() == 1 {
        return Some(contains[0].clone());
    }

    if selected.subgroup_ids.len() == 1 {
        return Some(selected.subgroup_ids[0].clone());
    }

    None
}

fn is_expected_state(
    state: &Option<crate::domain::user_state::UserState>,
    expected: RegistrationState,
) -> bool {
    let state_value = state
        .as_ref()
        .map(|s| RegistrationState::from_str(&s.state))
        .unwrap_or(RegistrationState::Idle);
    state_value == expected
}

/// Нормализация факультета под контракт внешнего API.
///
/// Почему нужна:
/// - в БД/регистрации факультет может храниться как "Факультет математики и ...",
///   а API ждёт короткое "ФМиИТ".
///
/// Возвращаем Option:
/// - Some("ФМиИТ") если уверенно распознали
/// - None если факультет неизвестный → выше есть явная обработка
fn normalize_faculty(value: &str) -> Option<&'static str> {
    let normalized = value.to_lowercase();

    // "распознавание по подстрокам" — эвристика для пользовательских/разноформатных данных.
    if normalized.contains("математики") && normalized.contains("информационных") {
        return Some("ФМиИТ");
    }
    if normalized == "фмиит" {
        return Some("ФМиИТ");
    }

    None
}
