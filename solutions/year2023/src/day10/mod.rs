use std::collections::{HashSet, VecDeque};

use aoc_core::Day;
use itertools::Itertools;

pub fn day() -> Day {
    let mut solution = Day::new(10);
    solution.part_1(parse, part_one);
    solution.part_2(parse, part_two);
    solution.add_file("test.txt");
    solution.add_file("input.txt");
    solution
}

fn parse(input: String) -> Map {
    let mut start = Position(0);
    let mut width = 0;

    let grid = input
        .split_terminator('\n')
        .enumerate()
        .flat_map(|(y, line)| {
            width = line.len();

            line.char_indices()
                .map(|(x, char)| {
                    if char == 'S' {
                        start = Position(x + y * width);
                    }

                    char.into()
                })
                .collect_vec()
                .into_iter()
        })
        .collect();

    Map::new(grid, width, start)
}

fn part_one(map: Map) -> String {
    for mut direction in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        let mut steps = 1;
        let mut position = map.move_position(map.start, direction);

        while let Some(next) = direction.redirect(map.get(position)) {
            direction = next;
            steps += 1;
            position = map.move_position(position, direction);
        }

        if let Tile::Start = map.get(position) {
            return (steps / 2).to_string();
        }
    }

    unreachable!()
}

fn part_two(map: Map) -> String {
    for mut direction in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        let mut turn = 0;
        let mut positions = HashSet::with_capacity(map.size());
        let mut directions = Vec::with_capacity(map.size());
        let mut position = map.move_position(map.start, direction);

        positions.insert(position);
        directions.push(direction);

        while let Some(next) = direction.redirect(map.get(position)) {
            if next == direction.cw() {
                turn += 1;
            } else if next == direction.ccw() {
                turn -= 1;
            }

            direction = next;
            position = map.move_position(position, direction);
            positions.insert(position);
            directions.push(direction);
        }

        if let Tile::Start = map.get(position) {
            let mut insides = HashSet::new();
            let mut queue = VecDeque::new();

            for direction in directions {
                let side = if turn < 0 {
                    map.move_position(position, direction.ccw())
                } else {
                    map.move_position(position, direction.cw())
                };
                if !positions.contains(&side) && insides.insert(side) {
                    queue.push_back(side);
                }

                // Only one side of a bend is added which sometimes causes tiles to be missed
                let tile = map.get(position);
                if direction == Direction::Up
                    && ((tile == Tile::NorthEast && turn < 0)
                        || (tile == Tile::NorthWest && turn > 0))
                {
                    let back = map.move_position_down(position);
                    if !positions.contains(&back) && insides.insert(back) {
                        queue.push_back(back);
                    }
                }

                position = map.move_position(position, direction);
            }

            while let Some(position) = queue.pop_front() {
                let up = map.move_position_up(position);
                if !positions.contains(&up) && insides.insert(up) {
                    queue.push_back(up);
                }

                let down = map.move_position_down(position);
                if !positions.contains(&down) && insides.insert(down) {
                    queue.push_back(down);
                }

                let left = map.move_position_left(position);
                if !positions.contains(&left) && insides.insert(left) {
                    queue.push_back(left);
                }

                let right = map.move_position_right(position);
                if !positions.contains(&right) && insides.insert(right) {
                    queue.push_back(right);
                }
            }

            return insides.len().to_string();
        }
    }

    unreachable!()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn cw(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }

    fn ccw(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }

    fn redirect(self, tile: Tile) -> Option<Self> {
        match tile {
            Tile::Start | Tile::Ground => None,
            Tile::Vertical => match self {
                Self::Up | Self::Down => Some(self),
                Self::Left | Self::Right => None,
            },
            Tile::Horizontal => match self {
                Self::Left | Self::Right => Some(self),
                Self::Up | Self::Down => None,
            },
            Tile::NorthEast => match self {
                Self::Down => Some(Self::Right),
                Self::Left => Some(Self::Up),
                Self::Up | Self::Right => None,
            },
            Tile::NorthWest => match self {
                Self::Down => Some(Self::Left),
                Self::Right => Some(Self::Up),
                Self::Up | Self::Left => None,
            },
            Tile::SouthEast => match self {
                Self::Up => Some(Self::Right),
                Self::Left => Some(Self::Down),
                Self::Down | Self::Right => None,
            },
            Tile::SouthWest => match self {
                Self::Up => Some(Self::Left),
                Self::Right => Some(Self::Down),
                Self::Down | Self::Left => None,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position(usize);

struct Map {
    grid: Vec<Tile>,
    width: usize,
    start: Position,
}

impl Map {
    #[inline]
    fn new(grid: Vec<Tile>, width: usize, start: Position) -> Self {
        Self { grid, width, start }
    }

    #[inline]
    fn get(&self, position: Position) -> Tile {
        self.grid[position.0]
    }

    #[inline]
    fn size(&self) -> usize {
        self.grid.len()
    }

    #[inline]
    fn move_position_up(&self, position: Position) -> Position {
        Position(position.0 - self.width)
    }

    #[inline]
    fn move_position_down(&self, position: Position) -> Position {
        Position(position.0 + self.width)
    }

    #[inline]
    fn move_position_left(&self, position: Position) -> Position {
        Position(position.0 - 1)
    }

    #[inline]
    fn move_position_right(&self, position: Position) -> Position {
        Position(position.0 + 1)
    }

    fn move_position(&self, position: Position, direction: Direction) -> Position {
        match direction {
            Direction::Up => Position(position.0 - self.width),
            Direction::Down => Position(position.0 + self.width),
            Direction::Left => Position(position.0 - 1),
            Direction::Right => Position(position.0 + 1),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Start,      // 'S'
    Ground,     // '.'
    Vertical,   // '|'
    Horizontal, // '-'
    NorthEast,  // 'L'
    NorthWest,  // 'J'
    SouthEast,  // 'F'
    SouthWest,  // '7'
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'S' => Self::Start,
            '.' => Self::Ground,
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            'F' => Self::SouthEast,
            '7' => Self::SouthWest,
            _ => Self::Ground,
        }
    }
}
