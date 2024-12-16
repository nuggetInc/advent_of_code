use std::{
    collections::VecDeque,
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
};

use aoc_core::{AocResult, Day};
use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

pub fn day() -> Day {
    let mut solution = Day::new();
    solution.part_1(part_one);
    solution.part_2(part_two);
    solution.add_file("files/test.in");
    solution.add_file("files/test2.in");
    solution.add_file("files/input.in");
    solution
}

fn parse(input: &String) -> (Grid, Position, Position) {
    let mut start_position = Position::new(0, 0);
    let mut end_position = Position::new(0, 0);

    let mut data = Vec::new();
    let mut width = 0;
    let mut height = 0;

    for (row, line) in input.split_terminator('\n').enumerate() {
        height = row + 1;
        for (col, c) in line.char_indices() {
            width = col + 1;

            data.push(c);

            if c == 'S' {
                start_position = Position::new(col as i32, row as i32)
            } else if c == 'E' {
                end_position = Position::new(col as i32, row as i32)
            }
        }
    }

    (
        Grid::new(data, Bounds::new(width as i32, height as i32)),
        start_position,
        end_position,
    )
}

fn part_one(input: &String) -> AocResult<u32> {
    let (grid, start_position, end_position) = parse(input);

    let mut queue = VecDeque::from([(start_position, Direction::East, 0, 0)]);
    let mut queued = FxHashMap::default();
    queued.insert((start_position, Direction::East), 0);

    while let Some((position, direction, moves, turns)) = queue.pop_front() {
        if position == end_position {
            continue;
        }

        for (neighbor, neighbor_direction) in position.neighbors() {
            if neighbor_direction == -direction || !matches!(grid.get(neighbor), Some('.' | 'E')) {
                continue;
            }

            let mut turns = turns;
            if neighbor_direction != direction {
                turns += 1;
            }

            let value = turns * 1000 + moves + 1;
            if let Some(previous) = queued.get_mut(&(neighbor, neighbor_direction)) {
                if value < *previous {
                    *previous = value;
                    queue.push_back((neighbor, neighbor_direction, moves + 1, turns));
                }
            } else {
                queued.insert((neighbor, neighbor_direction), value);
                queue.push_back((neighbor, neighbor_direction, moves + 1, turns));
            }
        }
    }

    Ok(*Direction::directions()
        .into_iter()
        .filter_map(|direction| queued.get(&(end_position, direction)))
        .min()
        .unwrap())
}

fn part_two(input: &String) -> AocResult<usize> {
    let (grid, start_position, end_position) = parse(input);

    let mut queue = VecDeque::from([(start_position, Direction::East, 0, 0)]);
    let mut queued = FxHashMap::default();
    queued.insert((start_position, Direction::East), (0, Vec::new()));

    while let Some((position, direction, moves, turns)) = queue.pop_front() {
        if position == end_position {
            continue;
        }

        for (neighbor, neighbor_direction) in position.neighbors() {
            if neighbor_direction == -direction || !matches!(grid.get(neighbor), Some('.' | 'E')) {
                continue;
            }

            let mut turns = turns;
            if neighbor_direction != direction {
                turns += 1;
            }

            let value = turns * 1000 + moves + 1;
            if let Some((previous, from)) = queued.get_mut(&(neighbor, neighbor_direction)) {
                if value < *previous {
                    *previous = value;
                    from.clear();
                    from.push((position, direction));
                    queue.push_back((neighbor, neighbor_direction, moves + 1, turns));
                }
                if value == *previous {
                    from.push((position, direction));
                }
            } else {
                queued.insert(
                    (neighbor, neighbor_direction),
                    (value, Vec::from([(position, direction)])),
                );
                queue.push_back((neighbor, neighbor_direction, moves + 1, turns));
            }
        }
    }

    let mut queue = VecDeque::from([(
        end_position,
        Direction::directions()
            .into_iter()
            .min_by_key(|direction| {
                queued
                    .get(&(end_position, *direction))
                    .map(|(value, _)| *value)
                    .unwrap_or(u32::MAX)
            })
            .unwrap(),
    )]);

    let mut counted = FxHashSet::default();
    counted.insert(end_position);
    while let Some((position, direction)) = queue.pop_front() {
        let Some((value, from)) = queued.get(&(position, direction)) else {
            continue;
        };

        for (from_position, from_direction) in from.into_iter().unique() {
            let Some((from_value, _)) = queued.get(&(*from_position, *from_direction)) else {
                continue;
            };

            if from_value < value {
                counted.insert(*from_position);
                queue.push_back((*from_position, *from_direction));
            }
        }
    }

    Ok(counted.len())
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn directions() -> [Direction; 4] {
        [Self::North, Self::South, Self::East, Self::West]
    }
}

impl Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
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

    fn neighbors(self) -> [(Position, Direction); 4] {
        [
            (self + Position::new(0, -1), Direction::North),
            (self + Position::new(0, 1), Direction::South),
            (self + Position::new(1, 0), Direction::East),
            (self + Position::new(-1, 0), Direction::West),
        ]
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

struct Grid {
    data: Vec<char>,
    bounds: Bounds,
}

impl Grid {
    fn new(data: Vec<char>, bounds: Bounds) -> Self {
        Self { data, bounds }
    }

    fn get(&self, position: Position) -> Option<&char> {
        if !self.bounds.contains(position) {
            return None;
        }

        self.data
            .get((position.x + position.y * self.bounds.width) as usize)
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
