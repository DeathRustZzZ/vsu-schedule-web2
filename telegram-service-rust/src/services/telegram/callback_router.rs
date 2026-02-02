// src/services/telegram/callback_router.rs
//! Центральный маршрутизатор для обработки callback-запросов
//! Заменяет множество if let блоков единым dispatcher'ом

use teloxide::prelude::*;
use log::{info, warn};
use std::sync::Arc;
use std::str::FromStr;

use crate::db::facade::DbFacade;
use crate::domain::callbacks::CallbackData;
use crate::domain::menu::MenuCommand;
use crate::services::telegram::registration;
use crate::services::telegram::menu_dispatcher::{MenuDispatcher, MenuCommandContext};

/// Основной маршрутизатор callback-запросов
pub async fn route_callback(
    bot: Bot,
    q: CallbackQuery,
    db: Arc<DbFacade>,
) -> Result<(), teloxide::RequestError> {
    if let Some(data) = q.data.clone() {
        let callback_query_id = q.id.clone();
        info!("Обработка callback: '{}' от {}", data, q.from.id);

        // Сначала пытаемся распарсить как MenuCommand
        match MenuCommand::from_str(&data) {
            Ok(menu_cmd) => {
                let context = MenuCommandContext {
                    bot: bot.clone(),
                    query: q,
                };
                if let Err(e) = MenuDispatcher::dispatch(&context, menu_cmd).await {
                    warn!("Ошибка при обработке меню команды '{}': {:?}", data, e);
                }
                return Ok(());
            }
            Err(_) => {
                // Не меню команда, проверяем как CallbackData
            }
        }

        // Пытаемся распарсить как CallbackData
        match CallbackData::from_str(&data) {
            Ok(callback) => {
                if let Err(e) = handle_callback_data(&bot, q, db, callback).await {
                    warn!("Ошибка при обработке callback '{}': {:?}", data, e);
                }
                let _ = bot.answer_callback_query(callback_query_id).await;
            }
            Err(e) => {
                warn!("Ошибка парсинга callback '{}': {}", data, e);
                // Отправить сообщение об ошибке пользователю
                let _ = bot
                    .send_message(
                        q.from.id,
                        "❌ Неизвестная команда. Пожалуйста, используйте меню.",
                    )
                    .await;
                let _ = bot.answer_callback_query(callback_query_id).await;
            }
        }
    }
    Ok(())
}

/// Обработчик распарсённых callback-данных
async fn handle_callback_data(
    bot: &Bot,
    q: CallbackQuery,
    db: Arc<DbFacade>,
    callback: CallbackData,
) -> Result<(), Box<dyn std::error::Error>> {
    use crate::domain::callbacks::CallbackData::*;

    match callback {
        // Основные команды
        TechButton(btn) => {
            registration::handlers::handle_tech_button(bot.clone(), q, db, btn).await?;
        }

        // Регистрация
        Faculty(faculty) => {
            registration::handlers::handle_faculty_choice(bot.clone(), q, db, faculty).await?;
        }
        StudyForm(form) => {
            registration::handlers::handle_study_form(bot.clone(), q, db, form).await?;
        }
        Course(course) => {
            registration::handlers::handle_course(bot.clone(), q, db, course).await?;
        }
        MitGroup(group) => {
            registration::handlers::handle_group(bot.clone(), q, db, group).await?;
        }
    }

    Ok(())
}
