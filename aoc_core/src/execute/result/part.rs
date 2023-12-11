use std::{ffi::OsStr, fmt, path::PathBuf, time::Duration};

use termion::color::{Black, Fg, Green, Red, Reset};

use crate::{AocResult, PartId};

pub trait PartResult
where
    Self: fmt::Display,
{
}

impl<T> PartResult for AocPartResult<T> where T: fmt::Display {}

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

impl<T> fmt::Display for AocPartResult<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let file_name = self
            .file
            .file_name()
            .and_then(OsStr::to_str)
            .expect("Couldn't get input filename");

        match &self.result {
            Ok(answer) => {
                let answer = answer.to_string();

                if answer.contains('\n') {
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

                    write!(f, "{}", answer)
                } else {
                    if let Some(expected) = &self.expected {
                        if &answer == expected {
                            write!(
                                f,
                                "{} V {}{}{} - {: <24}{}Answer: {: <18}{}{: >18?}{}",
                                Fg(Green),
                                Fg(Reset),
                                self.part.name(),
                                Fg(Black),
                                file_name,
                                Fg(Reset),
                                answer,
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
                                answer,
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
                            answer,
                            Fg(Black),
                            self.elapsed,
                            Fg(Reset),
                        )
                    }
                }
            }
            Err(error) => {
                writeln!(
                    f,
                    "{} X {}{}{} - {: <24}{}Error:{}{: >38?}{}",
                    Fg(Red),
                    Fg(Reset),
                    self.part.name(),
                    Fg(Black),
                    file_name,
                    Fg(Reset),
                    Fg(Black),
                    self.elapsed,
                    Fg(Reset),
                )?;

                write!(f, "{}", error)
            }
        }
    }
}
