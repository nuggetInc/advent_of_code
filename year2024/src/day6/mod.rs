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
    solution.add_file("files/input.in");
    solution
}

fn parse(input: &String) -> (Grid, Position, Position) {
    let mut data = Vec::new();
    let mut width = 0;
    let mut height = 0;

    let mut position = Position::new(0, 0);
    let mut direction = Position::new(0, 0);

    for (row, line) in input.split_terminator('\n').enumerate() {
        height = row + 1;
        for (col, c) in line.char_indices() {
            width = col + 1;

            if matches!(c, '^' | 'v' | '<' | '>') {
                position = Position::new(col as i32, row as i32);
            }
            match c {
                '^' => direction = Position::new(0, -1),
                'v' => direction = Position::new(0, 1),
                '<' => direction = Position::new(-1, 0),
                '>' => direction = Position::new(1, 0),
                _ => (),
            }

            data.push(c != '#');
        }
    }

    (
        Grid::new(data, Bounds::new(width as i32, height as i32)),
        position,
        direction,
    )
}

fn part_one(input: &String) -> AocResult<usize> {
    let (grid, mut position, mut direction) = parse(input);

    let mut positions = BTreeSet::new();

    while let Some(walkable) = grid.get(position) {
        if *walkable {
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
    let (mut grid, mut position, mut direction) = parse(input);

    let mut count = 0;
    let mut obstacles = BTreeSet::from([position]);

    while let Some(walkable) = grid.get(position) {
        if *walkable {
            let obstacle_position = position + direction;
            if grid.get(obstacle_position) == Some(&true) && obstacles.insert(obstacle_position) {
                *grid.get_mut(obstacle_position).unwrap() = false;

                let mut position = position;
                let mut direction = direction;

                let mut walked = BTreeSet::new();

                while let Some(walkable) = grid.get(position) {
                    if *walkable {
                        position += direction;
                    } else {
                        position -= direction;

                        if !walked.insert((position, direction)) {
                            count += 1;
                            break;
                        }

                        direction = direction.rotated();
                    }
                }

                *grid.get_mut(obstacle_position).unwrap() = true;
            }

            position += direction;
        } else {
            position -= direction;
            direction = direction.rotated();
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

    fn rotated(&self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
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
    data: Vec<bool>,
    bounds: Bounds,
}

impl Grid {
    fn new(data: Vec<bool>, bounds: Bounds) -> Self {
        Self { data, bounds }
    }

    fn get(&self, position: Position) -> Option<&bool> {
        if !self.bounds.contains(position) {
            return None;
        }

        self.data
            .get((position.x + position.y * self.bounds.width) as usize)
    }

    fn get_mut(&mut self, position: Position) -> Option<&mut bool> {
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
