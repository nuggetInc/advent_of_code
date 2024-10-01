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
    let mut year = Year::new();

    year.add_day(1, day01::day());
    year.add_day(2, day02::day());
    year.add_day(3, day03::day());
    year.add_day(4, day04::day());
    year.add_day(5, day05::day());
    year.add_day(6, day06::day());
    year.add_day(7, day07::day());
    year.add_day(8, day08::day());
    year.add_day(9, day09::day());
    year.add_day(10, day10::day());
    year.add_day(11, day11::day());
    year.add_day(12, day12::day());
    year.add_day(13, day13::day());
    year.add_day(14, day14::day());
    year.add_day(15, day15::day());
    year.add_day(16, day16::day());
    year.add_day(17, day17::day());
    year.add_day(18, day18::day());
    year.add_day(19, day19::day());
    year.add_day(20, day20::day());
    year.add_day(21, day21::day());

    year
}
