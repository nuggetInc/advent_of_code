use aoc_core::{Year, YearDay};

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

pub fn year() -> Year {
    let mut year = Year::new("Advent of Code 2023");

    year.add_day(YearDay::Day01, day01::day());
    year.add_day(YearDay::Day02, day02::day());
    year.add_day(YearDay::Day03, day03::day());
    year.add_day(YearDay::Day04, day04::day());
    year.add_day(YearDay::Day05, day05::day());
    year.add_day(YearDay::Day06, day06::day());
    year.add_day(YearDay::Day07, day07::day());
    year.add_day(YearDay::Day08, day08::day());
    year.add_day(YearDay::Day09, day09::day());
    year.add_day(YearDay::Day10, day10::day());
    year.add_day(YearDay::Day11, day11::day());
    year.add_day(YearDay::Day12, day12::day());
    year.add_day(YearDay::Day13, day13::day());
    year.add_day(YearDay::Day14, day14::day());
    year.add_day(YearDay::Day15, day15::day());
    year.add_day(YearDay::Day16, day16::day());

    year
}
