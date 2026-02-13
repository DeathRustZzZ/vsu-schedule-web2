use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StudyForm {
    FullTime, // очная
    PartTime, // заочная
}

impl StudyForm {
    pub fn title(&self) -> &'static str {
        match self {
            StudyForm::FullTime => "Очная форма",
            StudyForm::PartTime => "Заочная форма",
        }
    }

    pub fn callback(&self) -> &'static str {
        match self {
            StudyForm::FullTime => "form_fulltime",
            StudyForm::PartTime => "form_parttime",
        }
    }
}

impl FromStr for StudyForm {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "form_fulltime" => Ok(StudyForm::FullTime),
            "form_parttime" => Ok(StudyForm::PartTime),
            _ => Err(()),
        }
    }
}
