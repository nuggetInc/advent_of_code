use std::{
    collections::BTreeMap,
    ffi::OsStr,
    io::{self, Write},
    path::PathBuf,
    time::Duration,
};

use crossterm::{
    style::{Color, Print, SetForegroundColor, Stylize},
    QueueableCommand,
};

use crate::{AocResult, DayId, PartError, PartId, PartResult};

pub struct DayResult {
    day: DayId,
    file_parts: Vec<(PathBuf, BTreeMap<PartId, Result<PartResult, PartError>>)>,
}

impl DayResult {
    pub fn new(
        day: DayId,
        file_parts: Vec<(PathBuf, BTreeMap<PartId, Result<PartResult, PartError>>)>,
    ) -> Self {
        Self { day, file_parts }
    }

    pub fn elapsed(&self) -> Duration {
        self.file_parts
            .iter()
            .map(|(_, parts)| {
                parts
                    .values()
                    .filter_map(|result| result.as_ref().ok().map(|part| part.elapsed()))
                    .sum::<Duration>()
            })
            .sum()
    }

    pub fn print(&self) -> AocResult<()> {
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

            for (part_id, result) in parts {
                match result {
                    Ok(part) => part.print()?,
                    Err(error) => io::stdout()
                        .queue(Print(format!(" X {}", error).red()))?
                        .queue(Print(" - ".dark_grey()))?
                        .queue(Print(part_id.name()))?
                        .queue(Print("\n"))?
                        .flush()?,
                }
            }
        }

        Ok(())
    }
}
