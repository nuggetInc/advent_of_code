use std::{fs, io, path::Path, time::Instant};

use super::result::PartResult;
use crate::PartId;

pub trait Part {
    fn run(&self, file: &Path) -> io::Result<PartResult>;
}

impl<T> Part for AocPart<T> {
    fn run(&self, file: &Path) -> io::Result<PartResult> {
        let input = fs::read_to_string(file)?;

        let instant = Instant::now();
        let parsed = (self.parser)(input);
        let answer = (self.solution)(parsed);

        Ok(PartResult::new(
            self.part,
            file.to_owned(),
            answer,
            instant.elapsed(),
        ))
    }
}

pub struct AocPart<T> {
    part: PartId,
    parser: Box<dyn Fn(String) -> T + 'static>,
    solution: Box<dyn Fn(T) -> String + 'static>,
}

impl<T> AocPart<T> {
    pub fn new(
        part: PartId,
        parser: impl Fn(String) -> T + 'static,
        solution: impl Fn(T) -> String + 'static,
    ) -> Self {
        AocPart {
            part,
            parser: Box::new(parser),
            solution: Box::new(solution),
        }
    }
}
