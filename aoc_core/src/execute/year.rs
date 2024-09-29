use std::{collections::BTreeMap, error::Error, fmt, panic::Location};

use super::{day::Day, result::YearResult};
use crate::Id;

pub struct Year {
    id: Id<Year>,
    days: BTreeMap<Id<Day>, Day>,
}

impl Year {
    pub fn new(id: impl Into<Id<Year>>) -> Self {
        Self {
            id: id.into(),
            days: BTreeMap::new(),
        }
    }

    pub fn id(&self) -> Id<Year> {
        self.id
    }

    #[track_caller]
    pub fn add_day(&mut self, day: Day) {
        if self.days.contains_key(&day.id()) {
            eprintln!(
                "Year {}: Day {} overwritten at {}",
                self.id,
                day.id(),
                Location::caller()
            )
        }

        self.days.insert(day.id(), day);
    }

    pub fn get_day(&self, index: Id<Day>) -> Option<&Day> {
        self.days.get(&index)
    }

    pub fn run(&self) -> YearResult {
        let mut days = BTreeMap::new();

        for day in self.days.values() {
            days.insert(day.id(), day.run());
        }

        YearResult::new(self.id, days)
    }
}

#[derive(Debug)]
pub enum YearError {
    Unimplemented,
}

impl fmt::Display for YearError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            YearError::Unimplemented => write!(f, "day is not implemented"),
        }
    }
}

impl Error for YearError {}
