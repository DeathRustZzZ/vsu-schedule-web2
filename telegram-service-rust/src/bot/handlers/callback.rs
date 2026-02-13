use std::str::FromStr;
use std::sync::Arc;

use log::{debug, info, warn};
use teloxide::prelude::*;
use teloxide::types::MessageId;

use crate::bot::callbacks::{Action, Callback};
use crate::bot::flows::menu::{show_help, show_main_menu, show_profile};
use crate::bot::flows::registration::{
    RegistrationContext, complete_registration, start_registration,
};
use crate::bot::flows::schedule::{
    ScheduleFlowContext, filter_groups_by_course, normalize_faculty, show_date_picker,
    show_schedule_for_date, show_schedule_for_week, show_schedule_menu,
};
use crate::bot::keyboards;
use crate::bot::schedule_api::ScheduleApi;
use crate::bot::ui::render_screen;
use crate::db::facade::DbFacade;
use crate::domain::registration_state::RegistrationState;
use chrono::{Duration, NaiveDate};

struct ActionContext<'a> {
    bot: &'a Bot,
    db: &'a DbFacade,
    schedule_tz_offset_seconds: i32,
    telegram_id: i64,
    chat_id: ChatId,
    message_id: Option<MessageId>,
}

impl<'a> ActionContext<'a> {
    fn new(
        bot: &'a Bot,
        db: &'a DbFacade,
        schedule_tz_offset_seconds: i32,
        telegram_id: i64,
        chat_id: ChatId,
        message_id: Option<MessageId>,
    ) -> Self {
        Self {
            bot,
            db,
            schedule_tz_offset_seconds,
            telegram_id,
            chat_id,
            message_id,
        }
    }
}

/// Обработка callback_query (нажатия inline-кнопок).
///
/// Это одна из самых критичных точек UX:
/// - callback_data должен распознаться
/// - всегда нужно `answer_callback_query`, чтобы убрать "часики"
/// - регистрационные шаги должны быть устойчивы к "старым кнопкам"
pub async fn handle_callback(
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
        telegram_id,
        chat_id.0,
        message_id.map(|m| m.0)
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
            handle_action(
                ActionContext::new(
                    &bot,
                    db.as_ref(),
                    schedule_tz_offset_seconds,
                    telegram_id,
                    chat_id,
                    message_id,
                ),
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

            if let Err(err) = db
                .set_user_faculty(
                    telegram_id,
                    faculty.title(),
                    RegistrationState::AwaitingStudyForm,
                )
                .await
            {
                warn!(
                    "failed to update faculty for user {}: {:?}",
                    telegram_id, err
                );
                render_screen(
                    &bot,
                    db.as_ref(),
                    telegram_id,
                    chat_id,
                    message_id,
                    "❌ Не удалось выбрать факультет. Попробуй ещё раз.",
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
                "Выбери форму обучения:",
                Some(keyboards::study_form_keyboard()),
            )
            .await?;
        }
        Callback::StudyForm(study_form) => {
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

            if let Err(err) = db
                .set_user_study_form(
                    telegram_id,
                    study_form.title(),
                    RegistrationState::AwaitingCourse,
                )
                .await
            {
                warn!(
                    "failed to update study_form for user {}: {:?}",
                    telegram_id, err
                );
                render_screen(
                    &bot,
                    db.as_ref(),
                    telegram_id,
                    chat_id,
                    message_id,
                    "❌ Не удалось выбрать форму обучения. Попробуй ещё раз.",
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
                "Выбери курс:",
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

            if let Err(err) = db
                .set_user_course(
                    telegram_id,
                    course.title(),
                    RegistrationState::AwaitingGroup,
                )
                .await
            {
                warn!(
                    "failed to update course for user {}: {:?}",
                    telegram_id, err
                );
                render_screen(
                    &bot,
                    db.as_ref(),
                    telegram_id,
                    chat_id,
                    message_id,
                    "❌ Не удалось выбрать курс. Попробуй ещё раз.",
                    Some(keyboards::main_menu(false)),
                )
                .await?;
                bot.answer_callback_query(q.id).await?;
                return Ok(());
            }

            let faculty_raw = user_state
                .as_ref()
                .and_then(|s| s.faculty())
                .unwrap_or("ФМиИТ");
            let faculty = normalize_faculty(faculty_raw).unwrap_or(faculty_raw);

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
                        Some(keyboards::back_menu()),
                    )
                    .await?;
                    bot.answer_callback_query(q.id).await?;
                    return Ok(());
                }
            };

            let filtered = filter_groups_by_course(&available, course);

            render_screen(
                &bot,
                db.as_ref(),
                telegram_id,
                chat_id,
                message_id,
                "Выбери группу:",
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
            let username = q.from.username.clone();
            complete_registration(
                RegistrationContext::new(&bot, db.as_ref(), telegram_id, chat_id, message_id),
                group.title().to_string(),
                None,
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
            let faculty_raw = user_state
                .as_ref()
                .and_then(|s| s.faculty())
                .unwrap_or("ФМиИТ");
            let faculty = normalize_faculty(faculty_raw).unwrap_or(faculty_raw);

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
                        Some(keyboards::back_menu()),
                    )
                    .await?;
                    bot.answer_callback_query(q.id).await?;
                    return Ok(());
                }
            };

            let group = available.iter().find(|g| g.group_id == group_id);
            let subgroup_ids = match group {
                Some(g) => g.subgroup_ids.clone(),
                None => {
                    render_screen(
                        &bot,
                        db.as_ref(),
                        telegram_id,
                        chat_id,
                        message_id,
                        "❌ Не удалось найти группу. Попробуй выбрать заново.",
                        Some(keyboards::back_menu()),
                    )
                    .await?;
                    bot.answer_callback_query(q.id).await?;
                    return Ok(());
                }
            };

            if subgroup_ids.len() > 1 {
                render_screen(
                    &bot,
                    db.as_ref(),
                    telegram_id,
                    chat_id,
                    message_id,
                    "Выбери подгруппу:",
                    Some(keyboards::subgroups_keyboard(&group_id, &subgroup_ids)),
                )
                .await?;
            } else {
                let username = q.from.username.clone();
                let subgroup = subgroup_ids.first().cloned();
                complete_registration(
                    RegistrationContext::new(&bot, db.as_ref(), telegram_id, chat_id, message_id),
                    group_id,
                    subgroup,
                    username,
                )
                .await?;
            }
        }
        Callback::SubgroupChoice {
            group_id,
            subgroup_id,
        } => {
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
            if group_id.is_empty() {
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
                RegistrationContext::new(&bot, db.as_ref(), telegram_id, chat_id, message_id),
                group_id,
                Some(subgroup_id),
                username,
            )
            .await?;
        }
        Callback::ScheduleMenu => {
            show_schedule_menu(
                &bot,
                db.as_ref(),
                schedule_tz_offset_seconds,
                telegram_id,
                chat_id,
                message_id,
            )
            .await?;
        }
        Callback::ScheduleDate(value) => match NaiveDate::parse_from_str(&value, "%Y-%m-%d") {
            Ok(date) => {
                show_schedule_for_date(
                    ScheduleFlowContext::new(
                        &bot,
                        db.as_ref(),
                        schedule_api.as_ref(),
                        schedule_tz_offset_seconds,
                        telegram_id,
                        chat_id,
                        message_id,
                    ),
                    date,
                )
                .await?;
            }
            Err(_) => {
                render_screen(
                    &bot,
                    db.as_ref(),
                    telegram_id,
                    chat_id,
                    message_id,
                    "⚠️ Не удалось распознать дату. Попробуй выбрать ещё раз.",
                    Some(keyboards::schedule_back_menu()),
                )
                .await?;
            }
        },
        Callback::ScheduleWeek { start, end } => {
            let start_date = NaiveDate::parse_from_str(&start, "%Y-%m-%d").ok();
            let end_date = if end.trim().is_empty() {
                start_date.map(|date| date + Duration::days(6))
            } else {
                NaiveDate::parse_from_str(&end, "%Y-%m-%d").ok()
            };

            match (start_date, end_date) {
                (Some(start_date), Some(end_date)) if end_date >= start_date => {
                    show_schedule_for_week(
                        ScheduleFlowContext::new(
                            &bot,
                            db.as_ref(),
                            schedule_api.as_ref(),
                            schedule_tz_offset_seconds,
                            telegram_id,
                            chat_id,
                            message_id,
                        ),
                        start_date,
                        end_date,
                    )
                    .await?;
                }
                _ => {
                    render_screen(
                        &bot,
                        db.as_ref(),
                        telegram_id,
                        chat_id,
                        message_id,
                        "⚠️ Не удалось распознать диапазон дат. Попробуй выбрать ещё раз.",
                        Some(keyboards::schedule_back_menu()),
                    )
                    .await?;
                }
            }
        }
        Callback::SchedulePicker { start } => {
            let parsed = NaiveDate::parse_from_str(&start, "%Y-%m-%d").ok();
            show_date_picker(
                &bot,
                db.as_ref(),
                schedule_tz_offset_seconds,
                telegram_id,
                chat_id,
                message_id,
                parsed,
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
    ctx: ActionContext<'_>,
    action: Action,
) -> Result<(), teloxide::RequestError> {
    let ActionContext {
        bot,
        db,
        schedule_tz_offset_seconds,
        telegram_id,
        chat_id,
        message_id,
    } = ctx;

    debug!(
        "handle_action: user={} action={:?} chat_id={} message_id={:?}",
        telegram_id,
        action,
        chat_id.0,
        message_id.map(|m| m.0)
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
            show_schedule_menu(
                bot,
                db,
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

fn is_expected_state(
    state: &Option<crate::domain::user_state::UserState>,
    expected: RegistrationState,
) -> bool {
    let state_value = state
        .as_ref()
        .map(|s| RegistrationState::parse_or_idle(&s.state))
        .unwrap_or(RegistrationState::Idle);
    state_value == expected
}
