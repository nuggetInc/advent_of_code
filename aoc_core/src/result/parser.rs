use core::fmt;
use std::{ffi::OsStr, path::PathBuf, time::Duration};

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
            "Parsing{} - {: <30}{: >40?}{}",
            Fg(Black),
            self.file.file_name().and_then(OsStr::to_str).unwrap(),
            self.elapsed,
            Fg(Reset),
        )
    }
}
