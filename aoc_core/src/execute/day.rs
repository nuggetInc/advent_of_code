use std::{
    collections::BTreeMap,
    fmt, fs,
    panic::{Location, RefUnwindSafe},
    path::{Path, PathBuf},
};

use super::{
    part::{AocPart, Part},
    result::DayResult,
};
use crate::{AocResult, DayId, PartId};

pub struct Day {
    day: DayId,
    parts: BTreeMap<PartId, Box<dyn Part>>,
    files: Vec<PathBuf>,
}

impl Day {
    pub fn new(day: impl Into<DayId>) -> Self {
        Self {
            day: day.into(),
            parts: BTreeMap::new(),
            files: Vec::new(),
        }
    }

    pub fn day(&self) -> DayId {
        self.day
    }

    pub fn part_count(&self) -> usize {
        self.parts.len()
    }

    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    pub fn run(&self) -> AocResult<DayResult> {
        let mut parts = Vec::new();

        for input_file in &self.files {
            let mut output_file = input_file.clone();
            output_file.set_extension("out");

            let output = output_file
                .exists()
                .then(|| fs::read_to_string(output_file))
                .unwrap_or(Ok(String::new()))?;

            let mut expected_answers = output.split_terminator('\n');

            for part in self.parts.values() {
                let expected = expected_answers.next().map(str::to_owned);

                parts.push(part.run(input_file, expected)?);
            }
        }

        Ok(DayResult::new(self.day, parts))
    }

    pub fn part_1<Parsed: 'static, Answer: fmt::Display + 'static>(
        &mut self,
        parser: impl Fn(String) -> Parsed + RefUnwindSafe + 'static,
        part: impl Fn(Parsed) -> AocResult<Answer> + RefUnwindSafe + 'static,
    ) {
        self.parts.insert(
            PartId::PART_1,
            Box::new(AocPart::new(PartId::PART_1, parser, part)),
        );
    }

    pub fn part_2<Parsed: 'static, Answer: fmt::Display + 'static>(
        &mut self,
        parser: impl Fn(String) -> Parsed + RefUnwindSafe + 'static,
        part: impl Fn(Parsed) -> AocResult<Answer> + RefUnwindSafe + 'static,
    ) {
        self.parts.insert(
            PartId::PART_2,
            Box::new(AocPart::new(PartId::PART_2, parser, part)),
        );
    }

    #[track_caller]
    pub fn add_file(&mut self, path: impl AsRef<Path>) {
        let mut full_path = PathBuf::from(Location::caller().file());
        full_path.pop();
        full_path.push(path);

        if !full_path.exists() {
            eprintln!("The given path '{full_path:?}' does not exist");
            return;
        }

        self.files.push(full_path);
    }
}
