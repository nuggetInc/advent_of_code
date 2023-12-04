use aoc_core::{Year, YearDay};

mod day01;
mod day02;
mod day03;
mod day04;

pub fn year2023() -> Year {
    let mut year = Year::new("Advent of Code 2023");

    year.add_day(YearDay::Day01, day01::day());
    year.add_day(YearDay::Day02, day02::day());
    year.add_day(YearDay::Day03, day03::day());
    year.add_day(YearDay::Day04, day04::day());

    year
}