use aoc_core::Year;

mod day01;
mod day02;

pub fn year2023() -> Year {
    let mut year = Year::new();

    year.register_day("01".into(), day01::day01());
    year.register_day("02".into(), day02::day02());

    year
}
