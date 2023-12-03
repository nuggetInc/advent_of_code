use std::{collections::BTreeMap, path::PathBuf, time::Duration};

pub struct YearResult {
    pub name: String,
    pub days: BTreeMap<String, DayResult>,
    pub elapsed: Duration,
}

impl YearResult {
    pub fn new(name: String, days: BTreeMap<String, DayResult>, elapsed: Duration) -> Self {
        Self {
            name,
            days,
            elapsed,
        }
    }
}

pub struct DayResult {
    pub parsers: Vec<ParserResult>,
    pub parts: Vec<PartResult>,
    pub elapsed: Duration,
}

impl DayResult {
    pub fn new(parsers: Vec<ParserResult>, parts: Vec<PartResult>, elapsed: Duration) -> Self {
        Self {
            parsers,
            parts,
            elapsed,
        }
    }
}

pub struct PartResult {
    pub name: String,
    pub file: PathBuf,
    pub result: String,
    pub elapsed: Duration,
}

impl PartResult {
    pub fn new(name: String, file: PathBuf, result: String, elapsed: Duration) -> Self {
        Self {
            name,
            file,
            result,
            elapsed,
        }
    }
}

pub struct ParserResult {
    pub file: PathBuf,
    pub elapsed: Duration,
}

impl ParserResult {
    pub fn new(file: PathBuf, elapsed: Duration) -> Self {
        Self { file, elapsed }
    }
}
