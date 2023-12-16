use std::collections::{BTreeSet, VecDeque};

use aoc_core::{AocResult, Day};
use itertools::Itertools;

pub fn day() -> Day {
    let mut solution = Day::new(16);
    solution.part_1(parse, part_one);
    solution.part_2(parse, part_two);
    solution.add_file("files/test.in");
    solution.add_file("files/input.in");
    solution
}

fn parse(input: String) -> Map {
    let mut width = 0;

    let grid = input
        .split_terminator('\n')
        .map(|line| {
            width = line.len();
            line.chars().map(|char| Tile::from(char)).collect()
        })
        .collect_vec();

    let height = grid.len();

    Map::new(grid, width, height)
}

fn part_one(map: Map) -> AocResult<usize> {
    Ok(map.energized(Position::new(0, 0), Direction::Right))
}

fn part_two(map: Map) -> AocResult<usize> {
    let mut max = 0;

    for x in 0..map.width {
        max = map.energized(Position::new(x, 0), Direction::Down).max(max);
        max = map
            .energized(Position::new(x, map.height - 1), Direction::Up)
            .max(max);
    }

    for y in 0..map.height {
        max = map
            .energized(Position::new(0, y), Direction::Right)
            .max(max);
        max = map
            .energized(Position::new(map.width - 1, y), Direction::Left)
            .max(max);
    }

    Ok(max)
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Map {
    grid: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(grid: Vec<Vec<Tile>>, width: usize, height: usize) -> Self {
        Self {
            grid,
            width,
            height,
        }
    }

    fn get(&self, position: Position) -> Tile {
        self.grid[position.y][position.x]
    }

    fn try_move(&self, pos: Position, dir: Direction) -> Option<Position> {
        match dir {
            Direction::Up if pos.y > 0 => Some(Position::new(pos.x, pos.y - 1)),
            Direction::Down if pos.y < self.height - 1 => Some(Position::new(pos.x, pos.y + 1)),
            Direction::Left if pos.x > 0 => Some(Position::new(pos.x - 1, pos.y)),
            Direction::Right if pos.x < self.width - 1 => Some(Position::new(pos.x + 1, pos.y)),
            _ => None,
        }
    }

    fn energized(&self, position: Position, direction: Direction) -> usize {
        let mut visited = BTreeSet::new();
        let mut queue = VecDeque::new();

        queue.push_back((position, direction));

        while let Some((position, direction)) = queue.pop_front() {
            if !visited.insert((position, direction)) {
                continue;
            }

            match self.get(position) {
                Tile::Empty => {
                    if let Some(next) = self.try_move(position, direction) {
                        queue.push_back((next, direction));
                    }
                }
                Tile::Mirror => {
                    let new_dir = match direction {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Down,
                        Direction::Right => Direction::Up,
                    };

                    if let Some(next) = self.try_move(position, new_dir) {
                        queue.push_back((next, new_dir));
                    }
                }
                Tile::BackMirror => {
                    let new_dir = match direction {
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    };

                    if let Some(next) = self.try_move(position, new_dir) {
                        queue.push_back((next, new_dir));
                    }
                }
                Tile::DashSplitter => match direction {
                    Direction::Up | Direction::Down => {
                        if let Some(next) = self.try_move(position, Direction::Left) {
                            queue.push_back((next, Direction::Left));
                        }

                        if let Some(next) = self.try_move(position, Direction::Right) {
                            queue.push_back((next, Direction::Right));
                        }
                    }
                    Direction::Left | Direction::Right => {
                        if let Some(next) = self.try_move(position, direction) {
                            queue.push_back((next, direction));
                        }
                    }
                },
                Tile::PipeSplitter => match direction {
                    Direction::Up | Direction::Down => {
                        if let Some(next) = self.try_move(position, direction) {
                            queue.push_back((next, direction));
                        }
                    }
                    Direction::Left | Direction::Right => {
                        if let Some(next) = self.try_move(position, Direction::Up) {
                            queue.push_back((next, Direction::Up));
                        }

                        if let Some(next) = self.try_move(position, Direction::Down) {
                            queue.push_back((next, Direction::Down));
                        }
                    }
                },
            }
        }

        visited
            .into_iter()
            .map(|(pos, _)| pos)
            .collect::<BTreeSet<_>>()
            .len()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Mirror,
    BackMirror,
    DashSplitter,
    PipeSplitter,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '/' => Self::Mirror,
            '\\' => Self::BackMirror,
            '-' => Self::DashSplitter,
            '|' => Self::PipeSplitter,
            _ => Self::Empty,
        }
    }
}
