use std::{collections::BTreeMap, error::Error, fmt, panic::Location};

use super::{day::Day, result::YearResult};
use crate::{DayId, YearId};

pub struct Year {
    id: YearId,
    days: BTreeMap<DayId, Day>,
}

impl Year {
    pub fn new(id: impl Into<YearId>) -> Self {
        Self {
            id: id.into(),
            days: BTreeMap::new(),
        }
    }

    pub fn id(&self) -> YearId {
        self.id
    }

    #[track_caller]
    pub fn add_day(&mut self, day: Day) {
        if self.days.contains_key(&day.id()) {
            eprintln!(
                "{}: {} overwritten at {}",
                self.id.name(),
                day.id().name(),
                Location::caller()
            )
        }

        self.days.insert(day.id(), day);
    }

    pub fn get_day(&self, index: DayId) -> Option<&Day> {
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
