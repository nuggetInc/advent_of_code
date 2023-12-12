use std::{collections::BTreeMap, panic::Location, thread};

use super::{day::Day, result::YearResult};
use crate::{AocResult, DayId, YearId};

pub struct Year {
    year: YearId,
    days: BTreeMap<DayId, Day>,
}

impl Year {
    pub fn new(year: impl Into<YearId>) -> Self {
        Self {
            year: year.into(),
            days: BTreeMap::new(),
        }
    }

    #[track_caller]
    pub fn add_day(&mut self, day: Day) {
        if self.days.contains_key(&day.day()) {
            eprintln!(
                "{}: {} overwritten at {}",
                self.year.name(),
                day.day().name(),
                Location::caller()
            )
        }

        self.days.insert(day.day(), day);
    }

    pub fn get_day(&self, index: DayId) -> Option<&Day> {
        self.days.get(&index)
    }

    pub fn run(&mut self) -> AocResult<YearResult> {
        thread::scope(|scope| -> AocResult<YearResult> {
            let mut handles = Vec::new();
            let mut days = Vec::new();

            for day in self.days.values_mut() {
                let handle = scope.spawn(|| day.run());
                handles.push(handle);
            }

            for handle in handles {
                days.push(handle.join().unwrap()?);
            }

            Ok(YearResult::new(self.year, days))
        })
    }
}
