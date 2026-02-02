use teloxide::prelude::*;
use crate::services::telegram::registration::keyboards::{
    faculty_keyboard, study_form_keyboard, course_keyboard, mit_group_keyboard,
};
use crate::domain::{faculty::Faculty, study_form::StudyForm, course::Course, buttons::TechButton};
use crate::domain::groups::mit::MitGroup;
use crate::db::facade::DbFacade;
use crate::services::telegram::ui::render_ui;
use std::sync::Arc;
use log::{info, error};

/// Обработчик основных кнопок (Register, MySchedule, ChooseGroup)
pub async fn handle_tech_button(
    bot: Bot,
    q: CallbackQuery,
    db: Arc<DbFacade>,
    btn: TechButton,
) -> Result<(), teloxide::RequestError> {
    match btn {
        TechButton::Register => handle_register(bot, q, db).await?,
        TechButton::MySchedule => handle_schedule(bot, q, db).await?,
        TechButton::ChooseGroup => handle_choose_group(bot, q, db).await?,
    }
    Ok(())
}

/// Начало регистрации - предлагаем выбрать факультет
pub async fn handle_register(
    bot: Bot,
    q: CallbackQuery,
    db: Arc<DbFacade>,
) -> Result<(), teloxide::RequestError> {
    let telegram_id = q.from.id.0 as i64;
    let chat_id = q.message.as_ref().map(|m| m.chat().id).unwrap_or(q.from.id.into());
    if let Err(err) = db.reset_user_state(telegram_id).await {
        error!("Ошибка при сбросе состояния пользователя {}: {:?}", telegram_id, err);
        bot.send_message(
            q.from.id,
            "❌ Ошибка при начале регистрации. Пожалуйста, попробуй ещё раз.",
        )
        .await?;
        return Ok(());
    }

    render_ui(
        &bot,
        db.as_ref(),
        telegram_id,
        chat_id,
        q.message.as_ref().map(|m| m.id()),
        "✨ Начинаем регистрацию!\nВыбери факультет 📚",
        Some(faculty_keyboard()),
        None,
    )
    .await?;
    
    Ok(())
}

/// Показать расписание пользователя
pub async fn handle_schedule(
    bot: Bot,
    q: CallbackQuery,
    db: Arc<DbFacade>,
) -> Result<(), teloxide::RequestError> {
    let chat_id = q.message.as_ref().map(|m| m.chat().id).unwrap_or(q.from.id.into());
    render_ui(
        &bot,
        db.as_ref(),
        q.from.id.0 as i64,
        chat_id,
        q.message.as_ref().map(|m| m.id()),
        "📅 Вот твоё расписание...\n\n(функция в разработке)",
        None,
        None,
    )
    .await?;
    Ok(())
}

/// Позволить выбрать другую группу
pub async fn handle_choose_group(
    bot: Bot,
    q: CallbackQuery,
    db: Arc<DbFacade>,
) -> Result<(), teloxide::RequestError> {
    let chat_id = q.message.as_ref().map(|m| m.chat().id).unwrap_or(q.from.id.into());
    render_ui(
        &bot,
        db.as_ref(),
        q.from.id.0 as i64,
        chat_id,
        q.message.as_ref().map(|m| m.id()),
        "🔎 Выбери группу",
        Some(mit_group_keyboard()),
        None,
    )
    .await?;
    Ok(())
}

/// Обработка выбора факультета
pub async fn handle_faculty_choice(
    bot: Bot,
    q: CallbackQuery,
    db: Arc<DbFacade>,
    faculty: Faculty,
) -> Result<(), teloxide::RequestError> {
    info!("Пользователь {} выбрал факультет: {:?}", q.from.id, faculty);

    let telegram_id = q.from.id.0 as i64;
    let chat_id = q.message.as_ref().map(|m| m.chat().id).unwrap_or(q.from.id.into());
    if let Err(err) = db.set_user_faculty(telegram_id, faculty.title()).await {
        error!("Ошибка при сохранении факультета {}: {:?}", telegram_id, err);
        bot.send_message(
            q.from.id,
            "❌ Не удалось сохранить факультет. Пожалуйста, попробуй ещё раз.",
        )
        .await?;
        return Ok(());
    }
    
    render_ui(
        &bot,
        db.as_ref(),
        telegram_id,
        chat_id,
        q.message.as_ref().map(|m| m.id()),
        &format!("✅ Ты выбрал: {}\n\n📝 Теперь выбери форму обучения:", faculty.title()),
        Some(study_form_keyboard()),
        None,
    )
    .await?;
    
    Ok(())
}

/// Обработка выбора формы обучения
pub async fn handle_study_form(
    bot: Bot,
    q: CallbackQuery,
    db: Arc<DbFacade>,
    form: StudyForm,
) -> Result<(), teloxide::RequestError> {
    info!("Пользователь {} выбрал форму обучения: {:?}", q.from.id, form);

    let telegram_id = q.from.id.0 as i64;
    let chat_id = q.message.as_ref().map(|m| m.chat().id).unwrap_or(q.from.id.into());
    if let Err(err) = db.set_user_study_form(telegram_id, form.title()).await {
        error!("Ошибка при сохранении формы обучения {}: {:?}", telegram_id, err);
        bot.send_message(
            q.from.id,
            "❌ Не удалось сохранить форму обучения. Пожалуйста, попробуй ещё раз.",
        )
        .await?;
        return Ok(());
    }
    
    render_ui(
        &bot,
        db.as_ref(),
        telegram_id,
        chat_id,
        q.message.as_ref().map(|m| m.id()),
        &format!("✅ Форма: {}\n\n📚 Теперь выбери курс:", form.title()),
        Some(course_keyboard()),
        None,
    )
    .await?;
    
    Ok(())
}

/// Обработка выбора курса
pub async fn handle_course(
    bot: Bot,
    q: CallbackQuery,
    db: Arc<DbFacade>,
    course: Course,
) -> Result<(), teloxide::RequestError> {
    info!("Пользователь {} выбрал курс: {:?}", q.from.id, course);

    let telegram_id = q.from.id.0 as i64;
    let chat_id = q.message.as_ref().map(|m| m.chat().id).unwrap_or(q.from.id.into());
    if let Err(err) = db.set_user_course(telegram_id, course.title()).await {
        error!("Ошибка при сохранении курса {}: {:?}", telegram_id, err);
        bot.send_message(
            q.from.id,
            "❌ Не удалось сохранить курс. Пожалуйста, попробуй ещё раз.",
        )
        .await?;
        return Ok(());
    }
    
    render_ui(
        &bot,
        db.as_ref(),
        telegram_id,
        chat_id,
        q.message.as_ref().map(|m| m.id()),
        &format!("✅ Курс: {}\n\n👥 Теперь выбери свою группу:", course.title()),
        Some(mit_group_keyboard()),
        None,
    )
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
    let chat_id = q.message.as_ref().map(|m| m.chat().id).unwrap_or(q.from.id.into());
    
    info!("Пользователь {} выбрал группу: {:?}", q.from.id, group);
    
    let state = match db.get_user_state(telegram_id).await {
        Ok(state) => state,
        Err(err) => {
            error!("Ошибка при получении состояния пользователя {}: {:?}", telegram_id, err);
            bot.send_message(
                q.from.id,
                "❌ Ошибка при регистрации. Пожалуйста, попробуй ещё раз.",
            )
            .await?;
            return Ok(());
        }
    };

    let faculty = state.as_ref().and_then(|s| s.faculty()).unwrap_or("МИТ");
    let study_form = state.as_ref().and_then(|s| s.study_form()).unwrap_or("Очная");
    let course = state.as_ref().and_then(|s| s.course());
    let username = q.from.username.as_deref();

    match db
        .register_student(telegram_id, faculty, group.title(), study_form, course, username)
        .await
    {
        Ok(student) => {
            info!("Студент {} успешно зарегистрирован", telegram_id);
            render_ui(
                &bot,
                db.as_ref(),
                telegram_id,
                chat_id,
                q.message.as_ref().map(|m| m.id()),
                &format!(
                    "🎉 Ты успешно зарегистрирован!\n\n\
                    📚 Факультет: {}\n\
                    📝 Форма: {}\n\
                    👥 Группа: {}\n\n\
                    Теперь ты можешь просматривать расписание!",
                    student.faculty, student.study_form, student.group_name
                ),
                None,
                None,
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
