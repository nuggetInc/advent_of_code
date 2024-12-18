use aoc_core::Year;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub fn year() -> Year {
    let mut year = Year::new();

    year.add_day(1, day1::day());
    year.add_day(10, day10::day());
    year.add_day(11, day11::day());
    year.add_day(12, day12::day());
    year.add_day(13, day13::day());
    year.add_day(14, day14::day());
    year.add_day(15, day15::day());
    year.add_day(16, day16::day());
    year.add_day(2, day2::day());
    year.add_day(3, day3::day());
    year.add_day(4, day4::day());
    year.add_day(5, day5::day());
    year.add_day(6, day6::day());
    year.add_day(7, day7::day());
    year.add_day(8, day8::day());
    year.add_day(9, day9::day());

    year
}
