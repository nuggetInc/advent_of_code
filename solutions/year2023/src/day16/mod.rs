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
        .flat_map(|line| {
            width = line.len();
            line.chars().map(|char| Tile::from(char))
        })
        .collect_vec();

    let height = grid.len() / width;

    Map::new(grid, width, height)
}

fn part_one(map: Map) -> AocResult<usize> {
    Ok(map.energized(map.new_position(0, 0), Direction::Right))
}

fn part_two(map: Map) -> AocResult<usize> {
    let mut max = 0;

    for x in 0..map.width {
        max = map
            .energized(map.new_position(x, 0), Direction::Down)
            .max(max);
        max = map
            .energized(map.new_position(x, map.height - 1), Direction::Up)
            .max(max);
    }

    for y in 0..map.height {
        max = map
            .energized(map.new_position(0, y), Direction::Right)
            .max(max);
        max = map
            .energized(map.new_position(map.width - 1, y), Direction::Left)
            .max(max);
    }

    Ok(max)
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    index: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Map {
    grid: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(grid: Vec<Tile>, width: usize, height: usize) -> Self {
        Self {
            grid,
            width,
            height,
        }
    }

    fn new_position(&self, x: usize, y: usize) -> Position {
        Position {
            index: x + y * self.width,
        }
    }

    fn get(&self, position: Position) -> Tile {
        self.grid[position.index]
    }

    fn try_move(&self, pos: Position, dir: Direction) -> Option<Position> {
        match dir {
            Direction::Up if pos.index / self.width > 0 => Some(Position {
                index: pos.index - self.width,
            }),
            Direction::Down if pos.index / self.width < self.height - 1 => Some(Position {
                index: pos.index + self.width,
            }),
            Direction::Left if pos.index % self.width > 0 => Some(Position {
                index: pos.index - 1,
            }),
            Direction::Right if pos.index % self.width < self.width - 1 => Some(Position {
                index: pos.index + 1,
            }),
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
