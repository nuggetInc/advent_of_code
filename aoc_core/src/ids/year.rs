use std::{error::Error, fmt, num::ParseIntError, ops::Deref, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
    type Err = ParseYearIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = if s.starts_with("Year") || s.starts_with("year") {
            s[4..].parse()
        } else if s.starts_with('Y') || s.starts_with('y') {
            s[1..].parse()
        } else {
            s.parse()
        };

        match result {
            Ok(id) => Ok(Self(id)),
            Err(err) => Err(ParseYearIdError(err)),
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

#[derive(Debug)]
pub struct ParseYearIdError(ParseIntError);

impl fmt::Display for ParseYearIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cannot parse year: {}", self.0)
    }
}

impl Error for ParseYearIdError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}
