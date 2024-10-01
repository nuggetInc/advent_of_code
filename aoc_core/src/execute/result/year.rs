use std::{
    collections::BTreeMap,
    io::{self, Write},
    time::Duration,
};

use crossterm::{
    style::{Print, Stylize},
    QueueableCommand,
};

use super::DayResult;
use crate::{AocResult, Day, DayError, Id};

pub struct YearResult {
    days: BTreeMap<Id<Day>, Result<DayResult, DayError>>,
}

impl YearResult {
    pub fn new(days: BTreeMap<Id<Day>, Result<DayResult, DayError>>) -> Self {
        Self { days }
    }

    pub fn elapsed(&self) -> Duration {
        self.days
            .values()
            .filter_map(|result| result.as_ref().ok().map(|day| day.elapsed()))
            .sum()
    }

    pub fn print(&self) -> AocResult<()> {
        io::stdout()
            .queue(Print(format!(" - {:?}", self.elapsed()).dark_grey()))?
            .queue(Print("\n"))?
            .flush()?;

        for (day_id, result) in &self.days {
            io::stdout()
                .queue(Print("\n"))?
                .queue(Print(format!("Day {day_id}")))?
                .flush()?;

            match result {
                Ok(day) => day.print()?,
                Err(error) => io::stdout()
                    .queue(Print(format!(" X {}\n", error).red()))?
                    .flush()?,
            }
        }

        Ok(())
    }
}
