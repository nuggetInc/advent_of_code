use std::{
    collections::VecDeque,
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
};

use aoc_core::{AocResult, Day};
use fxhash::FxHashSet;

pub fn day() -> Day {
    let mut solution = Day::new();
    solution.part_1(part_one);
    solution.part_2(part_two);
    solution.add_file("files/test.in");
    solution.add_file("files/test2.in");
    solution.add_file("files/test3.in");
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

            data.push((c, None));
        }
    }

    Grid::new(data, Bounds::new(width as i32, height as i32))
}

fn part_one(input: &String) -> AocResult<u32> {
    let mut grid = parse(input);

    let mut current_plot_id = 0;

    let mut sum = 0;

    for row in 0..grid.bounds.height {
        for col in 0..grid.bounds.width {
            let start_position = Position::new(col, row);

            let Some((plot_plant, None)) = grid.get(start_position) else {
                continue;
            };

            let plot_plant = *plot_plant;

            let mut area = 0;
            let mut perimiter = 0;

            let mut queue = VecDeque::from([start_position]);

            while let Some(position) = queue.pop_front() {
                if let Some((plant, plot_id @ None)) = grid.get_mut(position) {
                    if *plant != plot_plant {
                        continue;
                    }

                    *plot_id = Some(current_plot_id);

                    area += 1;
                    perimiter += 4;

                    for neighbor in position.neighbors() {
                        if matches!(grid.get(neighbor), Some((_, Some(neighbor_plot_id))) if *neighbor_plot_id == current_plot_id)
                        {
                            perimiter -= 2;
                        }

                        queue.push_back(neighbor);
                    }
                }
            }

            sum += area * perimiter;
            current_plot_id += 1;
        }
    }

    Ok(sum)
}

fn part_two(input: &String) -> AocResult<u32> {
    let mut grid = parse(input);

    let mut current_plot_id = 0;

    let mut sum = 0;

    for row in 0..grid.bounds.height {
        for col in 0..grid.bounds.width {
            let start_position = Position::new(col, row);

            let Some((plot_plant, None)) = grid.get(start_position) else {
                continue;
            };

            let plot_plant = *plot_plant;
            let mut plot = FxHashSet::default();
            plot.insert(start_position);

            let mut queue = VecDeque::from([start_position]);

            let mut sides = 0;

            while let Some(position) = queue.pop_front() {
                if let Some((plant, plot_id @ None)) = grid.get_mut(position) {
                    if *plant != plot_plant {
                        continue;
                    }

                    let plant = *plant;

                    *plot_id = Some(current_plot_id);
                    plot.insert(position);

                    for direction in Position::directions() {
                        if !grid
                            .get(position + direction)
                            .is_some_and(|tile| tile.0 == plant)
                            && !grid
                                .get(position + direction.rotated_cw())
                                .is_some_and(|tile| tile.0 == plant)
                        {
                            sides += 1;
                        } else if grid
                            .get(position + direction)
                            .is_some_and(|tile| tile.0 == plant)
                            && grid
                                .get(position + direction.rotated_cw())
                                .is_some_and(|tile| tile.0 == plant)
                            && !grid
                                .get(position + direction + direction.rotated_cw())
                                .is_some_and(|tile| tile.0 == plant)
                        {
                            sides += 1;
                        }

                        queue.push_back(position + direction);
                    }
                }
            }

            sum += plot.len() as u32 * sides;
            current_plot_id += 1;
        }
    }

    Ok(sum)
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

    fn directions() -> [Position; 4] {
        [
            Position::new(0, -1),
            Position::new(0, 1),
            Position::new(-1, 0),
            Position::new(1, 0),
        ]
    }

    fn neighbors(self) -> [Position; 4] {
        [
            self + Position::new(0, -1),
            self + Position::new(0, 1),
            self + Position::new(-1, 0),
            self + Position::new(1, 0),
        ]
    }

    fn rotated_cw(self) -> Position {
        Position::new(self.y, -self.x)
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
    data: Vec<(char, Option<u32>)>,
    bounds: Bounds,
}

impl Grid {
    fn new(data: Vec<(char, Option<u32>)>, bounds: Bounds) -> Self {
        Self { data, bounds }
    }

    fn get(&self, position: Position) -> Option<&(char, Option<u32>)> {
        if !self.bounds.contains(position) {
            return None;
        }

        self.data
            .get((position.x + position.y * self.bounds.width) as usize)
    }

    fn get_mut(&mut self, position: Position) -> Option<&mut (char, Option<u32>)> {
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
