use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

use aoc_core::{AocResult, Day};
use itertools::Itertools;
use regex::Regex;

pub fn day() -> Day {
    let mut solution = Day::new();
    solution.part_1(part_one);
    solution.part_2(part_two);
    solution.add_file("files/test.in");
    solution.add_file("files/input.in");
    solution
}

fn parse(input: &String) -> Vec<(Position, Position)> {
    let regex = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

    regex
        .captures_iter(input)
        .map(|captures| {
            (
                Position::new(captures[1].parse().unwrap(), captures[2].parse().unwrap()),
                Position::new(captures[3].parse().unwrap(), captures[4].parse().unwrap()),
            )
        })
        .collect_vec()
}

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

fn part_one(input: &String) -> AocResult<u32> {
    let mut robots = parse(input);

    for _ in 0..100 {
        for (position, velocity) in &mut robots {
            *position += *velocity
        }
    }

    for (position, _) in &mut robots {
        position.x = (position.x % WIDTH + WIDTH) % WIDTH;
        position.y = (position.y % HEIGHT + HEIGHT) % HEIGHT;
    }

    let mut quadrants = [0; 4];

    for (position, _) in robots {
        if position.x < WIDTH / 2 {
            if position.y < HEIGHT / 2 {
                quadrants[0] += 1;
            } else if position.y > HEIGHT / 2 {
                quadrants[1] += 1;
            };
        } else if position.x > WIDTH / 2 {
            if position.y < HEIGHT / 2 {
                quadrants[2] += 1;
            } else if position.y > HEIGHT / 2 {
                quadrants[3] += 1;
            };
        };
    }

    Ok(quadrants.into_iter().product())
}

fn part_two(input: &String) -> AocResult<i32> {
    let mut robots = parse(input);

    for i in 1..=WIDTH * HEIGHT {
        for (position, velocity) in &mut robots {
            *position += *velocity;

            position.x = (position.x % WIDTH + WIDTH) % WIDTH;
            position.y = (position.y % HEIGHT + HEIGHT) % HEIGHT;
        }

        let mut ouput = String::new();

        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                if robots
                    .iter()
                    .any(|(position, _)| *position == Position::new(col, row))
                {
                    ouput += "#";
                } else {
                    ouput += ".";
                }
            }
            ouput += &format!(" {}\n", i);
        }

        if ouput.contains("########") {
            return Ok(i);
        }
    }

    todo!()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
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
