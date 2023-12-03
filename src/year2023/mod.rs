use aoc_core::Year;

mod day01;
mod day02;
mod day03;

pub fn year2023() -> Year {
    let mut year = Year::new();

    year.register_day("01".into(), day01::day01());
    year.register_day("02".into(), day02::day02());
    year.register_day("03".into(), day03::day03());

    year
}
