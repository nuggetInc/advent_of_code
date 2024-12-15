use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

use aoc_core::{AocResult, Day};
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

fn parse_one(input: &String) -> (Position, Grid, Vec<Direction>) {
    let mut start_position = Position::new(0, 0);

    let mut data = Vec::new();
    let mut width = 0;
    let mut height = 0;

    let (map, moves) = input.split_once("\n\n").unwrap();

    for (row, line) in map.split_terminator('\n').enumerate() {
        height = row + 1;
        for (col, c) in line.char_indices() {
            width = col + 1;

            data.push(c);

            if c == '@' {
                start_position = Position::new(col as i32, row as i32)
            }
        }
    }

    (
        start_position,
        Grid::new(data, Bounds::new(width as i32, height as i32)),
        parse_moves(moves),
    )
}

fn parse_two(input: &String) -> (Position, Grid, Vec<Direction>) {
    let mut start_position = Position::new(0, 0);

    let mut data = Vec::new();
    let mut width = 0;
    let mut height = 0;

    let (map, moves) = input.split_once("\n\n").unwrap();

    for (row, line) in map.split_terminator('\n').enumerate() {
        height = row + 1;
        for (col, c) in line.char_indices() {
            width = col + 1;

            match c {
                '@' => data.extend(['@', '.']),
                'O' => data.extend(['[', ']']),
                '.' => data.extend(['.', '.']),
                '#' => data.extend(['#', '#']),
                _ => unreachable!(),
            }

            if c == '@' {
                start_position = Position::new(col as i32 * 2, row as i32)
            }
        }
    }

    (
        start_position,
        Grid::new(data, Bounds::new(width as i32 * 2, height as i32)),
        parse_moves(moves),
    )
}

fn parse_moves(input: &str) -> Vec<Direction> {
    input
        .chars()
        .filter_map(|c| match c {
            '^' => Some(Direction::North),
            'v' => Some(Direction::South),
            '>' => Some(Direction::East),
            '<' => Some(Direction::West),
            _ => None,
        })
        .collect_vec()
}

fn part_one(input: &String) -> AocResult<i32> {
    let (mut position, mut grid, moves) = parse_one(input);

    for direction in moves {
        if try_move_one(&mut grid, position, direction) {
            position = position.moved(direction);
        }
    }

    let mut sum = 0;

    for row in 0..grid.bounds.height {
        for col in 0..grid.bounds.width {
            let position = Position::new(col, row);
            if grid.get(position).is_none_or(|c| *c != 'O') {
                continue;
            }

            sum += 100 * row + col;
        }
    }

    Ok(sum)
}

fn try_move_one(grid: &mut Grid, position: Position, direction: Direction) -> bool {
    let new_position = position.moved(direction);

    match grid.get(position) {
        Some('O' | '@') => {
            if try_move_one(grid, new_position, direction) {
                let c = *grid.get(position).unwrap();
                *grid.get_mut(new_position).unwrap() = c;
                *grid.get_mut(position).unwrap() = '.';

                true
            } else {
                false
            }
        }
        Some('.') => true,
        _ => false,
    }
}

fn part_two(input: &String) -> AocResult<i32> {
    let (mut position, mut grid, moves) = parse_two(input);

    for direction in moves {
        if can_move_two(&grid, position, direction, true) {
            move_two(&mut grid, position, direction, true);
            position = position.moved(direction);
        }
    }

    let mut sum = 0;

    for row in 0..grid.bounds.height {
        for col in 0..grid.bounds.width {
            let position = Position::new(col, row);
            if grid.get(position).is_none_or(|c| *c != '[') {
                continue;
            }

            sum += 100 * row + col;
        }
    }

    Ok(sum)
}

fn move_two(grid: &mut Grid, position: Position, direction: Direction, process_second: bool) {
    let new_position = position.moved(direction);

    match grid.get(position) {
        Some('[') => {
            move_two(grid, new_position, direction, true);

            let c = *grid.get(position).unwrap();
            *grid.get_mut(new_position).unwrap() = c;
            *grid.get_mut(position).unwrap() = '.';

            if process_second && direction.is_vertical() {
                move_two(grid, new_position.moved(Direction::East), direction, true);

                let c = *grid.get(position.moved(Direction::East)).unwrap();
                *grid.get_mut(new_position.moved(Direction::East)).unwrap() = c;
                *grid.get_mut(position.moved(Direction::East)).unwrap() = '.';
            }
        }
        Some(']') => {
            move_two(grid, new_position, direction, true);

            let c = *grid.get(position).unwrap();
            *grid.get_mut(new_position).unwrap() = c;
            *grid.get_mut(position).unwrap() = '.';

            if process_second && direction.is_vertical() {
                move_two(grid, new_position.moved(Direction::West), direction, true);

                let c = *grid.get(position.moved(Direction::West)).unwrap();
                *grid.get_mut(new_position.moved(Direction::West)).unwrap() = c;
                *grid.get_mut(position.moved(Direction::West)).unwrap() = '.';
            }
        }
        Some('@') => {
            move_two(grid, new_position, direction, true);

            let c = *grid.get(position).unwrap();
            *grid.get_mut(new_position).unwrap() = c;
            *grid.get_mut(position).unwrap() = '.';
        }
        _ => (),
    }
}

fn can_move_two(
    grid: &Grid,
    position: Position,
    direction: Direction,
    process_second: bool,
) -> bool {
    let new_position = position.moved(direction);

    match grid.get(position) {
        Some('[') if process_second && direction.is_vertical() => {
            can_move_two(grid, new_position, direction, true)
                && can_move_two(grid, new_position.moved(Direction::East), direction, true)
        }
        Some(']') if process_second && direction.is_vertical() => {
            can_move_two(grid, new_position, direction, true)
                && can_move_two(grid, new_position.moved(Direction::West), direction, true)
        }
        Some('[' | ']' | '@') => can_move_two(grid, new_position, direction, true),
        Some('.') => true,
        _ => false,
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn is_vertical(self) -> bool {
        matches!(self, Self::North | Self::South)
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

    fn moved(self, direction: Direction) -> Position {
        match direction {
            Direction::North => self + Position::new(0, -1),
            Direction::South => self + Position::new(0, 1),
            Direction::East => self + Position::new(1, 0),
            Direction::West => self + Position::new(-1, 0),
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

    fn get_mut(&mut self, position: Position) -> Option<&mut char> {
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
