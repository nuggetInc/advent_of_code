use std::{
    error::Error,
    fmt, fs, io,
    panic::{catch_unwind, RefUnwindSafe},
    path::Path,
    time::Instant,
};

use super::result::PartResult;
use crate::{AocResult, Id};

pub struct Part {
    id: Id<Part>,
    solver: Box<dyn Fn(String) -> AocResult<String> + RefUnwindSafe + 'static>,
}

impl Part {
    pub fn new<Parsed, Answer>(
        id: Id<Part>,
        parser: impl Fn(String) -> Parsed + RefUnwindSafe + 'static,
        solution: impl Fn(Parsed) -> AocResult<Answer> + RefUnwindSafe + 'static,
    ) -> Self
    where
        Answer: fmt::Display,
    {
        Part {
            id,
            solver: Box::new(move |s: String| solution(parser(s)).map(|a| a.to_string())),
        }
    }

    pub fn id(&self) -> Id<Part> {
        self.id
    }

    pub fn run(&self, file: &Path, expected: Option<String>) -> Result<PartResult, PartError> {
        let instant = Instant::now();

        let input = fs::read_to_string(file).map_err(|err| PartError::InputFileError(err))?;
        let result = catch_unwind(|| (self.solver)(input));

        let elapsed = instant.elapsed();

        match result {
            Ok(Ok(answer)) => Ok(PartResult::new(
                self.id,
                answer.to_string(),
                expected,
                elapsed,
            )),
            Ok(Err(err)) => Err(PartError::Error(err)),
            Err(_) => Err(PartError::Paniced),
        }
    }
}

#[derive(Debug)]
pub enum PartError {
    Unimplemented,
    Paniced,
    Error(Box<dyn Error>),
    InputFileError(io::Error),
}

impl fmt::Display for PartError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PartError::Unimplemented => write!(f, "part is not implemented"),
            PartError::Paniced => write!(f, "panic occurred during part"),
            PartError::Error(err) => write!(f, "{}", err),
            PartError::InputFileError(err) => write!(f, "cannot read input file: {}", err),
        }
    }
}

impl Error for PartError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            PartError::Unimplemented => None,
            PartError::Paniced => None,
            PartError::Error(err) => Some(err.as_ref()),
            PartError::InputFileError(err) => Some(err),
        }
    }
}
