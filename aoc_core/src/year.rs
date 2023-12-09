use std::{collections::BTreeMap, io, panic::Location, time::Instant};

use crate::{day::Day, result::YearResult, DayId, YearId};

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

    pub fn run(&mut self) -> io::Result<YearResult> {
        let instant = Instant::now();
        let mut days = Vec::new();

        for (_, day) in &mut self.days {
            days.push(day.run()?);
        }

        Ok(YearResult::new(self.year.name(), days, instant.elapsed()))
    }
}
