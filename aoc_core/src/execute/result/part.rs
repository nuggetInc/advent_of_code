use std::{
    fmt,
    io::{self, Write},
    time::Duration,
};

use crossterm::{
    style::{Print, Stylize},
    QueueableCommand,
};

use crate::{AocResult, PartId};

pub trait PartResult {
    fn elapsed(&self) -> Duration;
    fn print(&self) -> io::Result<()>;
}

impl<T> PartResult for AocPartResult<T>
where
    T: fmt::Display,
{
    fn elapsed(&self) -> Duration {
        self.elapsed
    }

    fn print(&self) -> io::Result<()> {
        match &self.result {
            Ok(answer) => {
                let answer = answer.to_string();

                if let Some(expected) = &self.expected {
                    if &answer == expected {
                        io::stdout()
                            .queue(Print(" V ".green()))?
                            .queue(Print(answer))?
                            .queue(Print(" - ".dark_grey()))?
                            .queue(Print(self.part.name()))?
                            .queue(Print(format!(" - {:?}", self.elapsed).dark_grey()))?
                            .queue(Print("\n"))?
                            .flush()
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
                            .flush()
                    }
                } else {
                    io::stdout()
                        .queue(Print(" - ".dark_grey()))?
                        .queue(Print(answer))?
                        .queue(Print(" - ".dark_grey()))?
                        .queue(Print(self.part.name()))?
                        .queue(Print(format!(" - {:?}", self.elapsed).dark_grey()))?
                        .queue(Print("\n"))?
                        .flush()
                }
            }
            Err(error) => io::stdout()
                .queue(Print(format!(" X {}", error).red()))?
                .queue(Print(" - ".dark_grey()))?
                .queue(Print(self.part.name()))?
                .queue(Print(format!(" - {:?}", self.elapsed).dark_grey()))?
                .queue(Print("\n"))?
                .flush(),
        }
    }
}

pub struct AocPartResult<T>
where
    T: fmt::Display,
{
    part: PartId,
    result: AocResult<T>,
    expected: Option<String>,
    elapsed: Duration,
}

impl<T> AocPartResult<T>
where
    T: fmt::Display,
{
    pub fn new(
        part: PartId,
        result: AocResult<T>,
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
}
