use std::{
    collections::VecDeque,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use aoc_core::{AocResult, Day};

pub fn day() -> Day {
    let mut solution = Day::new();
    solution.part_1(part_one);
    solution.part_2(part_two);
    solution.add_file("files/test.in");
    solution.add_file("files/input.in");
    solution
}

fn parse(input: &String) -> Grid {
    let mut data = Vec::new();
    let mut width = 0;
    let mut height = 0;

    for (row, line) in input.split_terminator('\n').enumerate() {
        height = row + 1;
        for (col, c) in line.char_indices() {
            width = col + 1;

            data.push((c.to_digit(10).unwrap() as u8, 0));
        }
    }

    Grid::new(data, Bounds::new(width as i32, height as i32))
}

fn part_one(input: &String) -> AocResult<u32> {
    let mut grid = parse(input);

    let mut queue = VecDeque::new();
    let mut visit_id = 1;

    for row in 0..grid.bounds.height {
        for col in 0..grid.bounds.width {
            let start_position = Position::new(col, row);
            if !matches!(grid.get(start_position), Some((0, _))) {
                continue;
            }

            queue.push_back((start_position, visit_id));
            visit_id += 1;
        }
    }

    let mut count = 0;

    while let Some((position, visit_id)) = queue.pop_front() {
        let (height, _) = *grid.get(position).unwrap();

        for neighbor in [
            position + Position::new(0, -1),
            position + Position::new(0, 1),
            position + Position::new(-1, 0),
            position + Position::new(1, 0),
        ] {
            if let Some((neighbor_height, neighbor_visit)) = grid.get_mut(neighbor) {
                if *neighbor_height != height + 1 {
                    continue;
                }

                if *neighbor_visit == visit_id {
                    continue;
                }

                *neighbor_visit = visit_id;
                if *neighbor_height == 9 {
                    count += 1;
                } else {
                    queue.push_front((neighbor, visit_id));
                }
            }
        }
    }

    Ok(count)
}

fn part_two(input: &String) -> AocResult<u32> {
    let mut grid = parse(input);

    let mut queue = VecDeque::new();

    for row in 0..grid.bounds.height {
        for col in 0..grid.bounds.width {
            let start_position = Position::new(col, row);
            if !matches!(grid.get(start_position), Some((0, _))) {
                continue;
            }

            queue.push_back(start_position);
            grid.get_mut(start_position).unwrap().1 = 1;
        }
    }

    let mut count = 0;

    while let Some(position) = queue.pop_front() {
        let (height, visits) = *grid.get(position).unwrap();
        if height == 9 {
            count += visits as u32;
            continue;
        }

        for neighbor in [
            position + Position::new(0, -1),
            position + Position::new(0, 1),
            position + Position::new(-1, 0),
            position + Position::new(1, 0),
        ] {
            if let Some((neighbor_height, neighbor_visits)) = grid.get_mut(neighbor) {
                if *neighbor_height != height + 1 {
                    continue;
                }

                if *neighbor_visits == 0 {
                    queue.push_back(neighbor);
                }

                *neighbor_visits += visits;
            }
        }
    }

    Ok(count)
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

struct Grid {
    data: Vec<(u8, u8)>,
    bounds: Bounds,
}

impl Grid {
    fn new(data: Vec<(u8, u8)>, bounds: Bounds) -> Self {
        Self { data, bounds }
    }

    fn get(&self, position: Position) -> Option<&(u8, u8)> {
        if !self.bounds.contains(position) {
            return None;
        }

        self.data
            .get((position.x + position.y * self.bounds.width) as usize)
    }

    fn get_mut(&mut self, position: Position) -> Option<&mut (u8, u8)> {
        if !self.bounds.contains(position) {
            return None;
        }

        self.data
            .get_mut((position.x + position.y * self.bounds.width) as usize)
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
