use std::{collections::BTreeMap, error::Error, fmt};

use super::{day::Day, result::YearResult};
use crate::Id;

pub struct Year {
    days: BTreeMap<Id<Day>, Day>,
}

impl Year {
    pub fn new() -> Self {
        Self {
            days: BTreeMap::new(),
        }
    }

    pub fn add_day(&mut self, day_id: impl Into<Id<Day>>, day: Day) {
        self.days.insert(day_id.into(), day);
    }

    pub fn get_day(&self, index: Id<Day>) -> Option<&Day> {
        self.days.get(&index)
    }

    pub fn run(&self) -> YearResult {
        let mut days = BTreeMap::new();

        for (day_id, day) in &self.days {
            days.insert(*day_id, day.run());
        }

        YearResult::new(days)
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
