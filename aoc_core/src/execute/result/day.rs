use std::{
    ffi::OsStr,
    io::{self, Write},
    path::PathBuf,
    time::Duration,
};

use crossterm::{
    style::{Color, Print, SetForegroundColor, Stylize},
    QueueableCommand,
};

use crate::{DayId, PartResult};

pub struct DayResult {
    day: DayId,
    file_parts: Vec<(PathBuf, Vec<PartResult>)>,
}

impl DayResult {
    pub fn new(day: DayId, parts: Vec<(PathBuf, Vec<PartResult>)>) -> Self {
        Self {
            day,
            file_parts: parts,
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.file_parts
            .iter()
            .map(|(_, parts)| parts.iter().map(|part| part.elapsed()).sum::<Duration>())
            .sum()
    }

    pub fn print(&self) -> io::Result<()> {
        io::stdout()
            .queue(Print(self.day.name()))?
            .queue(Print(format!(" - {:?}", self.elapsed()).dark_grey()))?
            .queue(Print("\n\n"))?
            .flush()?;

        for (file, parts) in &self.file_parts {
            let file_name = file
                .file_name()
                .and_then(OsStr::to_str)
                .expect("Couldn't get input filename");

            io::stdout()
                .queue(SetForegroundColor(Color::DarkGrey))?
                .queue(Print(file_name))?
                .queue(Print(": \n"))?
                .queue(SetForegroundColor(Color::Reset))?
                .flush()?;

            for part in parts {
                part.print()?;
            }
        }

        Ok(())
    }
}
