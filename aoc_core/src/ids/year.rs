use std::{num::ParseIntError, ops::Deref, str::FromStr};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct YearId(u16);

impl YearId {
    pub fn name(&self) -> String {
        format!("Advent of Code {}", self.0)
    }

    pub fn folder_name(&self) -> String {
        format!("year{}", self.0)
    }
}

impl FromStr for YearId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("Year") || s.starts_with("year") {
            s[4..].parse()
        } else if s.starts_with("Y") || s.starts_with("y") {
            s[1..].parse()
        } else {
            s.parse()
        }
    }
}

impl From<u16> for YearId {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl Deref for YearId {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
