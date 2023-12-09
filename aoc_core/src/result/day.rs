use core::fmt;
use std::time::Duration;

use termion::color::{Black, Fg, Reset};

use super::PartResult;
use crate::DayId;

pub struct DayResult {
    day: DayId,
    parts: Vec<PartResult>,
    elapsed: Duration,
}

impl DayResult {
    pub fn new(day: DayId, parts: Vec<PartResult>, elapsed: Duration) -> Self {
        Self {
            day,
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
            self.day.name(),
            Fg(Black),
            self.elapsed,
            Fg(Reset),
        )?;

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
