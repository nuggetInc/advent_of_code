use std::{
    collections::BTreeSet,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use aoc_core::{AocResult, Day};

pub fn day() -> Day {
    let mut solution = Day::new();
    solution.part_1(part_one);
    solution.part_2(part_two);
    solution.add_file("files/test.in");
    solution.add_file("files/test2.in");
    solution.add_file("files/input.in");
    solution
}

fn parse(input: &String) -> (Vec<(Position, char)>, Bounds) {
    let mut bounds = Bounds::new(0, 0);
    let antennas = input
        .split_terminator('\n')
        .enumerate()
        .flat_map(|(row, line)| {
            bounds = Bounds::new(line.len() as i32, row as i32 + 1);

            line.char_indices().filter_map(move |(col, c)| {
                (c != '.').then_some((Position::new(col as i32, row as i32), c))
            })
        })
        .collect();

    (antennas, bounds)
}

fn part_one(input: &String) -> AocResult<usize> {
    let (antennas, bounds) = parse(input);

    let mut positions = BTreeSet::new();

    for (pos, freq) in &antennas {
        for (other_pos, other_freq) in &antennas {
            if freq != other_freq || pos == other_pos {
                continue;
            }

            let diff = *pos - *other_pos;
            let antinode = *pos + diff;

            if bounds.contains(antinode) {
                positions.insert(antinode);
            }
        }
    }

    Ok(positions.len())
}

fn part_two(input: &String) -> AocResult<usize> {
    let (antennas, bounds) = parse(input);

    let mut positions = BTreeSet::new();

    for (pos, freq) in &antennas {
        for (other_pos, other_freq) in &antennas {
            if freq != other_freq || pos == other_pos {
                continue;
            }

            let diff = *pos - *other_pos;
            let mut antinode = *pos;

            while bounds.contains(antinode) {
                positions.insert(antinode);
                antinode = antinode + diff
            }
        }
    }

    Ok(positions.len())
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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

struct Bounds {
    width: i32,
    height: i32,
}

impl Bounds {
    fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }

    fn contains(&self, position: Position) -> bool {
        return position.x >= 0
            && position.y >= 0
            && position.x < self.width
            && position.y < self.height;
    }
}
