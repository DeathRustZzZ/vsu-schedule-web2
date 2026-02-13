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

    pub fn parse_or_idle(state: &str) -> Self {
        <Self as std::str::FromStr>::from_str(state).unwrap_or(Self::Idle)
    }
}

impl std::str::FromStr for RegistrationState {
    type Err = ();

    fn from_str(state: &str) -> Result<Self, Self::Err> {
        Ok(match state {
            "await_faculty" => RegistrationState::AwaitingFaculty,
            "await_study_form" => RegistrationState::AwaitingStudyForm,
            "await_course" => RegistrationState::AwaitingCourse,
            "await_group" => RegistrationState::AwaitingGroup,
            _ => return Err(()),
        })
    }
}
