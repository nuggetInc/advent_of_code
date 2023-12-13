use std::{
    fmt, fs,
    panic::{catch_unwind, RefUnwindSafe},
    path::Path,
    time::Instant,
};

use super::result::PartResult;
use crate::{AocError, AocResult, PartId};

pub trait Part {
    fn run(&self, file: &Path, expected: Option<String>) -> AocResult<PartResult>;
}

impl<Parsed, Answer> Part for AocPart<Parsed, Answer>
where
    Answer: fmt::Display + 'static,
{
    fn run(&self, file: &Path, expected: Option<String>) -> AocResult<PartResult> {
        let instant = Instant::now();

        let result = catch_unwind(|| {
            let input = fs::read_to_string(file)?;
            let parsed = (self.parser)(input);
            (self.solution)(parsed)
        });

        let elapsed = instant.elapsed();

        Ok(PartResult::new(
            self.id,
            result
                .unwrap_or_else(|_| Err(Box::new(AocError::Paniced)))
                .map(|a| a.to_string()),
            expected,
            elapsed,
        ))
    }
}

pub struct AocPart<Parsed, Answer>
where
    Answer: fmt::Display,
{
    id: PartId,
    parser: Box<dyn Fn(String) -> Parsed + RefUnwindSafe + 'static>,
    solution: Box<dyn Fn(Parsed) -> AocResult<Answer> + RefUnwindSafe + 'static>,
}

impl<Parsed, Answer> AocPart<Parsed, Answer>
where
    Answer: fmt::Display,
{
    pub fn new(
        id: PartId,
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
