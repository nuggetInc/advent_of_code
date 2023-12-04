use core::fmt;
use std::time::Duration;

use termion::color::{Black, Fg, Reset};

use super::DayResult;
pub struct YearResult {
    name: String,
    days: Vec<DayResult>,
    elapsed: Duration,
}

impl YearResult {
    pub fn new(name: String, days: Vec<DayResult>, elapsed: Duration) -> Self {
        Self {
            name,
            days,
            elapsed,
        }
    }
}

impl fmt::Display for YearResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{: <40}{}{: >20?}{}",
            self.name,
            Fg(Black),
            self.elapsed,
            Fg(Reset),
        )?;

        let last = self.days.last();

        if let Some(last) = last {
            for day in &self.days {
                writeln!(f)?;
                if std::ptr::eq(day, last) {
                    write!(f, "{}", day)?;
                } else {
                    writeln!(f, "{}", day)?;
                }
            }
        }
        Ok(())
    }
}
