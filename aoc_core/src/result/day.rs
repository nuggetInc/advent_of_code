use core::fmt;
use std::time::Duration;

use termion::color::{Black, Fg, Reset};

use super::{ParserResult, PartResult};
use crate::YearDay;
pub struct DayResult {
    day: YearDay,
    parsers: Vec<ParserResult>,
    parts: Vec<PartResult>,
    elapsed: Duration,
}

impl DayResult {
    pub fn new(
        day: YearDay,
        parsers: Vec<ParserResult>,
        parts: Vec<PartResult>,
        elapsed: Duration,
    ) -> Self {
        Self {
            day,
            parsers,
            parts,
            elapsed,
        }
    }
}

impl fmt::Display for DayResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{: <40}{}{: >40?}{}",
            self.day.to_string(),
            Fg(Black),
            self.elapsed,
            Fg(Reset),
        )?;

        for parser in &self.parsers {
            writeln!(f, "{}", parser)?;
        }

        let last = self.parts.last();

        if let Some(last) = last {
            for part in &self.parts {
                if std::ptr::eq(part, last) {
                    write!(f, "{}", part)?;
                } else {
                    writeln!(f, "{}", part)?;
                }
            }
        }

        Ok(())
    }
}
