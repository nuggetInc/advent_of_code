use std::{error::Error, fmt, num::ParseIntError, ops::Deref, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PartId(u8);

impl PartId {
    pub const PART_1: PartId = PartId(1);
    pub const PART_2: PartId = PartId(2);

    pub fn name(&self) -> String {
        format!("Part {}", self.0)
    }
}

impl FromStr for PartId {
    type Err = ParsePartIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = if s.starts_with("Part") || s.starts_with("part") {
            s[4..].parse()
        } else if s.starts_with('P') || s.starts_with('p') {
            s[1..].parse()
        } else {
            s.parse()
        };

        match result {
            Ok(id) => Ok(Self(id)),
            Err(err) => Err(ParsePartIdError(err)),
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

#[derive(Debug)]
pub struct ParsePartIdError(ParseIntError);

impl fmt::Display for ParsePartIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cannot parse part: {}", self.0)
    }
}

impl Error for ParsePartIdError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}
