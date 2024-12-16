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

fn parse(input: &String) -> (Vec<Node>, usize, usize) {
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

    let grid = Grid::new(data, Bounds::new(width as i32, height as i32));

    let mut nodes = Vec::default();
    let mut node_indices = FxHashMap::default();

    for row in 1..grid.bounds.height - 1 {
        for col in 1..grid.bounds.width - 1 {
            let position = Position::new(col, row);
            if matches!(grid.get(Position::new(col, row)), Some('#')) {
                continue;
            }

            let north = matches!(grid.get(Position::new(col, row - 1)), Some('.' | 'S' | 'E'));
            let south = matches!(grid.get(Position::new(col, row + 1)), Some('.' | 'S' | 'E'));
            let east = matches!(grid.get(Position::new(col + 1, row)), Some('.' | 'S' | 'E'));
            let west = matches!(grid.get(Position::new(col - 1, row)), Some('.' | 'S' | 'E'));

            if (north ^ south) || (east ^ west) || (north & south & east & west) {
                node_indices.insert(position, nodes.len());
                nodes.push(Node {
                    neighbors: [None; 4],
                });
            }
        }
    }

    for row in 1..grid.bounds.height - 1 {
        let mut previous = None;
        for col in 1..grid.bounds.width - 1 {
            let position = Position::new(col, row);
            if !node_indices.contains_key(&position) {
                continue;
            }

            if let Some(previous_position) = previous {
                let index = *node_indices.get(&position).unwrap();
                let previous_index = *node_indices.get(&previous_position).unwrap();

                let distance = position.distance(previous_position) as u32;

                nodes.get_mut(index).unwrap().neighbors[Direction::West as usize] =
                    Some((previous_index, distance));
                nodes.get_mut(previous_index).unwrap().neighbors[Direction::East as usize] =
                    Some((index, distance));
            }

            if matches!(grid.get(Position::new(col + 1, row)), Some('.' | 'S' | 'E')) {
                previous = Some(position);
            } else {
                previous = None;
            }
        }
    }

    for col in 1..grid.bounds.width - 1 {
        let mut previous = None;
        for row in 1..grid.bounds.height - 1 {
            let position = Position::new(col, row);
            if !node_indices.contains_key(&position) {
                continue;
            }

            if let Some(previous_position) = previous {
                let index = *node_indices.get(&position).unwrap();
                let previous_index = *node_indices.get(&previous_position).unwrap();

                let distance = position.distance(previous_position) as u32;

                nodes.get_mut(index).unwrap().neighbors[Direction::North as usize] =
                    Some((previous_index, distance));
                nodes.get_mut(previous_index).unwrap().neighbors[Direction::South as usize] =
                    Some((index, distance));
            }

            if matches!(grid.get(Position::new(col, row + 1)), Some('.' | 'S' | 'E')) {
                previous = Some(position);
            } else {
                previous = None;
            }
        }
    }

    let start_index = *node_indices.get(&start_position).unwrap();
    let end_index = *node_indices.get(&end_position).unwrap();

    (nodes, start_index, end_index)
}

fn part_one(input: &String) -> AocResult<u32> {
    let (nodes, start_index, end_index) = parse(input);

    let mut queue = VecDeque::from([(start_index, Direction::East, 0, 0)]);
    let mut queued = FxHashMap::default();
    queued.insert((start_index, Direction::East), 0);

    while let Some((index, direction, moves, turns)) = queue.pop_front() {
        if index == end_index {
            continue;
        }

        for new_direction in Direction::directions() {
            if new_direction == -direction {
                continue;
            }

            let Some((new_index, distance)) = nodes[index].neighbors[new_direction as usize] else {
                continue;
            };

            let mut turns = turns;
            if new_direction != direction {
                turns += 1;
            }

            let value = turns * 1000 + moves + distance;
            if let Some(previous) = queued.get_mut(&(new_index, new_direction)) {
                if value < *previous {
                    *previous = value;
                    queue.push_back((new_index, new_direction, moves + distance, turns));
                }
            } else {
                queued.insert((new_index, new_direction), value);
                queue.push_back((new_index, new_direction, moves + distance, turns));
            }
        }
    }

    Ok(*Direction::directions()
        .into_iter()
        .filter_map(|direction| queued.get(&(end_index, direction)))
        .min()
        .unwrap())
}

fn part_two(input: &String) -> AocResult<u32> {
    let (nodes, start_index, end_index) = parse(input);

    let mut queue = VecDeque::from([(start_index, Direction::East, 0, 0)]);
    let mut queued = FxHashMap::default();
    queued.insert((start_index, Direction::East), (0, Vec::new()));

    while let Some((index, direction, moves, turns)) = queue.pop_front() {
        if index == end_index {
            continue;
        }

        for new_direction in Direction::directions() {
            if new_direction == -direction {
                continue;
            }

            let Some((new_index, distance)) = nodes[index].neighbors[new_direction as usize] else {
                continue;
            };

            let moves = moves + distance;
            let mut turns = turns;
            if new_direction != direction {
                turns += 1;
            }

            let value = turns * 1000 + moves;
            if let Some((previous, from)) = queued.get_mut(&(new_index, new_direction)) {
                if value < *previous {
                    *previous = value;
                    from.clear();
                    from.push((index, direction));
                    queue.push_back((new_index, new_direction, moves, turns));
                } else if value == *previous {
                    from.push((index, direction));
                }
            } else {
                queued.insert(
                    (new_index, new_direction),
                    (value, Vec::from([(index, direction)])),
                );
                queue.push_back((new_index, new_direction, moves, turns));
            }
        }
    }

    let mut queue = VecDeque::from([(
        end_index,
        Direction::directions()
            .into_iter()
            .min_by_key(|direction| {
                queued
                    .get(&(end_index, *direction))
                    .map(|(value, _)| *value)
                    .unwrap_or(u32::MAX)
            })
            .unwrap(),
    )]);

    let mut counted = FxHashSet::default();
    while let Some((index, direction)) = queue.pop_front() {
        let Some((value, from)) = queued.get(&(index, direction)) else {
            continue;
        };

        for (from_index, from_direction) in from {
            let Some((from_value, _)) = queued.get(&(*from_index, *from_direction)) else {
                continue;
            };

            if from_value < value {
                counted.insert((index, -direction));
                queue.push_back((*from_index, *from_direction));
            }
        }
    }

    let mut count = counted.iter().map(|(index, _)| index).unique().count() as u32 + 1;
    for (index, direction) in counted {
        count += nodes[index].neighbors[direction as usize].unwrap().1 - 1;
    }

    Ok(count)
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

    fn distance(self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
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

#[derive(Debug)]
struct Node {
    neighbors: [Option<(usize, u32)>; 4],
}
