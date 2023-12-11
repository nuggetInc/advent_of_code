use std::{fmt, fs, io, path::Path, time::Instant};

use super::result::AocPartResult;
use crate::{AocResult, PartId, PartResult};

pub trait Part {
    fn run(&self, file: &Path, expected: Option<String>) -> io::Result<Box<dyn PartResult>>;
}

impl<Parsed, Answer> Part for AocPart<Parsed, Answer>
where
    Answer: fmt::Display + 'static,
{
    fn run(&self, file: &Path, expected: Option<String>) -> io::Result<Box<dyn PartResult>> {
        let input = fs::read_to_string(file)?;

        let instant = Instant::now();
        let parsed = (self.parser)(input);
        let answer = (self.solution)(parsed);

        Ok(Box::new(AocPartResult::new(
            self.part,
            file.to_owned(),
            answer,
            expected,
            instant.elapsed(),
        )))
    }
}

pub struct AocPart<Parsed, Answer>
where
    Answer: fmt::Display,
{
    part: PartId,
    parser: Box<dyn Fn(String) -> Parsed + 'static>,
    solution: Box<dyn Fn(Parsed) -> AocResult<Answer> + 'static>,
}

impl<Parsed, Answer> AocPart<Parsed, Answer>
where
    Answer: fmt::Display,
{
    pub fn new(
        part: PartId,
        parser: impl Fn(String) -> Parsed + 'static,
        solution: impl Fn(Parsed) -> AocResult<Answer> + 'static,
    ) -> Self {
        AocPart {
            part,
            parser: Box::new(parser),
            solution: Box::new(solution),
        }
    }
}
