use log::{debug, info, warn};
use teloxide::prelude::*;
use teloxide::types::MessageId;

use crate::bot::keyboards;
use crate::bot::ui::render_screen;
use crate::db::facade::DbFacade;

/// Главное меню с ветвлением по состоянию регистрации.
pub async fn show_main_menu(
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
pub async fn show_help(
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

/// Профиль пользователя.
pub async fn show_profile(
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
                "👤 Профиль\n\nФакультет: {}\nФорма: {}\nКурс: {}\nГруппа: {}\nПодгруппа: {}",
                student.faculty,
                student.study_form,
                student.course.as_deref().unwrap_or("—"),
                student.group_name,
                student.subgroup_name.as_deref().unwrap_or("—"),
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
