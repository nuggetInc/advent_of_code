use std::{collections::BTreeMap, time::Instant};

use crate::{day::Day, result::YearResult};

pub struct Year {
    name: String,
    days: BTreeMap<String, Box<dyn Day>>,
}

impl Year {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            days: BTreeMap::new(),
        }
    }

    pub fn register_day(&mut self, name: &str, day: impl Day + 'static) {
        self.days.insert(name.to_owned(), Box::new(day));
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
