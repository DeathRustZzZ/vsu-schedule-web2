//src/services/telegram/menu_dispatcher.rs
//! Диспетчер для удобной обработки команд меню с контекстом

use teloxide::prelude::*;
use teloxide::types::InlineKeyboardMarkup;
use crate::domain::menu::MenuCommand;
use crate::services::telegram::menu;
use crate::services::telegram::registration;
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
        Self::edit_or_send(
            context,
            "🏠 Главное меню\n\nВыберите действие:",
            menu::menu_two_column_keyboard(&[
                MenuCommand::MyProfile,
                MenuCommand::MySchedule,
                MenuCommand::ChooseGroup,
                MenuCommand::Help,
            ]),
        )
        .await?;

        Ok(())
    }

    /// Показать меню регистрации
    async fn show_registration_menu(
        context: &MenuCommandContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        context
            .bot
            .send_message(
                context.query.from.id,
                "📝 *Регистрация*\n\nНачнём процесс регистрации. Выберите факультет:",
            )
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .await?;

        Ok(())
    }

    /// Показать профиль
    async fn show_profile(
        context: &MenuCommandContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Self::edit_or_send(
            context,
            "👤 Мой профиль\n\n(здесь будут данные профиля)",
            menu::menu_inline_keyboard(&[MenuCommand::Back]),
        )
        .await?;

        Ok(())
    }

    /// Показать расписание
    async fn show_schedule(
        context: &MenuCommandContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Self::edit_or_send(
            context,
            "📅 Моё расписание\n\n(здесь будет расписание)",
            menu::menu_inline_keyboard(&[MenuCommand::Back]),
        )
        .await?;

        Ok(())
    }

    /// Показать выбор группы
    async fn show_group_selection(
        context: &MenuCommandContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Self::edit_or_send(
            context,
            "👥 Выберите группу\n\nДля просмотра расписания выберите нужную группу:",
            menu::menu_inline_keyboard(&[MenuCommand::Back]),
        )
        .await?;

        Ok(())
    }

    /// Показать справку
    async fn show_help(
        context: &MenuCommandContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        context
            .bot
            .send_message(
                context.query.from.id,
                "❓ Справка\n\n\
                Этот бот помогает вам:\n\
                👤 Просматривать свой профиль\n\
                📅 Смотреть расписание вашей группы\n\
                👥 Выбирать другие группы для просмотра их расписания\n\n\
                Используйте меню для удобной навигации.",
            )
            .await?;

        Ok(())
    }

    async fn edit_or_send(
        context: &MenuCommandContext,
        text: &str,
        markup: InlineKeyboardMarkup,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(message) = context.query.message.as_ref() {
            context
                .bot
                .edit_message_text(context.query.from.id, message.id(), text)
                .reply_markup(markup)
                .await?;
        } else {
            context
                .bot
                .send_message(context.query.from.id, text)
                .reply_markup(markup)
                .await?;
        }
        Ok(())
    }
}
