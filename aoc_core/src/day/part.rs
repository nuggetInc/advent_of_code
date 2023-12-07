use std::{fs, path::PathBuf, time::Instant};

use super::daypart::DayPart;
use crate::result::PartResult;

pub trait Part {
    fn run(&self, file: &PathBuf) -> PartResult;
}

impl<T> Part for AocPart<T> {
    fn run(&self, file: &PathBuf) -> PartResult {
        let input = fs::read_to_string(file).unwrap();

        let instant = Instant::now();
        let parsed = (self.parser)(input);
        let answer = (self.solution)(parsed);

        PartResult::new(self.part.clone(), file.clone(), answer, instant.elapsed())
    }
}

pub struct AocPart<T> {
    part: DayPart,
    parser: Box<dyn Fn(String) -> T + 'static>,
    solution: Box<dyn Fn(T) -> String + 'static>,
}

impl<T> AocPart<T> {
    pub fn new(
        part: DayPart,
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
