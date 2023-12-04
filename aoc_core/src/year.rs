use std::{collections::BTreeMap, time::Instant};

use crate::{day::Day, result::YearResult, YearDay};

pub struct Year {
    name: String,
    days: BTreeMap<YearDay, Box<dyn Day>>,
}

impl Year {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            days: BTreeMap::new(),
        }
    }

    pub fn add_day(&mut self, index: YearDay, day: impl Day + 'static) {
        self.days.insert(index, Box::new(day));
    }

    pub fn get_day(&self, index: &YearDay) -> Option<&dyn Day> {
        self.days.get(index).map(Box::as_ref)
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
