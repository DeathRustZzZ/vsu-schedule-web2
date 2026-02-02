//src/domain/buttons.rs
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TechButton {
    Register,
    MySchedule,
    ChooseGroup,
}

impl TechButton {
    pub fn title(&self) -> &'static str {
        match self {
            TechButton::Register => "Зарегистрироваться",
            TechButton::MySchedule => "Моё расписание",
            TechButton::ChooseGroup => "Выбрать группу",
        }
    }

    pub fn callback(&self) -> &'static str {
        match self {
            TechButton::Register => "register",
            TechButton::MySchedule => "my_schedule",
            TechButton::ChooseGroup => "choose_group",
        }
    }
}

/// Позволяет парсить строку из callback_data обратно в enum
impl FromStr for TechButton {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "register" => Ok(TechButton::Register),
            "my_schedule" => Ok(TechButton::MySchedule),
            "choose_group" => Ok(TechButton::ChooseGroup),
            _ => Err(()),
        }
    }
}
