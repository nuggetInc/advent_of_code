use aoc_core::Year;

mod day1;

pub fn year() -> Year {
    let mut year = Year::new();

    year.add_day(1, day1::day());

    year
}
