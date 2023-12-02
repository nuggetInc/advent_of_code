use aoc_core::Year;

pub mod day01;

pub fn year2023() -> Year {
    let mut year = Year::new();

    year.register_day("01".into(), day01::day01());

    year
}
