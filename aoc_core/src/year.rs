use std::collections::HashMap;

use crate::day::AocDay;

pub struct Year {
    days: HashMap<String, Box<dyn AocDay>>,
}

impl Year {
    pub fn new() -> Self {
        Self {
            days: Default::default(),
        }
    }

    pub fn register_day(&mut self, name: String, day: impl AocDay + 'static) {
        self.days.insert(name, Box::new(day));
    }

    pub fn run(&mut self) {
        for (name, day) in &mut self.days {
            println!("{name}: ");
            day.run(None);
        }
    }
}
