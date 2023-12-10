use aoc_core::Year;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

pub fn year() -> Year {
    let mut year = Year::new(2023);

    year.add_day(day01::day());
    year.add_day(day02::day());
    year.add_day(day03::day());
    year.add_day(day04::day());
    year.add_day(day05::day());
    year.add_day(day06::day());
    year.add_day(day07::day());
    year.add_day(day08::day());
    year.add_day(day09::day());
    year.add_day(day10::day());

    year
}
