mod daypart;
mod part;

use std::{
    collections::BTreeMap,
    panic::Location,
    path::{Path, PathBuf},
    time::Instant,
};

pub use daypart::DayPart;

use self::part::{AocPart, Part};
use crate::{result::DayResult, year::YearDay};

pub struct Day {
    day: YearDay,
    parts: BTreeMap<DayPart, Box<dyn Part>>,
    files: Vec<PathBuf>,
}

impl Day {
    pub fn new(day: YearDay) -> Self {
        Self {
            day,
            parts: BTreeMap::new(),
            files: Vec::new(),
        }
    }

    pub fn run(&self) -> DayResult {
        let instant = Instant::now();

        let mut parts = Vec::new();

        for file in &self.files {
            for (_, part) in &self.parts {
                parts.push(part.run(file));
            }
        }

        DayResult::new(self.day, parts, instant.elapsed())
    }

    pub fn part_1<T: 'static>(
        &mut self,
        parser: impl Fn(String) -> T + 'static,
        part: impl Fn(T) -> String + 'static,
    ) {
        self.parts.insert(
            DayPart::Part1,
            Box::new(AocPart::new(DayPart::Part1, parser, part)),
        );
    }

    pub fn part_2<T: 'static>(
        &mut self,
        parser: impl Fn(String) -> T + 'static,
        part: impl Fn(T) -> String + 'static,
    ) {
        self.parts.insert(
            DayPart::Part2,
            Box::new(AocPart::new(DayPart::Part2, parser, part)),
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
