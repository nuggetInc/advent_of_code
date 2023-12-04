use aoc_core::Year;

mod day01;
mod day02;
mod day03;
mod day04;

pub fn year2023() -> Year {
    let mut year = Year::new("Advent of Code 2023");

    year.register_day("Day 01", day01::day01());
    year.register_day("Day 02", day02::day02());
    year.register_day("Day 03", day03::day03());
    year.register_day("Day 04", day04::day04());

    year
}
