use std::collections::HashSet;

use aoc_core::{AocResult, Day};

pub fn day() -> Day {
    let mut solution = Day::new(9);
    solution.part_1(|x| x, part_one);
    solution.part_2(|x| x, part_two);
    solution.add_file("files/input.in");
    solution
}

fn part_one(input: String) -> AocResult<usize> {
    let mut knots = [(0, 0); 2];

    let mut set = HashSet::new();
    set.insert(knots[1]);

    for line in input.split_terminator('\n') {
        let mut split = line.split(' ');

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

                if !(-1..=1).contains(&diff_x) || !(-1..=1).contains(&diff_y) {
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

    Ok(set.len())
}

fn part_two(input: String) -> AocResult<usize> {
    let mut knots = [(0, 0); 10];

    let mut set = HashSet::new();
    set.insert(knots[9]);

    for line in input.split_terminator('\n') {
        let mut split = line.split(' ');

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

                if !(-1..=1).contains(&diff_x) || !(-1..=1).contains(&diff_y) {
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

    Ok(set.len())
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
