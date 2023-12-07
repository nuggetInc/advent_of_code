mod yearday;
use std::{collections::BTreeMap, time::Instant};

pub use yearday::YearDay;

use crate::{day::Day, result::YearResult};

pub struct Year {
    name: String,
    days: BTreeMap<YearDay, Day>,
}

impl Year {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            days: BTreeMap::new(),
        }
    }

    pub fn add_day(&mut self, index: YearDay, day: Day) {
        self.days.insert(index, day);
    }

    pub fn get_day(&self, index: &YearDay) -> Option<&Day> {
        self.days.get(index)
    }

    pub fn run(&mut self) -> YearResult {
        let instant = Instant::now();
        let mut days = Vec::new();

        for (_, day) in &mut self.days {
            days.push(day.run());
        }

        YearResult::new(self.name.to_owned(), days, instant.elapsed())
    }
}
