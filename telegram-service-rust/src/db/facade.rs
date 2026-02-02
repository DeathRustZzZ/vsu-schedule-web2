use sqlx::PgPool;
use crate::domain::student::Student;
use crate::domain::user_state::UserState;
use crate::db::{students_repo, user_states_repo};
use log::{info, debug, error};

/// DbFacade — фасад для работы с базой.
/// Все операции с БД идут через него.
#[derive(Clone)]
pub struct DbFacade {
    pool: PgPool,
}

impl DbFacade {
    /// Создаём новый фасад
    pub fn new(pool: PgPool) -> Self {
        info!("Создан новый DbFacade с пулом соединений.");
        Self { pool }
    }

    // ============= Студенты =============

    /// Найти студента по Telegram ID
    pub async fn find_student(&self, telegram_id: i64) -> anyhow::Result<Option<Student>> {
        debug!("Поиск студента с telegram_id={}", telegram_id);
        students_repo::find_by_telegram_id(&self.pool, telegram_id).await
    }

    /// Зарегистрировать нового студента
    ///
    /// - `telegram_id`: ID пользователя в Telegram
    /// - `faculty`: факультет
    /// - `group`: учебная группа
    /// - `study_form`: форма обучения ("очная" или "заочная")
    pub async fn register_student(
        &self,
        telegram_id: i64,
        faculty: &str,
        group: &str,
        study_form: &str,
        course: Option<&str>,
        username: Option<&str>,
    ) -> anyhow::Result<Student> {
        info!(
            "Регистрация студента: telegram_id={}, faculty={}, group={}, study_form={}, course={:?}, username={:?}",
            telegram_id, faculty, group, study_form, course, username
        );
        match students_repo::insert(
            &self.pool,
            telegram_id,
            faculty,
            group,
            study_form,
            course,
            username,
        )
        .await
        {
            Ok(student) => {
                info!(
                    "Студент с telegram_id={} успешно зарегистрирован (id={}).",
                    telegram_id, student.id
                );
                // Удаляем состояние после успешной регистрации
                let _ = user_states_repo::delete(&self.pool, telegram_id).await;
                Ok(student)
            }
            Err(e) => {
                error!("Ошибка при регистрации студента telegram_id={}: {:?}", telegram_id, e);
                Err(e)
            }
        }
    }

    /// Обновить данные студента
    pub async fn update_student(
        &self,
        telegram_id: i64,
        faculty: &str,
        group: &str,
        study_form: &str,
    ) -> anyhow::Result<Option<Student>> {
        students_repo::update(&self.pool, telegram_id, faculty, group, study_form).await
    }

    /// Удалить студента
    pub async fn delete_student(&self, telegram_id: i64) -> anyhow::Result<u64> {
        students_repo::delete(&self.pool, telegram_id).await
    }

    // ============= Состояние пользователя =============

    /// Найти состояние пользователя
    pub async fn get_user_state(&self, telegram_id: i64) -> anyhow::Result<Option<UserState>> {
        user_states_repo::find_by_telegram_id(&self.pool, telegram_id).await
    }

    /// Создать состояние пользователя (начало регистрации)
    pub async fn create_user_state(&self, telegram_id: i64) -> anyhow::Result<UserState> {
        user_states_repo::insert(&self.pool, telegram_id).await
    }

    /// Сбросить состояние пользователя (новая регистрация)
    pub async fn reset_user_state(&self, telegram_id: i64) -> anyhow::Result<UserState> {
        user_states_repo::reset(&self.pool, telegram_id).await
    }

    /// Обновить факультет в состоянии
    pub async fn set_user_faculty(&self, telegram_id: i64, faculty: &str) -> anyhow::Result<UserState> {
        user_states_repo::update_faculty(&self.pool, telegram_id, faculty).await
    }

    /// Обновить форму обучения в состоянии
    pub async fn set_user_study_form(&self, telegram_id: i64, form: &str) -> anyhow::Result<UserState> {
        user_states_repo::update_study_form(&self.pool, telegram_id, form).await
    }

    /// Обновить курс в состоянии
    pub async fn set_user_course(&self, telegram_id: i64, course: &str) -> anyhow::Result<UserState> {
        user_states_repo::update_course(&self.pool, telegram_id, course).await
    }
}
