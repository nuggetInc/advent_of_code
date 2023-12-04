use core::fmt;
use std::{ffi::OsStr, path::PathBuf, time::Duration};

use termion::color::{Black, Fg, Reset};
pub struct PartResult {
    name: String,
    file: PathBuf,
    result: String,
    elapsed: Duration,
}

impl PartResult {
    pub fn new(name: String, file: PathBuf, result: String, elapsed: Duration) -> Self {
        Self {
            name,
            file,
            result,
            elapsed,
        }
    }
}

impl fmt::Display for PartResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{} - {: <27}{}Answer: {: <18}{}{: >18?}{}",
            self.name,
            Fg(Black),
            self.file.file_name().and_then(OsStr::to_str).unwrap(),
            Fg(Reset),
            self.result,
            Fg(Black),
            self.elapsed,
            Fg(Reset),
        )
    }
}
