// src/domain/callbacks.rs
//! Единый enum для всех callback-данных из Telegram
//! Позволяет централизованно парсить и обрабатывать callbacks

use std::str::FromStr;
use crate::domain::{
    buttons::TechButton,
    faculty::Faculty,
    study_form::StudyForm,
    course::Course,
    groups::mit::MitGroup,
};

/// Все возможные callback-команды в приложении
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CallbackData {
    // Основные кнопки
    TechButton(TechButton),
    
    // Регистрация: выбор факультета
    Faculty(Faculty),
    
    // Регистрация: выбор формы обучения
    StudyForm(StudyForm),
    
    // Регистрация: выбор курса
    Course(Course),
    
    // Регистрация: выбор группы
    MitGroup(MitGroup),
}

impl FromStr for CallbackData {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Пытаемся парсить как TechButton
        if let Ok(btn) = TechButton::from_str(s) {
            return Ok(CallbackData::TechButton(btn));
        }

        // Пытаемся парсить как Faculty
        if let Ok(fac) = Faculty::from_str(s) {
            return Ok(CallbackData::Faculty(fac));
        }

        // Пытаемся парсить как StudyForm
        if let Ok(form) = StudyForm::from_str(s) {
            return Ok(CallbackData::StudyForm(form));
        }

        // Пытаемся парсить как Course
        if let Ok(course) = Course::from_str(s) {
            return Ok(CallbackData::Course(course));
        }

        // Пытаемся парсить как MitGroup
        if let Ok(group) = MitGroup::from_str(s) {
            return Ok(CallbackData::MitGroup(group));
        }

        Err(format!("Неизвестный callback: {}", s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tech_button() {
        assert_eq!(
            "register".parse::<CallbackData>().unwrap(),
            CallbackData::TechButton(TechButton::Register)
        );
    }

    #[test]
    fn test_parse_faculty() {
        assert_eq!(
            "faculty_mit".parse::<CallbackData>().unwrap(),
            CallbackData::Faculty(Faculty::Mit)
        );
    }

    #[test]
    fn test_invalid_callback() {
        assert!("invalid_callback".parse::<CallbackData>().is_err());
    }
}

