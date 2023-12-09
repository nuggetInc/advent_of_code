use core::fmt;
use std::{ffi::OsStr, path::PathBuf, time::Duration};

use termion::color::{Black, Fg, Reset};

use crate::PartId;

pub struct PartResult {
    part: PartId,
    file: PathBuf,
    answer: String,
    elapsed: Duration,
}

impl PartResult {
    pub fn new(part: PartId, file: PathBuf, answer: String, elapsed: Duration) -> Self {
        Self {
            part,
            file,
            answer,
            elapsed,
        }
    }
}

impl fmt::Display for PartResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.answer.contains('\n') {
            writeln!(
                f,
                "{}{} - {: <28}{}Answer:{}{: >36?}{}",
                self.part.name(),
                Fg(Black),
                self.file
                    .file_name()
                    .and_then(OsStr::to_str)
                    .expect("Couldn't get input filename"),
                Fg(Reset),
                Fg(Black),
                self.elapsed,
                Fg(Reset),
            )?;

            write!(f, "{}", self.answer)
        } else {
            write!(
                f,
                "{}{} - {: <27}{}Answer: {: <18}{}{: >18?}{}",
                self.part.name(),
                Fg(Black),
                self.file
                    .file_name()
                    .and_then(OsStr::to_str)
                    .expect("Couldn't get input filename"),
                Fg(Reset),
                self.answer,
                Fg(Black),
                self.elapsed,
                Fg(Reset),
            )
        }
    }
}