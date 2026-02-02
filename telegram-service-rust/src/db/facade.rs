use sqlx::PgPool;
use log::{debug, info, error};

use crate::domain::student::Student;
use crate::domain::user_state::UserState;
use crate::db::{students_repo, user_states_repo};
use crate::domain::registration_state::RegistrationState;

#[derive(Clone)]
pub struct DbFacade {
    pool: PgPool,
}

impl DbFacade {
    pub fn new(pool: PgPool) -> Self {
        info!("Создан DbFacade");
        Self { pool }
    }

    pub async fn find_student(&self, telegram_id: i64) -> anyhow::Result<Option<Student>> {
        debug!("Поиск студента telegram_id={}", telegram_id);
        students_repo::find_by_telegram_id(&self.pool, telegram_id).await
    }

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
            "Регистрация студента telegram_id={}, faculty={}, group={}, study_form={}, course={:?}, username={:?}",
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
            Ok(student) => Ok(student),
            Err(err) => {
                error!("Ошибка регистрации студента {}: {:?}", telegram_id, err);
                Err(err)
            }
        }
    }

    pub async fn get_user_state(&self, telegram_id: i64) -> anyhow::Result<Option<UserState>> {
        user_states_repo::find_by_telegram_id(&self.pool, telegram_id).await
    }

    pub async fn reset_registration(
        &self,
        telegram_id: i64,
        state: RegistrationState,
    ) -> anyhow::Result<UserState> {
        user_states_repo::clear_registration(&self.pool, telegram_id, state.as_str()).await
    }

    pub async fn set_state(
        &self,
        telegram_id: i64,
        state: RegistrationState,
    ) -> anyhow::Result<UserState> {
        user_states_repo::upsert_state(&self.pool, telegram_id, state.as_str()).await
    }

    pub async fn set_user_faculty(
        &self,
        telegram_id: i64,
        faculty: &str,
        state: RegistrationState,
    ) -> anyhow::Result<UserState> {
        user_states_repo::update_faculty(&self.pool, telegram_id, faculty, state.as_str()).await
    }

    pub async fn set_user_study_form(
        &self,
        telegram_id: i64,
        form: &str,
        state: RegistrationState,
    ) -> anyhow::Result<UserState> {
        user_states_repo::update_study_form(&self.pool, telegram_id, form, state.as_str()).await
    }

    pub async fn set_user_course(
        &self,
        telegram_id: i64,
        course: &str,
        state: RegistrationState,
    ) -> anyhow::Result<UserState> {
        user_states_repo::update_course(&self.pool, telegram_id, course, state.as_str()).await
    }

    pub async fn get_ui_message(&self, telegram_id: i64) -> anyhow::Result<Option<(i64, i32)>> {
        let state = self.get_user_state(telegram_id).await?;
        Ok(state.and_then(|s| {
            s.ui_chat_id
                .and_then(|chat_id| s.ui_message_id.map(|msg_id| (chat_id, msg_id)))
        }))
    }

    pub async fn set_ui_message(
        &self,
        telegram_id: i64,
        chat_id: i64,
        message_id: i32,
    ) -> anyhow::Result<UserState> {
        user_states_repo::update_ui_message(&self.pool, telegram_id, chat_id, message_id).await
    }
}
