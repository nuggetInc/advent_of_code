use std::{num::ParseIntError, ops::Deref, str::FromStr};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DayId(u8);

impl DayId {
    pub fn name(&self) -> String {
        format!("Day {:02}", self.0)
    }

    pub fn folder_name(&self) -> String {
        format!("Day{:02}", self.0)
    }
}

impl FromStr for DayId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("Day") || s.starts_with("day") {
            s[3..].parse()
        } else if s.starts_with("D") || s.starts_with("d") {
            s[1..].parse()
        } else {
            s.parse()
        }
    }
}

impl From<u8> for DayId {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl Deref for DayId {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}