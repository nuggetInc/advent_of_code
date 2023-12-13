use std::{
    ffi::OsStr,
    fmt,
    io::{self, Write},
    path::PathBuf,
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
        let file_name = self
            .file
            .file_name()
            .and_then(OsStr::to_str)
            .expect("Couldn't get input filename");

        match &self.result {
            Ok(answer) => {
                let answer = answer.to_string();

                if answer.contains('\n') {
                    io::stdout()
                        .queue(Print(self.part.name()))?
                        .queue(Print(format!(" - {}", file_name).dark_grey()))?
                        .queue(Print(" Answer: "))?
                        .queue(Print(format!(" - {:?}", self.elapsed).dark_grey()))?
                        .queue(Print("\n"))?
                        .queue(Print(answer))?
                        .queue(Print("\n"))?
                        .flush()
                } else {
                    if let Some(expected) = &self.expected {
                        if &answer == expected {
                            io::stdout()
                                .queue(Print(" V ".green()))?
                                .queue(Print(self.part.name()))?
                                .queue(Print(format!(" - {}", file_name).dark_grey()))?
                                .queue(Print(" Answer: "))?
                                .queue(Print(answer))?
                                .queue(Print(format!(" - {:?}", self.elapsed).dark_grey()))?
                                .queue(Print("\n"))?
                                .flush()
                        } else {
                            io::stdout()
                                .queue(Print(" X ".red()))?
                                .queue(Print(self.part.name()))?
                                .queue(Print(format!(" - {}", file_name).dark_grey()))?
                                .queue(Print(" Answer: "))?
                                .queue(Print(answer))?
                                .queue(Print(format!(" - {:?}", self.elapsed).dark_grey()))?
                                .queue(Print(" Expected: "))?
                                .queue(Print(expected))?
                                .queue(Print("\n"))?
                                .flush()
                        }
                    } else {
                        io::stdout()
                            .queue(Print(" - ".dark_grey()))?
                            .queue(Print(self.part.name()))?
                            .queue(Print(format!(" - {}", file_name).dark_grey()))?
                            .queue(Print(" Answer: "))?
                            .queue(Print(answer))?
                            .queue(Print(format!(" - {:?}", self.elapsed).dark_grey()))?
                            .queue(Print("\n"))?
                            .flush()
                    }
                }
            }
            Err(error) => io::stdout()
                .queue(Print(" X ".red()))?
                .queue(Print(self.part.name()))?
                .queue(Print(format!(" - {}", file_name).dark_grey()))?
                .queue(Print(" Error: "))?
                .queue(Print(format!(" - {:?}", self.elapsed).dark_grey()))?
                .queue(Print("\n"))?
                .queue(Print(error))?
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
    file: PathBuf,
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
        file: PathBuf,
        result: AocResult<T>,
        expected: Option<String>,
        elapsed: Duration,
    ) -> Self {
        Self {
            part,
            file,
            result,
            expected,
            elapsed,
        }
    }
}
