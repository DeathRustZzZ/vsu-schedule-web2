use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Course {
    First,
    Second,
    Third,
    Fourth,
}

impl Course {
    pub fn title(&self) -> &'static str {
        match self {
            Course::First => "1 курс",
            Course::Second => "2 курс",
            Course::Third => "3 курс",
            Course::Fourth => "4 курс",
        }
    }

    pub fn callback(&self) -> &'static str {
        match self {
            Course::First => "course_1",
            Course::Second => "course_2",
            Course::Third => "course_3",
            Course::Fourth => "course_4",
        }
    }
}

impl FromStr for Course {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "course_1" => Ok(Course::First),
            "course_2" => Ok(Course::Second),
            "course_3" => Ok(Course::Third),
            "course_4" => Ok(Course::Fourth),
            _ => Err(()),
        }
    }
}
