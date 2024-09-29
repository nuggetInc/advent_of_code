use std::{
    error::Error,
    fmt, fs, io,
    panic::{catch_unwind, RefUnwindSafe},
    path::Path,
    time::Instant,
};

use super::result::PartResult;
use crate::{AocResult, Id};

pub trait Part {
    fn id(&self) -> Id<AocPart>;
    fn run(&self, file: &Path, expected: Option<String>) -> Result<PartResult, PartError>;
}

impl<Parsed, Answer> Part for AocPart<Parsed, Answer>
where
    Answer: fmt::Display + 'static,
{
    fn id(&self) -> Id<AocPart> {
        self.id
    }

    fn run(&self, file: &Path, expected: Option<String>) -> Result<PartResult, PartError> {
        let instant = Instant::now();

        let input = fs::read_to_string(file).map_err(|err| PartError::InputFileError(err))?;
        let result = catch_unwind(|| {
            let parsed = (self.parser)(input);
            (self.solution)(parsed)
        });

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

pub struct AocPart<Parsed = String, Answer = String>
where
    Answer: fmt::Display,
{
    id: Id<AocPart>,
    parser: Box<dyn Fn(String) -> Parsed + RefUnwindSafe + 'static>,
    solution: Box<dyn Fn(Parsed) -> AocResult<Answer> + RefUnwindSafe + 'static>,
}

impl<Parsed, Answer> AocPart<Parsed, Answer>
where
    Answer: fmt::Display,
{
    pub fn new(
        id: Id<AocPart>,
        parser: impl Fn(String) -> Parsed + RefUnwindSafe + 'static,
        solution: impl Fn(Parsed) -> AocResult<Answer> + RefUnwindSafe + 'static,
    ) -> Self {
        AocPart {
            id,
            parser: Box::new(parser),
            solution: Box::new(solution),
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
