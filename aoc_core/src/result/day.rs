use core::fmt;
use std::time::Duration;
use termion::color::{Black, Fg, Reset};

use super::{ParserResult, PartResult};
pub struct DayResult {
    name: String,
    parsers: Vec<ParserResult>,
    parts: Vec<PartResult>,
    elapsed: Duration,
}

impl DayResult {
    pub fn new(
        name: String,
        parsers: Vec<ParserResult>,
        parts: Vec<PartResult>,
        elapsed: Duration,
    ) -> Self {
        Self {
            name,
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
            "{: <40}{}{: >20?}{}",
            self.name,
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
