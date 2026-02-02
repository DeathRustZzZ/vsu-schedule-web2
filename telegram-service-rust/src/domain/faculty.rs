//src/domain/faculty.rs
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Faculty {
    Mit,
    Hbg,
    Ped,
    Spp,
    Foreign,
    Fks,
    Hzk,
    Hgf,
    Law,
}

impl Faculty {
    pub fn title(&self) -> &'static str {
        match self {
            Faculty::Mit => "Факультет математики и информационных технологий",
            Faculty::Hbg => "Факультет химико-биологических и географических наук",
            Faculty::Ped => "Педагогический факультет",
            Faculty::Spp => "Факультет социальной педагогики и психологии",
            Faculty::Foreign => "Факультет иностранных граждан",
            Faculty::Fks => "Факультет физической культуры и спорта",
            Faculty::Hzk => "Факультет гуманитарного знания и коммуникаций",
            Faculty::Hgf => "Художественно-графический факультет",
            Faculty::Law => "Юридический факультет",
        }
    }

    pub fn callback(&self) -> &'static str {
        match self {
            Faculty::Mit => "faculty_mit",
            Faculty::Hbg => "faculty_hbg",
            Faculty::Ped => "faculty_ped",
            Faculty::Spp => "faculty_spp",
            Faculty::Foreign => "faculty_foreign",
            Faculty::Fks => "faculty_fks",
            Faculty::Hzk => "faculty_hzk",
            Faculty::Hgf => "faculty_hgf",
            Faculty::Law => "faculty_law",
        }
    }
}

impl FromStr for Faculty {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "faculty_mit" => Ok(Faculty::Mit),
            "faculty_hbg" => Ok(Faculty::Hbg),
            "faculty_ped" => Ok(Faculty::Ped),
            "faculty_spp" => Ok(Faculty::Spp),
            "faculty_foreign" => Ok(Faculty::Foreign),
            "faculty_fks" => Ok(Faculty::Fks),
            "faculty_hzk" => Ok(Faculty::Hzk),
            "faculty_hgf" => Ok(Faculty::Hgf),
            "faculty_law" => Ok(Faculty::Law),
            _ => Err(()),
        }
    }
}
