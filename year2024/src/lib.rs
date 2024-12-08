use aoc_core::Year;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

pub fn year() -> Year {
    let mut year = Year::new();

    year.add_day(1, day1::day());
    year.add_day(2, day2::day());
    year.add_day(3, day3::day());
    year.add_day(4, day4::day());
    year.add_day(5, day5::day());
    year.add_day(6, day6::day());
    year.add_day(7, day7::day());
    year.add_day(8, day8::day());

    year
}
