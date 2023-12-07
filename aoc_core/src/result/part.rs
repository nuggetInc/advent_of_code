use core::fmt;
use std::{ffi::OsStr, path::PathBuf, time::Duration};

use termion::color::{Black, Fg, Reset};

use crate::day::DayPart;
pub struct PartResult {
    part: DayPart,
    file: PathBuf,
    answer: String,
    elapsed: Duration,
}

impl PartResult {
    pub fn new(part: DayPart, file: PathBuf, answer: String, elapsed: Duration) -> Self {
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
        write!(
            f,
            "{}{} - {: <27}{}Answer: {: <18}{}{: >18?}{}",
            self.part,
            Fg(Black),
            self.file.file_name().and_then(OsStr::to_str).unwrap(),
            Fg(Reset),
            self.answer,
            Fg(Black),
            self.elapsed,
            Fg(Reset),
        )
    }
}
