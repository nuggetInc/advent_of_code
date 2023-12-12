use std::{
    fmt, fs,
    panic::{catch_unwind, RefUnwindSafe},
    path::Path,
    time::Instant,
};

use super::result::AocPartResult;
use crate::{AocError, AocResult, PartId, PartResult};

pub trait Part
where
    Self: Send + Sync,
{
    fn run(&self, file: &Path, expected: Option<String>) -> AocResult<Box<dyn PartResult>>;
}

impl<Parsed, Answer> Part for AocPart<Parsed, Answer>
where
    Parsed: Send + Sync,
    Answer: fmt::Display + Send + Sync + 'static,
{
    fn run(&self, file: &Path, expected: Option<String>) -> AocResult<Box<dyn PartResult>> {
        let instant = Instant::now();

        let result = catch_unwind(|| {
            let input = match fs::read_to_string(file) {
                Ok(input) => input,
                Err(error) => return AocResult::Err(Box::new(error)),
            };
            let parsed = (self.parser)(input);
            (self.solution)(parsed)
        });

        let elapsed = instant.elapsed();

        Ok(Box::new(AocPartResult::<Answer>::new(
            self.part,
            file.to_owned(),
            match result {
                Ok(answer) => answer,
                Err(_) => Err(Box::new(AocError::Paniced)),
            },
            expected,
            elapsed,
        )))
    }
}

pub struct AocPart<Parsed, Answer>
where
    Answer: fmt::Display,
{
    part: PartId,
    parser: Box<dyn Fn(String) -> Parsed + Send + Sync + RefUnwindSafe + 'static>,
    solution: Box<dyn Fn(Parsed) -> AocResult<Answer> + Send + Sync + RefUnwindSafe + 'static>,
}

impl<Parsed, Answer> AocPart<Parsed, Answer>
where
    Answer: fmt::Display,
{
    pub fn new(
        part: PartId,
        parser: impl Fn(String) -> Parsed + Send + Sync + RefUnwindSafe + 'static,
        solution: impl Fn(Parsed) -> AocResult<Answer> + Send + Sync + RefUnwindSafe + 'static,
    ) -> Self {
        AocPart {
            part,
            parser: Box::new(parser),
            solution: Box::new(solution),
        }
    }
}
