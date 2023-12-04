use core::fmt;
use std::ffi::OsStr;
use std::{path::PathBuf, time::Duration};
use termion::color::{Black, Fg, Reset};
pub struct ParserResult {
    file: PathBuf,
    elapsed: Duration,
}

impl ParserResult {
    pub fn new(file: PathBuf, elapsed: Duration) -> Self {
        Self { file, elapsed }
    }
}

impl fmt::Display for ParserResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{: <40}{}{: >20?}{}",
            self.file.file_name().and_then(OsStr::to_str).unwrap(),
            Fg(Black),
            self.elapsed,
            Fg(Reset),
        )
    }
}
