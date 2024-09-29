use std::{
    collections::VecDeque,
    ops::{Index, IndexMut},
};

use aoc_core::{AocResult, Day};
use itertools::Itertools;

pub fn day() -> Day {
    let mut solution = Day::new(17);
    solution.part_1(|s: String| part_one(parse(s)));
    solution.part_2(|s: String| part_two(parse(s)));
    solution.add_file("files/test.in");
    solution.add_file("files/input.in");
    solution
}

fn parse(input: String) -> Map<i32> {
    let mut width = 0;

    let grid = input
        .split_terminator('\n')
        .flat_map(|line| {
            width = line.len();
            line.chars().map(|char| char.to_digit(10).unwrap() as i32)
        })
        .collect_vec();

    Map::new(grid, width)
}

fn part_one(map: Map<i32>) -> AocResult<i32> {
    Ok(calculate_heat_loss(&map, 1, 3))
}

fn part_two(map: Map<i32>) -> AocResult<i32> {
    Ok(calculate_heat_loss(&map, 4, 10))
}

fn calculate_heat_loss(map: &Map<i32>, cart_min: usize, cart_max: usize) -> i32 {
    let vert_grid = vec![None::<i32>; map.grid.len()];
    let mut vert_map = Map::new(vert_grid, map.grid.len());

    let hori_grid = vec![None::<i32>; map.grid.len()];
    let mut hori_map = Map::new(hori_grid, map.grid.len());

    let start = map.start();
    vert_map[start] = Some(0);
    hori_map[start] = Some(0);

    let mut queue = VecDeque::new();
    queue.push_back((start, Axis::Vertical));
    queue.push_back((start, Axis::Horizontal));

    while let Some((pos, axis)) = queue.pop_front() {
        match axis {
            Axis::Vertical => {
                let loss = vert_map[pos];

                for dir in [Direction::Left, Direction::Right] {
                    let mut heat_loss = 0;

                    for delta in 1..cart_min {
                        let Some(next_pos) = map.position(pos, dir, delta) else {
                            break;
                        };

                        heat_loss += map[next_pos];
                    }

                    for delta in cart_min..=cart_max {
                        let Some(next_pos) = map.position(pos, dir, delta) else {
                            break;
                        };

                        heat_loss += map[next_pos];

                        if loss.unwrap() + heat_loss < hori_map[next_pos].unwrap_or(i32::MAX) {
                            hori_map[next_pos] = Some(loss.unwrap() + heat_loss);
                            queue.push_back((next_pos, Axis::Horizontal));
                        }
                    }
                }
            }
            Axis::Horizontal => {
                let loss = hori_map[pos];

                for dir in [Direction::Up, Direction::Down] {
                    let mut heat_loss = 0;

                    for delta in 1..cart_min {
                        let Some(next_pos) = map.position(pos, dir, delta) else {
                            break;
                        };

                        heat_loss += map[next_pos];
                    }

                    for delta in cart_min..=cart_max {
                        let Some(next_pos) = map.position(pos, dir, delta) else {
                            break;
                        };

                        heat_loss += map[next_pos];

                        if loss.unwrap() + heat_loss < vert_map[next_pos].unwrap_or(i32::MAX) {
                            vert_map[next_pos] = Some(loss.unwrap() + heat_loss);
                            queue.push_back((next_pos, Axis::Vertical));
                        }
                    }
                }
            }
        };
    }

    let end = map.end();
    vert_map[end].unwrap().min(hori_map[end].unwrap())
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    index: usize,
}

struct Map<T> {
    grid: Vec<T>,
    width: usize,
}

impl<T> Map<T> {
    fn new(grid: Vec<T>, width: usize) -> Self {
        Self { grid, width }
    }

    fn start(&self) -> Position {
        Position { index: 0 }
    }

    fn end(&self) -> Position {
        Position {
            index: self.grid.len() - 1,
        }
    }

    fn position(&self, pos: Position, dir: Direction, delta: usize) -> Option<Position> {
        match dir {
            Direction::Up if pos.index >= delta * self.width => Some(Position {
                index: pos.index - delta * self.width,
            }),
            Direction::Down if pos.index < self.grid.len() - delta * self.width => Some(Position {
                index: pos.index + delta * self.width,
            }),
            Direction::Left if pos.index % self.width >= delta => Some(Position {
                index: pos.index - delta,
            }),
            Direction::Right if pos.index % self.width < self.width - delta => Some(Position {
                index: pos.index + delta,
            }),
            _ => None,
        }
    }
}

impl<T> Index<Position> for Map<T> {
    type Output = T;

    fn index(&self, index: Position) -> &Self::Output {
        &self.grid[index.index]
    }
}

impl<T> IndexMut<Position> for Map<T> {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.grid[index.index]
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Axis {
    Vertical,
    Horizontal,
}
