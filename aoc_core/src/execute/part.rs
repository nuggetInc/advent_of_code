use std::{
    error::Error,
    fmt,
    panic::{catch_unwind, RefUnwindSafe},
    time::Instant,
};

use super::result::PartResult;
use crate::AocResult;

pub struct Part {
    solver: Box<dyn Fn(&String) -> AocResult<String> + RefUnwindSafe + 'static>,
}

impl Part {
    pub fn new<Answer>(
        solver: impl Fn(&String) -> AocResult<Answer> + RefUnwindSafe + 'static,
    ) -> Self
    where
        Answer: fmt::Display,
    {
        Part {
            solver: Box::new(move |s: &String| solver(s).map(|a| a.to_string())),
        }
    }

    pub fn run(&self, input: &String, expected: Option<String>) -> Result<PartResult, PartError> {
        let instant = Instant::now();

        let result = catch_unwind(|| (self.solver)(input));

        let elapsed = instant.elapsed();

        match result {
            Ok(Ok(answer)) => Ok(PartResult::new(answer.to_string(), expected, elapsed)),
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
}

impl fmt::Display for PartError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PartError::Unimplemented => write!(f, "part is not implemented"),
            PartError::Paniced => write!(f, "panic occurred during part"),
            PartError::Error(err) => write!(f, "{}", err),
        }
    }
}

impl Error for PartError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            PartError::Unimplemented => None,
            PartError::Paniced => None,
            PartError::Error(err) => Some(err.as_ref()),
        }
    }
}
