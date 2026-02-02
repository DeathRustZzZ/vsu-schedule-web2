use crate::db::facade::DbFacade;
use crate::domain::{study_form::StudyForm};
use crate::domain::groups::mit::MitGroup;
use std::sync::Arc;

pub struct RegistrationFacade {
    db: Arc<DbFacade>,
}

impl RegistrationFacade {
    pub fn new(db: Arc<DbFacade>) -> Self {
        Self { db }
    }

    pub async fn register_mit_student(
        &self,
        telegram_id: i64,
        group: MitGroup,
        form: StudyForm,
    ) -> anyhow::Result<()> {
        self.db.register_student(
            telegram_id,
            "МИТ",
            group.title(),
            form.title(),
            None,
            None,
        ).await?;
        Ok(())
    }
}

