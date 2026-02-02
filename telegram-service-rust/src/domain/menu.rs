//src/domain/menu.rs
//! Определение меню и команд навигации

use std::str::FromStr;

/// Основные команды меню
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuCommand {
    // Главное меню
    MainMenu,
    
    // Регистрация и профиль
    Register,
    MyProfile,
    MySchedule,
    ChooseGroup,
    
    // Навигация
    Back,
    Help,
}

impl MenuCommand {
    /// Текст кнопки для отображения пользователю
    pub fn text(&self) -> &'static str {
        match self {
            MenuCommand::MainMenu => "🏠 Главное меню",
            MenuCommand::Register => "📝 Зарегистрироваться",
            MenuCommand::MyProfile => "👤 Мой профиль",
            MenuCommand::MySchedule => "📅 Моё расписание",
            MenuCommand::ChooseGroup => "👥 Выбрать группу",
            MenuCommand::Back => "◀️ Назад",
            MenuCommand::Help => "❓ Справка",
        }
    }

    /// Уникальный идентификатор команды для callback-данных
    pub fn callback(&self) -> &'static str {
        match self {
            MenuCommand::MainMenu => "menu_main",
            MenuCommand::Register => "menu_register",
            MenuCommand::MyProfile => "menu_profile",
            MenuCommand::MySchedule => "menu_schedule",
            MenuCommand::ChooseGroup => "menu_choose_group",
            MenuCommand::Back => "menu_back",
            MenuCommand::Help => "menu_help",
        }
    }
}

impl FromStr for MenuCommand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "menu_main" => Ok(MenuCommand::MainMenu),
            "menu_register" => Ok(MenuCommand::Register),
            "menu_profile" => Ok(MenuCommand::MyProfile),
            "menu_schedule" => Ok(MenuCommand::MySchedule),
            "menu_choose_group" => Ok(MenuCommand::ChooseGroup),
            "menu_back" => Ok(MenuCommand::Back),
            "menu_help" => Ok(MenuCommand::Help),
            other => Err(format!("Unknown menu command: {}", other)),
        }
    }
}

/// Система меню с иерархией
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuState {
    /// Главное меню
    Main,
    /// Меню не зарегистрированного пользователя
    Registration,
    /// Меню зарегистрированного пользователя
    UserMenu,
    /// Меню выбора расписания
    Schedule,
}

impl MenuState {
    /// Получить описание текущего состояния меню
    pub fn description(&self) -> &'static str {
        match self {
            MenuState::Main => "Добро пожаловать! Выберите действие:",
            MenuState::Registration => "Процесс регистрации. Выберите опцию:",
            MenuState::UserMenu => "Главное меню. Что вы хотите сделать?",
            MenuState::Schedule => "Выберите группу для просмотра расписания:",
        }
    }
}


