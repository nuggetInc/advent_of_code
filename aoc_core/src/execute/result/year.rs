use std::{
    io::{self, Write},
    time::Duration,
};

use crossterm::{
    style::{Print, Stylize},
    QueueableCommand,
};

use super::DayResult;
use crate::YearId;

pub struct YearResult {
    year: YearId,
    days: Vec<DayResult>,
}

impl YearResult {
    pub fn new(year: YearId, days: Vec<DayResult>) -> Self {
        Self { year, days }
    }

    pub fn elapsed(&self) -> Duration {
        self.days.iter().map(|d| d.elapsed()).sum()
    }

    pub fn print(&self) -> io::Result<()> {
        io::stdout()
            .queue(Print(self.year.name()))?
            .queue(Print(format!(" - {:?}", self.elapsed()).dark_grey()))?
            .queue(Print("\n"))?
            .flush()?;

        for day in &self.days {
            io::stdout().queue(Print("\n"))?.flush()?;
            day.print()?;
        }

        Ok(())
    }
}
