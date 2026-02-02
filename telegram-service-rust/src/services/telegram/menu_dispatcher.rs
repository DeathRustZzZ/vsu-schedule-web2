//src/services/telegram/menu_dispatcher.rs
//! Диспетчер для удобной обработки команд меню с контекстом

use teloxide::prelude::*;
use crate::domain::menu::MenuCommand;
use crate::services::telegram::menu;
use log::info;

/// Контекст команды меню
pub struct MenuCommandContext {
    pub bot: Bot,
    pub query: CallbackQuery,
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
                Self::show_registration_menu(context).await?;
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
                Self::show_group_selection(context).await?;
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

        // Уведомить пользователя об успешной обработке команды
        context.bot.answer_callback_query(context.query.id.clone()).await?;
        Ok(())
    }

    /// Показать главное меню
    async fn show_main_menu(
        context: &MenuCommandContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        context
            .bot
            .edit_message_text(
                context.query.from.id,
                context.query.message.as_ref().unwrap().id(),
                "🏠 **Главное меню**\n\nВыберите действие:",
            )
            .reply_markup(menu::menu_two_column_keyboard(&[
                MenuCommand::MyProfile,
                MenuCommand::MySchedule,
                MenuCommand::ChooseGroup,
                MenuCommand::Help,
            ]))
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
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
        context
            .bot
            .edit_message_text(
                context.query.from.id,
                context.query.message.as_ref().unwrap().id(),
                "👤 *Мой профиль*\n\n_\\(здесь будут данные профиля\\)_",
            )
            .reply_markup(menu::menu_inline_keyboard(&[MenuCommand::Back]))
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .await?;

        Ok(())
    }

    /// Показать расписание
    async fn show_schedule(
        context: &MenuCommandContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        context
            .bot
            .edit_message_text(
                context.query.from.id,
                context.query.message.as_ref().unwrap().id(),
                "📅 *Моё расписание*\n\n_\\(здесь будет расписание\\)_",
            )
            .reply_markup(menu::menu_inline_keyboard(&[MenuCommand::Back]))
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .await?;

        Ok(())
    }

    /// Показать выбор группы
    async fn show_group_selection(
        context: &MenuCommandContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        context
            .bot
            .edit_message_text(
                context.query.from.id,
                context.query.message.as_ref().unwrap().id(),
                "👥 *Выберите группу*\n\nДля просмотра расписания выберите нужную группу:",
            )
            .reply_markup(menu::menu_inline_keyboard(&[MenuCommand::Back]))
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
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
                "❓ *Справка*\n\n\
                Этот бот помогает вам:\n\
                👤 Просматривать свой профиль\n\
                📅 Смотреть расписание вашей группы\n\
                👥 Выбирать другие группы для просмотра их расписания\n\n\
                Используйте меню для удобной навигации.",
            )
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .await?;

        Ok(())
    }
}

