use std::{num::ParseIntError, ops::Deref, str::FromStr};

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
