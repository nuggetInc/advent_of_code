mod yearday;
use std::{collections::BTreeMap, io, panic::Location, time::Instant};

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

    #[track_caller]
    pub fn add_day(&mut self, day: Day) {
        if self.days.contains_key(&day.day()) {
            eprintln!(
                "{}: {} overwritten at {}",
                self.name,
                day.day(),
                Location::caller()
            )
        }

        self.days.insert(day.day(), day);
    }

    pub fn get_day(&self, index: &YearDay) -> Option<&Day> {
        self.days.get(index)
    }

    pub fn run(&mut self) -> io::Result<YearResult> {
        let instant = Instant::now();
        let mut days = Vec::new();

        for (_, day) in &mut self.days {
            days.push(day.run()?);
        }

        Ok(YearResult::new(
            self.name.to_owned(),
            days,
            instant.elapsed(),
        ))
    }
}
