use std::{
    collections::{BTreeMap, BTreeSet},
    ops::{Add, AddAssign, Sub, SubAssign},
    time::Instant,
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

fn parse(input: &String) -> (Grid, Vector, Vector) {
    let mut data = Vec::new();
    let mut width = 0;
    let mut height = 0;

    let mut position = Vector::new(0, 0);
    let mut direction = Vector::new(0, 0);

    for (row, line) in input.split_terminator('\n').enumerate() {
        height = row + 1;
        for (col, c) in line.char_indices() {
            width = col + 1;

            if matches!(c, '^' | 'v' | '<' | '>') {
                position = Vector::new(col as i32, row as i32);
            }
            match c {
                '^' => direction = Vector::new(0, -1),
                'v' => direction = Vector::new(0, 1),
                '<' => direction = Vector::new(-1, 0),
                '>' => direction = Vector::new(1, 0),
                _ => (),
            }

            data.push(c != '#');
        }
    }

    (
        Grid {
            data,
            width,
            height,
        },
        position,
        direction,
    )
}

fn part_one(input: &String) -> AocResult<usize> {
    let (grid, mut position, mut direction) = parse(input);

    let mut positions = BTreeSet::new();

    while let Some(walkable) = grid.get(position.x, position.y) {
        if walkable {
            positions.insert(position);
            position += direction;
        } else {
            position -= direction;
            direction = direction.rotated();
        }
    }

    Ok(positions.len())
}

fn part_two(input: &String) -> AocResult<u32> {
    let (mut grid, start_position, start_direction) = parse(input);

    let mut position = start_position;
    let mut direction = start_direction;

    let mut positions = BTreeMap::new();

    while let Some(walkable) = grid.get(position.x, position.y) {
        if walkable {
            if !positions.contains_key(&position) {
                positions.insert(position, direction);
            }
            position += direction;
        } else {
            position -= direction;
            direction = direction.rotated();
        }
    }

    let mut count = 0;

    for (obstacle_position, start_direction) in positions {
        if obstacle_position == start_position {
            continue;
        }

        let mut position = obstacle_position - start_direction;
        let mut direction = start_direction;

        let mut walked = BTreeSet::new();
        grid.set(obstacle_position.x, obstacle_position.y, false);

        let instant = Instant::now();

        while let Some(walkable) = grid.get(position.x, position.y) {
            if walkable {
                if !walked.insert((position, direction)) {
                    count += 1;
                    break;
                }
                position += direction;
            } else {
                position -= direction;
                direction = direction.rotated();
            }
        }

        eprintln!("{:?}", instant.elapsed());

        grid.set(obstacle_position.x, obstacle_position.y, true);
    }

    Ok(count)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn rotated(&self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }
}

impl Add<Vector> for Vector {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Vector> for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl Sub<Vector> for Vector {
    type Output = Self;

    fn sub(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<Vector> for Vector {
    fn sub_assign(&mut self, rhs: Vector) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

struct Grid {
    data: Vec<bool>,
    width: usize,
    height: usize,
}

impl Grid {
    fn get(&self, col: i32, row: i32) -> Option<bool> {
        if col < 0 || col >= self.width as i32 || row < 0 {
            return None;
        }

        self.data
            .get((col + row * self.width as i32) as usize)
            .cloned()
    }

    fn set(&mut self, col: i32, row: i32, value: bool) {
        if col < 0 || col >= self.width as i32 || row < 0 {
            return;
        }

        *self
            .data
            .get_mut((col + row * self.width as i32) as usize)
            .unwrap() = value;
    }
}
