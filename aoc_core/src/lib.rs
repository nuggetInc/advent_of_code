mod download;
mod execute;

use std::{num::ParseIntError, ops::Deref, str::FromStr};

pub use download::download_problem;
pub use execute::*;

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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PartId(u8);

impl PartId {
    pub const PART_1: PartId = PartId(1);
    pub const PART_2: PartId = PartId(2);

    pub fn name(&self) -> String {
        format!("Part {}", self.0)
    }
}

impl FromStr for PartId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("Part") || s.starts_with("part") {
            s[4..].parse()
        } else if s.starts_with("P") || s.starts_with("p") {
            s[1..].parse()
        } else {
            s.parse()
        }
    }
}

impl From<u8> for PartId {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl Deref for PartId {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
