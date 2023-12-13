use std::{collections::BTreeMap, panic::Location};

use crate::{Year, YearId};

#[derive(Default)]
pub struct Solutions {
    years: BTreeMap<YearId, Year>,
}

impl Solutions {
    #[track_caller]
    pub fn add_year(&mut self, year: Year) {
        if self.years.contains_key(&year.id()) {
            eprintln!("{} overwritten at {}", year.id().name(), Location::caller(),)
        }

        self.years.insert(year.id(), year);
    }

    pub fn get_year(&self, index: YearId) -> Option<&Year> {
        self.years.get(&index)
    }
}
