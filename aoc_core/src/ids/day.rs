use std::{error::Error, fmt, num::ParseIntError, ops::Deref, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DayId(u8);

impl DayId {
    pub fn name(&self) -> String {
        format!("Day {:02}", self.0)
    }

    pub fn folder_name(&self) -> String {
        format!("day{:02}", self.0)
    }
}

impl FromStr for DayId {
    type Err = ParseDayIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = if s.starts_with("Day") || s.starts_with("day") {
            s[3..].parse::<u8>()
        } else if s.starts_with('D') || s.starts_with('d') {
            s[1..].parse::<u8>()
        } else {
            s.parse::<u8>()
        };

        match result {
            Ok(id) => Ok(Self(id)),
            Err(err) => Err(ParseDayIdError(err)),
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

#[derive(Debug)]
pub struct ParseDayIdError(ParseIntError);

impl fmt::Display for ParseDayIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cannot parse day: {}", self.0)
    }
}

impl Error for ParseDayIdError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}
