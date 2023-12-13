use std::{
    io::{self, Write},
    time::Duration,
};

use crossterm::{
    style::{Print, Stylize},
    QueueableCommand,
};

use crate::{AocResult, PartId};

pub struct PartResult {
    part: PartId,
    result: AocResult<String>,
    expected: Option<String>,
    elapsed: Duration,
}

impl PartResult {
    pub fn new(
        part: PartId,
        result: AocResult<String>,
        expected: Option<String>,
        elapsed: Duration,
    ) -> Self {
        Self {
            part,
            result,
            expected,
            elapsed,
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }

    pub fn result(self) -> AocResult<String> {
        self.result
    }

    pub fn print(&self) -> AocResult<()> {
        match &self.result {
            Ok(answer) => {
                if let Some(expected) = &self.expected {
                    if answer == expected {
                        io::stdout()
                            .queue(Print(" V ".green()))?
                            .queue(Print(answer))?
                            .queue(Print(" - ".dark_grey()))?
                            .queue(Print(self.part.name()))?
                            .queue(Print(format!(" - {:?}", self.elapsed).dark_grey()))?
                            .queue(Print("\n"))?
                            .flush()?;
                    } else {
                        io::stdout()
                            .queue(Print(" X ".red()))?
                            .queue(Print(answer))?
                            .queue(Print(" - ".dark_grey()))?
                            .queue(Print(expected.to_owned()))?
                            .queue(Print(" - ".dark_grey()))?
                            .queue(Print(self.part.name()))?
                            .queue(Print(format!(" - {:?}", self.elapsed).dark_grey()))?
                            .queue(Print("\n"))?
                            .flush()?;
                    }
                } else {
                    io::stdout()
                        .queue(Print(" - ".dark_grey()))?
                        .queue(Print(answer))?
                        .queue(Print(" - ".dark_grey()))?
                        .queue(Print(self.part.name()))?
                        .queue(Print(format!(" - {:?}", self.elapsed).dark_grey()))?
                        .queue(Print("\n"))?
                        .flush()?;
                }
            }
            Err(error) => io::stdout()
                .queue(Print(format!(" X {}", error).red()))?
                .queue(Print(" - ".dark_grey()))?
                .queue(Print(self.part.name()))?
                .queue(Print(format!(" - {:?}", self.elapsed).dark_grey()))?
                .queue(Print("\n"))?
                .flush()?,
        }

        Ok(())
    }
}
