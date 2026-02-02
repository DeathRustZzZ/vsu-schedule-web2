#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegistrationState {
    Idle,
    AwaitingFaculty,
    AwaitingStudyForm,
    AwaitingCourse,
    AwaitingGroup,
}

impl RegistrationState {
    pub fn as_str(&self) -> &'static str {
        match self {
            RegistrationState::Idle => "idle",
            RegistrationState::AwaitingFaculty => "await_faculty",
            RegistrationState::AwaitingStudyForm => "await_study_form",
            RegistrationState::AwaitingCourse => "await_course",
            RegistrationState::AwaitingGroup => "await_group",
        }
    }

    pub fn from_str(state: &str) -> Self {
        match state {
            "await_faculty" => RegistrationState::AwaitingFaculty,
            "await_study_form" => RegistrationState::AwaitingStudyForm,
            "await_course" => RegistrationState::AwaitingCourse,
            "await_group" => RegistrationState::AwaitingGroup,
            _ => RegistrationState::Idle,
        }
    }
}
