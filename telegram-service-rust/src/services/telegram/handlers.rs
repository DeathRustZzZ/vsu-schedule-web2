use teloxide::prelude::*;
use crate::db::facade::DbFacade;
use crate::services::telegram::callback_router;
use crate::services::telegram::reply_menu::{self, TopLevelCommand};
use crate::services::telegram::registration::keyboards::{
    faculty_keyboard, mit_group_keyboard,
};
use crate::services::telegram::ui::render_ui;
use std::sync::Arc;
use std::str::FromStr;
use log::{info, debug, error};

/// Обработчик сообщений от пользователей
pub async fn handle_message(
    bot: Bot,
    msg: Message,
    db: Arc<DbFacade>,
) -> Result<(), teloxide::RequestError> {
    let telegram_id = match msg.from {
        Some(ref user) => user.id.0 as i64,
        None => {
            debug!("handle_message: сообщение без отправителя, пропускаем");
            return Ok(());
        }
    };
    debug!("handle_message: получен запрос от пользователя {}", telegram_id);

    // Проверяем, зарегистрирован ли пользователь
    match db.find_student(telegram_id).await {
        Ok(Some(student)) => {
            info!("Пользователь {} найден: {}", telegram_id, student.group_name);

            if let Some(text) = msg.text() {
                if let Ok(cmd) = TopLevelCommand::from_str(text) {
                    handle_top_level_command(
                        &bot,
                        &msg,
                        db.as_ref(),
                        true,
                        cmd,
                    )
                    .await?;
                    return Ok(());
                }
            }

            // Приветствие с меню зарегистрированного пользователя
            render_ui(
                &bot,
                db.as_ref(),
                telegram_id,
                msg.chat.id,
                None,
                &format!("👋 Привет, {}!\n\nИспользуй меню ниже для навигации.", student.group_name),
                None,
                Some(reply_menu::keyboard(true)),
            )
            .await?;
        }
        Ok(None) => {
            debug!("Пользователь {} не найден, предлагаем регистрацию", telegram_id);

            if let Some(text) = msg.text() {
                if let Ok(cmd) = TopLevelCommand::from_str(text) {
                    handle_top_level_command(
                        &bot,
                        &msg,
                        db.as_ref(),
                        false,
                        cmd,
                    )
                    .await?;
                    return Ok(());
                }
            }

            // Приветствие с предложением регистрации
            render_ui(
                &bot,
                db.as_ref(),
                telegram_id,
                msg.chat.id,
                None,
                "👋 Привет! Ты ещё не зарегистрирован.\n\nИспользуй меню ниже, чтобы начать регистрацию.",
                None,
                Some(reply_menu::keyboard(false)),
            )
            .await?;
        }
        Err(err) => {
            error!("Ошибка поиска студента {}: {:?}", telegram_id, err);
            render_ui(
                &bot,
                db.as_ref(),
                telegram_id,
                msg.chat.id,
                None,
                "❌ Ошибка при обращении к базе. Попробуйте позже.",
                None,
                Some(reply_menu::keyboard(false)),
            )
            .await?;
        }
    }
    Ok(())
}

/// Обработчик callback-запросов (нажатия кнопок)
/// Делегирует обработку callback_router'у для централизованной маршрутизации
pub async fn handle_callback(
    bot: Bot,
    q: CallbackQuery,
    db: Arc<DbFacade>,
) -> Result<(), teloxide::RequestError> {
    callback_router::route_callback(bot, q, db).await
}

async fn handle_top_level_command(
    bot: &Bot,
    msg: &Message,
    db: &DbFacade,
    is_registered: bool,
    cmd: TopLevelCommand,
) -> Result<(), teloxide::RequestError> {
    match cmd {
        TopLevelCommand::MainMenu => {
            render_ui(
                bot,
                db,
                msg.from.as_ref().map(|u| u.id.0 as i64).unwrap_or_default(),
                msg.chat.id,
                None,
                "🏠 Главное меню\n\nВыбери действие в меню снизу.",
                None,
                Some(reply_menu::keyboard(is_registered)),
            )
            .await?;
        }
        TopLevelCommand::Register => {
            render_ui(
                bot,
                db,
                msg.from.as_ref().map(|u| u.id.0 as i64).unwrap_or_default(),
                msg.chat.id,
                None,
                "✨ Начинаем регистрацию!\nВыбери факультет 📚",
                Some(faculty_keyboard()),
                Some(reply_menu::keyboard(false)),
            )
            .await?;
        }
        TopLevelCommand::MySchedule => {
            render_ui(
                bot,
                db,
                msg.from.as_ref().map(|u| u.id.0 as i64).unwrap_or_default(),
                msg.chat.id,
                None,
                "📅 Вот твоё расписание...\n\n(функция в разработке)",
                None,
                Some(reply_menu::keyboard(is_registered)),
            )
            .await?;
        }
        TopLevelCommand::MyProfile => {
            render_ui(
                bot,
                db,
                msg.from.as_ref().map(|u| u.id.0 as i64).unwrap_or_default(),
                msg.chat.id,
                None,
                "👤 Мой профиль\n\n(здесь будут данные профиля)",
                None,
                Some(reply_menu::keyboard(is_registered)),
            )
            .await?;
        }
        TopLevelCommand::ChooseGroup => {
            render_ui(
                bot,
                db,
                msg.from.as_ref().map(|u| u.id.0 as i64).unwrap_or_default(),
                msg.chat.id,
                None,
                "🔎 Выбери группу",
                Some(mit_group_keyboard()),
                Some(reply_menu::keyboard(is_registered)),
            )
            .await?;
        }
        TopLevelCommand::Help => {
            render_ui(
                bot,
                db,
                msg.from.as_ref().map(|u| u.id.0 as i64).unwrap_or_default(),
                msg.chat.id,
                None,
                "❓ Справка\n\nИспользуй меню снизу для навигации по функциям бота.",
                None,
                Some(reply_menu::keyboard(is_registered)),
            )
            .await?;
        }
    }
    Ok(())
}
