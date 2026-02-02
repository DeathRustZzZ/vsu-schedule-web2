use teloxide::prelude::*;
use crate::services::telegram::registration::keyboards::{
    faculty_keyboard, study_form_keyboard, course_keyboard, mit_group_keyboard,
};
use crate::domain::{faculty::Faculty, study_form::StudyForm, course::Course, buttons::TechButton};
use crate::domain::groups::mit::MitGroup;
use crate::db::facade::DbFacade;
use std::sync::Arc;
use log::{info, error};

/// Обработчик основных кнопок (Register, MySchedule, ChooseGroup)
pub async fn handle_tech_button(
    bot: Bot,
    q: CallbackQuery,
    btn: TechButton,
) -> Result<(), teloxide::RequestError> {
    match btn {
        TechButton::Register => handle_register(bot, q).await?,
        TechButton::MySchedule => handle_schedule(bot, q).await?,
        TechButton::ChooseGroup => handle_choose_group(bot, q).await?,
    }
    Ok(())
}

/// Начало регистрации - предлагаем выбрать факультет
pub async fn handle_register(bot: Bot, q: CallbackQuery) -> Result<(), teloxide::RequestError> {
    bot.send_message(q.from.id, "✨ Начинаем регистрацию!\nВыбери факультет 📚")
        .reply_markup(faculty_keyboard())
        .await?;
    
    Ok(())
}

/// Показать расписание пользователя
pub async fn handle_schedule(bot: Bot, q: CallbackQuery) -> Result<(), teloxide::RequestError> {
    bot.send_message(q.from.id, "📅 Вот твоё расписание...\n\n(функция в разработке)")
        .await?;
    Ok(())
}

/// Позволить выбрать другую группу
pub async fn handle_choose_group(bot: Bot, q: CallbackQuery) -> Result<(), teloxide::RequestError> {
    bot.send_message(q.from.id, "🔎 Выбери группу")
        .reply_markup(mit_group_keyboard())
        .await?;
    Ok(())
}

/// Обработка выбора факультета
pub async fn handle_faculty_choice(
    bot: Bot,
    q: CallbackQuery,
    faculty: Faculty,
) -> Result<(), teloxide::RequestError> {
    info!("Пользователь {} выбрал факультет: {:?}", q.from.id, faculty);
    
    bot.send_message(
        q.from.id,
        format!("✅ Ты выбрал: {}\n\n📝 Теперь выбери форму обучения:", faculty.title()),
    )
    .reply_markup(study_form_keyboard())
    .await?;
    
    Ok(())
}

/// Обработка выбора формы обучения
pub async fn handle_study_form(
    bot: Bot,
    q: CallbackQuery,
    form: StudyForm,
) -> Result<(), teloxide::RequestError> {
    info!("Пользователь {} выбрал форму обучения: {:?}", q.from.id, form);
    
    bot.send_message(
        q.from.id,
        format!("✅ Форма: {}\n\n📚 Теперь выбери курс:", form.title()),
    )
    .reply_markup(course_keyboard())
    .await?;
    
    Ok(())
}

/// Обработка выбора курса
pub async fn handle_course(
    bot: Bot,
    q: CallbackQuery,
    course: Course,
) -> Result<(), teloxide::RequestError> {
    info!("Пользователь {} выбрал курс: {:?}", q.from.id, course);
    
    bot.send_message(
        q.from.id,
        format!("✅ Курс: {}\n\n👥 Теперь выбери свою группу:", course.title()),
    )
    .reply_markup(mit_group_keyboard())
    .await?;
    
    Ok(())
}

/// Завершение регистрации - сохранение группы
pub async fn handle_group(
    bot: Bot,
    q: CallbackQuery,
    db: Arc<DbFacade>,
    group: MitGroup,
) -> Result<(), teloxide::RequestError> {
    let telegram_id = q.from.id.0 as i64;
    
    info!("Пользователь {} выбрал группу: {:?}", q.from.id, group);
    
    // Для простоты предполагаем, что выбирающий МИТ факультет, очная форма
    // В реальной системе нужно брать эти данные из состояния пользователя
    match db.register_student(telegram_id, "МИТ", group.title(), "Очная").await {
        Ok(student) => {
            info!("Студент {} успешно зарегистрирован", telegram_id);
            bot.send_message(
                q.from.id,
                format!(
                    "🎉 Ты успешно зарегистрирован!\n\n\
                    📚 Факультет: {}\n\
                    📝 Форма: {}\n\
                    👥 Группа: {}\n\n\
                    Теперь ты можешь просматривать расписание!",
                    student.faculty, student.study_form, student.group_name
                ),
            )
            .await?;
        }
        Err(err) => {
            error!("Ошибка при регистрации студента {}: {:?}", telegram_id, err);
            bot.send_message(
                q.from.id,
                "❌ Ошибка при регистрации. Пожалуйста, попробуй ещё раз.",
            )
            .await?;
        }
    }
    
    Ok(())
}
