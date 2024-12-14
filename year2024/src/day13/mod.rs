use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use aoc_core::{AocResult, Day};
use itertools::Itertools;
use num::integer::{gcd, lcm};
use regex::Regex;

pub fn day() -> Day {
    let mut solution = Day::new();
    solution.part_1(part_one);
    solution.part_2(part_two);
    solution.add_file("files/test.in");
    solution.add_file("files/input.in");
    solution
}

fn parse(input: &String) -> Vec<(Position, Position, Position)> {
    let regex = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)
Button B: X\+(\d+), Y\+(\d+)
Prize: X=(\d+), Y=(\d+)",
    )
    .unwrap();

    regex
        .captures_iter(input)
        .map(|captures| {
            (
                Position::new(captures[1].parse().unwrap(), captures[2].parse().unwrap()),
                Position::new(captures[3].parse().unwrap(), captures[4].parse().unwrap()),
                Position::new(captures[5].parse().unwrap(), captures[6].parse().unwrap()),
            )
        })
        .collect_vec()
}

fn part_one(input: &String) -> AocResult<i64> {
    let machines = parse(input);

    let mut sum = 0;

    for (a, b, target) in machines {
        let mut tokens = None;

        for i in 0..=100 {
            for j in 0..=100 {
                if a * i + b * j != target {
                    continue;
                }

                if tokens.is_none_or(|t| t > i * 3 + j) {
                    tokens = Some(i * 3 + j);
                }
            }
        }

        sum += tokens.unwrap_or(0);
    }

    Ok(sum)
}

fn part_two(input: &String) -> AocResult<i64> {
    let mut machines = parse(input);

    for machine in &mut machines {
        machine.2 += Position::new(10_000_000_000_000, 10_000_000_000_000);
    }

    let mut sum = 0;

    for (a, b, c) in machines {
        let x = (b.x * c.y - b.y * c.x) / (a.x * b.y - a.y * b.x);
        let y = (c.x * a.y - c.y * a.x) / (a.x * b.y - a.y * b.x);

        if a * -x + b * -y == c {
            sum += -x * 3 + -y;
        }
    }

    Ok(sum)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl Add<Position> for Position {
    type Output = Self;

    fn add(self, rhs: Position) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Position> for Position {
    fn add_assign(&mut self, rhs: Position) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl Sub<Position> for Position {
    type Output = Self;

    fn sub(self, rhs: Position) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<Position> for Position {
    fn sub_assign(&mut self, rhs: Position) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

impl Neg for Position {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Mul<i64> for Position {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<i64> for Position {
    fn mul_assign(&mut self, rhs: i64) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
        };
    }
}
