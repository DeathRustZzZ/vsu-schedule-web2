use log::{debug, info, warn};
use teloxide::prelude::*;
use teloxide::types::MessageId;

use crate::bot::keyboards;
use crate::bot::ui::render_screen;
use crate::db::facade::{DbFacade, RegistrationParams};
use crate::domain::registration_state::RegistrationState;

pub struct RegistrationContext<'a> {
    pub bot: &'a Bot,
    pub db: &'a DbFacade,
    pub telegram_id: i64,
    pub chat_id: ChatId,
    pub message_id: Option<MessageId>,
}

impl<'a> RegistrationContext<'a> {
    pub fn new(
        bot: &'a Bot,
        db: &'a DbFacade,
        telegram_id: i64,
        chat_id: ChatId,
        message_id: Option<MessageId>,
    ) -> Self {
        Self {
            bot,
            db,
            telegram_id,
            chat_id,
            message_id,
        }
    }
}

/// Запускаем регистрацию: сбрасываем состояние и просим выбрать факультет.
pub async fn start_registration(
    bot: &Bot,
    db: &DbFacade,
    telegram_id: i64,
    chat_id: ChatId,
    message_id: Option<MessageId>,
) -> Result<(), teloxide::RequestError> {
    info!(
        "start_registration: user={} chat_id={}",
        telegram_id, chat_id.0
    );

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

/// Финальный шаг регистрации.
/// Тут много edge-case’ов (устаревшие кнопки, неполное состояние), поэтому warn/debug оправданы.
pub async fn complete_registration(
    ctx: RegistrationContext<'_>,
    group_name: String,
    subgroup_name: Option<String>,
    username: Option<String>,
) -> Result<(), teloxide::RequestError> {
    let RegistrationContext {
        bot,
        db,
        telegram_id,
        chat_id,
        message_id,
    } = ctx;

    info!(
        "complete_registration: user={} group='{}' chat_id={}",
        telegram_id, group_name, chat_id.0
    );

    let state = match db.get_user_state(telegram_id).await {
        Ok(state) => {
            debug!("user_state loaded: user={} state={:?}", telegram_id, state);
            state
        }
        Err(err) => {
            warn!(
                "Не удалось получить состояние пользователя {}: {:?}",
                telegram_id, err
            );
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
    debug!(
        "student record exists={} for user={}",
        student.is_some(),
        telegram_id
    );

    if state.is_none() && student.is_none() {
        debug!("no state and no student record → restarting registration");
        start_registration(bot, db, telegram_id, chat_id, message_id).await?;
        return Ok(());
    }

    if let Some(ref current_state) = state
        && (current_state.faculty().is_none() || current_state.study_form().is_none())
    {
        warn!(
            "incomplete registration state for user={} state={:?} → restarting registration",
            telegram_id, current_state
        );
        start_registration(bot, db, telegram_id, chat_id, message_id).await?;
        return Ok(());
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
        telegram_id, faculty, study_form, course, group_name, username
    );

    match db
        .register_student(
            telegram_id,
            RegistrationParams {
                faculty,
                group_name: &group_name,
                subgroup_name: subgroup_name.as_deref(),
                study_form,
                course,
                username: username.as_deref(),
            },
        )
        .await
    {
        Ok(student) => {
            info!(
                "registration successful: user={} student_id={}",
                telegram_id, student.id
            );

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
                "🎉 Регистрация завершена!\n\nФакультет: {}\nФорма: {}\nКурс: {}\nГруппа: {}\nПодгруппа: {}",
                student.faculty,
                student.study_form,
                student.course.as_deref().unwrap_or("—"),
                student.group_name,
                student.subgroup_name.as_deref().unwrap_or("—"),
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
