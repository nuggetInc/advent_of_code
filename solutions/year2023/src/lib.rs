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
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;

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
    year.add_day(day11::day());
    year.add_day(day12::day());
    year.add_day(day13::day());
    year.add_day(day14::day());
    year.add_day(day15::day());
    year.add_day(day16::day());
    year.add_day(day17::day());
    year.add_day(day18::day());
    year.add_day(day19::day());
    year.add_day(day20::day());
    year.add_day(day21::day());

    year
}
