use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MitGroup {
    PI24Z1,
    PI24Z2,
    PI23Z1,
    PI23Z2,
}

impl MitGroup {
    pub fn title(&self) -> &'static str {
        match self {
            MitGroup::PI24Z1 => "24ПИнж1з_1",
            MitGroup::PI24Z2 => "24ПИнж1з_2",
            MitGroup::PI23Z1 => "23ПИнж1з_1",
            MitGroup::PI23Z2 => "23ПИнж1з_2",
        }
    }
    pub fn callback(&self) -> &'static str {
        match self {
            MitGroup::PI24Z1 => "mit_group_24_pinj1z",
            MitGroup::PI24Z2 => "mit_group_24_pinj1z_2",
            MitGroup::PI23Z1 => "mit_group_23_pinj1z",
            MitGroup::PI23Z2 => "mit_group_23_pinj1z_2",
        }
    }
}

impl FromStr for MitGroup {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mit_group_24_pinj1z" => Ok(MitGroup::PI24Z1),
            "mit_group_24_pinj1z_2" => Ok(MitGroup::PI24Z2),
            "mit_group_23_pinj1z" => Ok(MitGroup::PI23Z1),
            "mit_group_23_pinj1z_2" => Ok(MitGroup::PI23Z2),
            _ => Err(()),
        }
    }
}
