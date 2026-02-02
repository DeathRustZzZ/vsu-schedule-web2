use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MitGroup {
    ISIT,
    PI,
    PInj,
    PM,
    UIR,
    MF,
}

impl MitGroup {
    pub fn title(&self) -> &'static str {
        match self {
            MitGroup::ISIT => "ИСИТ",
            MitGroup::PI => "ПИ",
            MitGroup::PInj => "ПИнж",
            MitGroup::PM => "ПМ",
            MitGroup::UIR => "УИР",
            MitGroup::MF => "МФ",
        }
    }
    pub fn callback(&self) -> &'static str {
        match self {
            MitGroup::ISIT => "mit_group_isit",
            MitGroup::PI => "mit_group_pi",
            MitGroup::PInj => "mit_group_pinj",
            MitGroup::PM => "mit_group_pm",
            MitGroup::UIR => "mit_group_uir",
            MitGroup::MF => "mit_group_mf",
        }
    }
}

impl FromStr for MitGroup {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mit_group_isit" => Ok(MitGroup::ISIT),
            "mit_group_pi" => Ok(MitGroup::PI),
            "mit_group_pinj" => Ok(MitGroup::PInj),
            "mit_group_pm" => Ok(MitGroup::PM),
            "mit_group_uir" => Ok(MitGroup::UIR),
            "mit_group_mf" => Ok(MitGroup::MF),
            _ => Err(()),
        }
    }
}
