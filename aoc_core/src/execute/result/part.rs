use core::fmt;
use std::{ffi::OsStr, path::PathBuf, time::Duration};

use termion::color::{Black, Fg, Green, Red, Reset};

use crate::PartId;

pub struct PartResult {
    part: PartId,
    file: PathBuf,
    answer: String,
    expected: Option<String>,
    elapsed: Duration,
}

impl PartResult {
    pub fn new(
        part: PartId,
        file: PathBuf,
        answer: String,
        expected: Option<String>,
        elapsed: Duration,
    ) -> Self {
        Self {
            part,
            file,
            answer,
            expected,
            elapsed,
        }
    }
}

impl fmt::Display for PartResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let file_name = self
            .file
            .file_name()
            .and_then(OsStr::to_str)
            .expect("Couldn't get input filename");

        if self.answer.contains('\n') {
            writeln!(
                f,
                "{}{} - {: <28}{}Answer:{}{: >36?}{}",
                self.part.name(),
                Fg(Black),
                file_name,
                Fg(Reset),
                Fg(Black),
                self.elapsed,
                Fg(Reset),
            )?;

            write!(f, "{}", self.answer)
        } else {
            if let Some(expected) = &self.expected {
                if &self.answer == expected {
                    write!(
                        f,
                        "{} V {}{}{} - {: <24}{}Answer: {: <18}{}{: >18?}{}",
                        Fg(Green),
                        Fg(Reset),
                        self.part.name(),
                        Fg(Black),
                        file_name,
                        Fg(Reset),
                        self.answer,
                        Fg(Black),
                        self.elapsed,
                        Fg(Reset),
                    )
                } else {
                    write!(
                        f,
                        "{} X {}{}{} - {: <24}{}Answer: {: <18}{}{: >18?}{} Expected: {}",
                        Fg(Red),
                        Fg(Reset),
                        self.part.name(),
                        Fg(Black),
                        file_name,
                        Fg(Reset),
                        self.answer,
                        Fg(Black),
                        self.elapsed,
                        Fg(Reset),
                        expected,
                    )
                }
            } else {
                write!(
                    f,
                    "{} - {}{}{} - {: <24}{}Answer: {: <18}{}{: >18?}{}",
                    Fg(Black),
                    Fg(Reset),
                    self.part.name(),
                    Fg(Black),
                    file_name,
                    Fg(Reset),
                    self.answer,
                    Fg(Black),
                    self.elapsed,
                    Fg(Reset),
                )
            }
        }
    }
}
