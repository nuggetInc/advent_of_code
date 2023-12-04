mod parser;
mod part;

use std::{collections::BTreeMap, fs, path::PathBuf, time::Instant};

use crate::result::{DayResult, ParserResult, PartResult};

use self::{parser::DayParser, part::DayPart};

pub trait Day {
    fn run(&self) -> DayResult;
}

impl<T> Day for AocDay<T> {
    fn run(&self) -> DayResult {
        let day_instant = Instant::now();

        let mut parsers = Vec::new();
        let mut parts = Vec::new();

        for file in &self.files {
            let input = fs::read_to_string(file).unwrap();
            let parser_instant = Instant::now();
            let parsed = self.parser.run(input);
            parsers.push(ParserResult::new(file.to_owned(), parser_instant.elapsed()));

            for (name, part) in &self.parts {
                let part_instant = Instant::now();
                let answer = part.run(&parsed);

                parts.push(PartResult::new(
                    name.to_owned(),
                    file.to_owned(),
                    answer,
                    part_instant.elapsed(),
                ));
            }
        }

        DayResult::new(parsers, parts, day_instant.elapsed())
    }
}

pub struct AocDay<T> {
    parser: DayParser<T>,
    parts: BTreeMap<String, DayPart<T>>,
    files: Vec<PathBuf>,
}

impl<T> AocDay<T> {
    pub fn new(parser: impl Fn(String) -> T + 'static) -> Self {
        Self {
            parser: DayParser::new(parser),
            parts: BTreeMap::new(),
            files: Vec::new(),
        }
    }

    pub fn add_part(&mut self, name: String, part: impl Fn(&T) -> String + 'static) {
        self.parts.insert(name, DayPart::new(part));
    }

    pub fn add_file(&mut self, path: impl Into<PathBuf>) {
        let path: PathBuf = path.into();

        if !path.exists() {
            eprintln!("The given path '{path:?}' does not exist");
            return;
        }

        self.files.push(path);
    }
}
