use std::collections::HashSet;

use aoc_core::{AocDay, Day, YearDay};

pub fn day() -> impl Day {
    let mut solution = AocDay::new(YearDay::Day09, |x| x);
    solution.part_1(part_one);
    solution.part_2(part_two);
    solution.add_file("input.txt");
    solution
}

fn part_one(input: &String) -> String {
    let mut knots = [(0, 0); 2];

    let mut set = HashSet::new();
    set.insert(knots[1]);

    for line in input.split("\n") {
        let mut split = line.split(" ");

        let direction = match split.next() {
            Some("U") => Direction::Up,
            Some("D") => Direction::Down,
            Some("L") => Direction::Left,
            Some("R") => Direction::Right,
            value => unimplemented!("{value:?}"),
        };

        let amount: u32 = split.next().unwrap().parse().unwrap();

        for _ in 0..amount {
            knots[0].0 += match direction {
                Direction::Left => -1,
                Direction::Right => 1,
                _ => 0,
            };

            knots[0].1 += match direction {
                Direction::Up => 1,
                Direction::Down => -1,
                _ => 0,
            };

            for i in 1..2 {
                let diff_x: i32 = knots[i - 1].0 - knots[i].0;
                let diff_y: i32 = knots[i - 1].1 - knots[i].1;

                if diff_x < -1 || diff_x > 1 || diff_y < -1 || diff_y > 1 {
                    if diff_x != 0 {
                        knots[i].0 += diff_x / diff_x.abs();
                    }
                    if diff_y != 0 {
                        knots[i].1 += diff_y / diff_y.abs();
                    }
                }
            }

            set.insert(knots[1]);
        }
    }

    set.len().to_string()
}

fn part_two(input: &String) -> String {
    let mut knots = [(0, 0); 10];

    let mut set = HashSet::new();
    set.insert(knots[9]);

    for line in input.split("\n") {
        let mut split = line.split(" ");

        let direction = match split.next() {
            Some("U") => Direction::Up,
            Some("D") => Direction::Down,
            Some("L") => Direction::Left,
            Some("R") => Direction::Right,
            value => unimplemented!("{value:?}"),
        };

        let amount: u32 = split.next().unwrap().parse().unwrap();

        for _ in 0..amount {
            knots[0].0 += match direction {
                Direction::Left => -1,
                Direction::Right => 1,
                _ => 0,
            };

            knots[0].1 += match direction {
                Direction::Up => 1,
                Direction::Down => -1,
                _ => 0,
            };

            for i in 1..10 {
                let diff_x: i32 = knots[i - 1].0 - knots[i].0;
                let diff_y: i32 = knots[i - 1].1 - knots[i].1;

                if diff_x < -1 || diff_x > 1 || diff_y < -1 || diff_y > 1 {
                    if diff_x != 0 {
                        knots[i].0 += diff_x / diff_x.abs();
                    }
                    if diff_y != 0 {
                        knots[i].1 += diff_y / diff_y.abs();
                    }
                }
            }

            set.insert(knots[9]);
        }
    }

    set.len().to_string()
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
