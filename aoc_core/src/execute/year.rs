use std::{collections::BTreeMap, panic::Location};

use super::{day::Day, result::YearResult};
use crate::{AocResult, DayId, YearId};

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

    pub fn run(&self) -> AocResult<YearResult> {
        let mut days = Vec::new();

        for day in self.days.values() {
            days.push(day.run()?);
        }

        Ok(YearResult::new(self.id, days))
    }
}
