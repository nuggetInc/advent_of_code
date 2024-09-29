use std::{collections::BTreeMap, panic::Location};

use crate::{Id, Year};

#[derive(Default)]
pub struct Solutions {
    years: BTreeMap<Id<Year>, Year>,
}

impl Solutions {
    #[track_caller]
    pub fn add_year(&mut self, year: Year) {
        if self.years.contains_key(&year.id()) {
            eprintln!("Year {} overwritten at {}", year.id(), Location::caller())
        }

        self.years.insert(year.id(), year);
    }

    pub fn get_year(&self, index: Id<Year>) -> Option<&Year> {
        self.years.get(&index)
    }
}
