use std::{
    io::{self, Write},
    time::Duration,
};

use crossterm::{
    style::{Print, Stylize},
    QueueableCommand,
};

use crate::{DayId, PartResult};

pub struct DayResult {
    day: DayId,
    parts: Vec<Box<dyn PartResult>>,
}

impl DayResult {
    pub fn new(day: DayId, parts: Vec<Box<dyn PartResult>>) -> Self {
        Self { day, parts }
    }

    pub fn elapsed(&self) -> Duration {
        self.parts.iter().map(|p| p.elapsed()).sum()
    }

    pub fn print(&self) -> io::Result<()> {
        io::stdout()
            .queue(Print(self.day.name()))?
            .queue(Print(format!(" - {:?}", self.elapsed()).dark_grey()))?
            .queue(Print("\n\n"))?
            .flush()?;

        for part in &self.parts {
            part.print()?;
        }

        Ok(())
    }
}
