use std::collections::BinaryHeap;

use aoc_core::{AocResult, Day};
use itertools::Itertools;

pub fn day() -> Day {
    let mut solution = Day::new(10);
    solution.part_1(|s: String| part_one(parse(s)));
    solution.part_2(|s: String| part_two(parse(s)));
    solution.add_file("files/test.in");
    solution.add_file("files/input.in");
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

fn part_one(map: Map) -> AocResult<i32> {
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
            return Ok(steps / 2);
        }
    }

    unreachable!()
}

fn part_two(map: Map) -> AocResult<usize> {
    for mut direction in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        let mut position = map.move_position(map.start, direction);

        let mut positions = BinaryHeap::with_capacity(map.size());
        positions.push(position);

        let initial_direction = direction;
        while let Some(next) = direction.redirect(map.get(position)) {
            direction = next;
            position = map.move_position(position, direction);
            positions.push(position);
        }

        if let Tile::Start = map.get(position) {
            let mut count = 0;
            let mut do_count = false;
            let mut last_open = Tile::Ground;

            let mut last_position = Position(0);
            for position in positions.into_sorted_vec().into_iter() {
                if do_count && position.0 - last_position.0 > 1 {
                    count += position.0 - last_position.0 - 1;
                }
                last_position = position;

                match map.get(position) {
                    Tile::Start => match (initial_direction, direction) {
                        (Direction::Right, Direction::Down) | (Direction::Up, Direction::Left) => {
                            last_open = Tile::NorthEast
                        }
                        (Direction::Right, Direction::Up) | (Direction::Down, Direction::Left) => {
                            last_open = Tile::SouthEast
                        }
                        (Direction::Left, Direction::Down) | (Direction::Up, Direction::Right)
                            if last_open == Tile::SouthEast =>
                        {
                            do_count = !do_count
                        }
                        (Direction::Left, Direction::Up) | (Direction::Down, Direction::Right)
                            if last_open == Tile::NorthEast =>
                        {
                            do_count = !do_count
                        }
                        (Direction::Up, Direction::Up) | (Direction::Down, Direction::Down) => {
                            do_count = !do_count
                        }
                        _ => (),
                    },
                    Tile::Vertical => do_count = !do_count,
                    Tile::NorthEast => last_open = Tile::NorthEast,
                    Tile::NorthWest if last_open == Tile::SouthEast => do_count = !do_count,
                    Tile::SouthEast => last_open = Tile::SouthEast,
                    Tile::SouthWest if last_open == Tile::NorthEast => do_count = !do_count,
                    _ => (),
                }
            }

            return Ok(count);
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
