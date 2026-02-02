//src/services/telegram/menu_dispatcher.rs
//! Диспетчер для удобной обработки команд меню с контекстом

use teloxide::prelude::*;
use crate::domain::menu::MenuCommand;
use crate::services::telegram::menu;
use crate::services::telegram::registration;
use crate::services::telegram::ui::render_ui;
use crate::db::facade::DbFacade;
use log::info;
use std::sync::Arc;

/// Контекст команды меню
pub struct MenuCommandContext {
    pub bot: Bot,
    pub query: CallbackQuery,
    pub db: Arc<DbFacade>,
}

/// Диспетчер команд меню
pub struct MenuDispatcher;

impl MenuDispatcher {
    /// Обработать команду меню с автоматическим уведомлением
    pub async fn dispatch(
        context: &MenuCommandContext,
        command: MenuCommand,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match command {
            MenuCommand::MainMenu => {
                info!("📍 Переход в главное меню пользователем {}", context.query.from.id);
                Self::show_main_menu(context).await?;
            }
            MenuCommand::Register => {
                info!("📝 Регистрация пользователя {}", context.query.from.id);
                registration::handlers::handle_register(
                    context.bot.clone(),
                    context.query.clone(),
                    Arc::clone(&context.db),
                )
                .await?;
            }
            MenuCommand::MyProfile => {
                info!("👤 Просмотр профиля пользователем {}", context.query.from.id);
                Self::show_profile(context).await?;
            }
            MenuCommand::MySchedule => {
                info!("📅 Просмотр расписания пользователем {}", context.query.from.id);
                Self::show_schedule(context).await?;
            }
            MenuCommand::ChooseGroup => {
                info!("👥 Выбор группы пользователем {}", context.query.from.id);
                registration::handlers::handle_choose_group(
                    context.bot.clone(),
                    context.query.clone(),
                    Arc::clone(&context.db),
                )
                .await?;
            }
            MenuCommand::Back => {
                info!("◀️ Возврат в главное меню пользователем {}", context.query.from.id);
                Self::show_main_menu(context).await?;
            }
            MenuCommand::Help => {
                info!("❓ Справка для пользователя {}", context.query.from.id);
                Self::show_help(context).await?;
            }
        }

        Ok(())
    }

    /// Показать главное меню
    async fn show_main_menu(
        context: &MenuCommandContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        render_ui(
            &context.bot,
            context.db.as_ref(),
            context.query.from.id.0 as i64,
            context.query
                .message
                .as_ref()
                .map(|m| m.chat().id)
                .unwrap_or(context.query.from.id.into()),
            context.query.message.as_ref().map(|m| m.id()),
            "🏠 Главное меню\n\nВыберите действие:",
            Some(menu::menu_two_column_keyboard(&[
                MenuCommand::MyProfile,
                MenuCommand::MySchedule,
                MenuCommand::ChooseGroup,
                MenuCommand::Help,
            ])),
            None,
        )
        .await?;

        Ok(())
    }


    /// Показать профиль
    async fn show_profile(
        context: &MenuCommandContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        render_ui(
            &context.bot,
            context.db.as_ref(),
            context.query.from.id.0 as i64,
            context.query
                .message
                .as_ref()
                .map(|m| m.chat().id)
                .unwrap_or(context.query.from.id.into()),
            context.query.message.as_ref().map(|m| m.id()),
            "👤 Мой профиль\n\n(здесь будут данные профиля)",
            Some(menu::menu_inline_keyboard(&[MenuCommand::Back])),
            None,
        )
        .await?;

        Ok(())
    }

    /// Показать расписание
    async fn show_schedule(
        context: &MenuCommandContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        render_ui(
            &context.bot,
            context.db.as_ref(),
            context.query.from.id.0 as i64,
            context.query
                .message
                .as_ref()
                .map(|m| m.chat().id)
                .unwrap_or(context.query.from.id.into()),
            context.query.message.as_ref().map(|m| m.id()),
            "📅 Моё расписание\n\n(здесь будет расписание)",
            Some(menu::menu_inline_keyboard(&[MenuCommand::Back])),
            None,
        )
        .await?;

        Ok(())
    }

    /// Показать выбор группы
    async fn show_group_selection(
        context: &MenuCommandContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        render_ui(
            &context.bot,
            context.db.as_ref(),
            context.query.from.id.0 as i64,
            context.query
                .message
                .as_ref()
                .map(|m| m.chat().id)
                .unwrap_or(context.query.from.id.into()),
            context.query.message.as_ref().map(|m| m.id()),
            "👥 Выберите группу\n\nДля просмотра расписания выберите нужную группу:",
            Some(menu::menu_inline_keyboard(&[MenuCommand::Back])),
            None,
        )
        .await?;

        Ok(())
    }

    /// Показать справку
    async fn show_help(
        context: &MenuCommandContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        render_ui(
            &context.bot,
            context.db.as_ref(),
            context.query.from.id.0 as i64,
            context.query
                .message
                .as_ref()
                .map(|m| m.chat().id)
                .unwrap_or(context.query.from.id.into()),
            context.query.message.as_ref().map(|m| m.id()),
            "❓ Справка\n\n\
Этот бот помогает вам:\n\
👤 Просматривать свой профиль\n\
📅 Смотреть расписание вашей группы\n\
👥 Выбирать другие группы для просмотра их расписания\n\n\
Используйте меню для удобной навигации.",
            None,
            None,
        )
        .await?;

        Ok(())
    }
}
