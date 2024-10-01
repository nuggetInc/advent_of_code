use std::collections::BTreeMap;

use crate::{Id, Year};

#[derive(Default)]
pub struct Solutions {
    years: BTreeMap<Id<Year>, Year>,
}

impl Solutions {
    pub fn add_year(&mut self, year_id: impl Into<Id<Year>>, year: Year) {
        self.years.insert(year_id.into(), year);
    }

    pub fn get_year(&self, index: Id<Year>) -> Option<&Year> {
        self.years.get(&index)
    }
}
