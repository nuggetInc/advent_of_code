use std::collections::VecDeque;

use aoc_core::{AocResult, Day};
use itertools::Itertools;

pub fn day() -> Day {
    let mut solution = Day::new(17);
    solution.part_1(parse, part_one);
    solution.part_2(parse, part_two);
    solution.add_file("files/test.in");
    solution.add_file("files/input.in");
    solution
}

fn parse(input: String) -> Map<i32> {
    let mut width = 0;

    let grid = input
        .split_terminator('\n')
        .map(|line| {
            width = line.len();
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect_vec();

    let height = grid.len();

    Map::new(grid, width, height)
}

fn part_one(map: Map<i32>) -> AocResult<i32> {
    let vert_grid = vec![vec![None::<i32>; map.width]; map.height];
    let mut vert_map = Map::new(vert_grid, map.width, map.height);

    let hori_grid = vec![vec![None::<i32>; map.width]; map.height];
    let mut hori_map = Map::new(hori_grid, map.width, map.height);

    *vert_map.get_mut(Position::new(0, 0)).unwrap() = Some(0);
    *hori_map.get_mut(Position::new(0, 0)).unwrap() = Some(0);

    let mut queue = VecDeque::new();
    for delta in 1..=3 {
        queue.push_back((Position::new(0, delta), Axis::Vertical));
        queue.push_back((Position::new(delta, 0), Axis::Horizontal));
    }

    while let Some((pos, axis)) = queue.pop_front() {
        let mut min = match axis {
            Axis::Vertical => *vert_map.get(pos).unwrap(),
            Axis::Horizontal => *hori_map.get(pos).unwrap(),
        };

        match axis {
            Axis::Vertical => {
                for dir in [Direction::Up, Direction::Down] {
                    for delta in 1..=3 {
                        let Some(loss) = map
                            .position(pos, dir, delta)
                            .and_then(|other_pos| hori_map.get(other_pos))
                        else {
                            break;
                        };

                        let Some(loss) = loss else {
                            continue;
                        };

                        let mut value = loss + map.get(pos).unwrap();
                        for delta in 1..delta {
                            let Some(loss) = map
                                .position(pos, dir, delta)
                                .and_then(|other_pos| map.get(other_pos))
                            else {
                                continue;
                            };

                            value += loss;
                        }

                        if value < min.unwrap_or(i32::MAX) {
                            min = Some(value);
                        }
                    }
                }

                let loss = vert_map.get_mut(pos).unwrap();

                if min.eq(loss) {
                    continue;
                }

                *loss = min;

                for dir in [Direction::Left, Direction::Right] {
                    for delta in 1..=3 {
                        let Some(next_pos) = map.position(pos, dir, delta) else {
                            continue;
                        };

                        queue.push_back((next_pos, Axis::Horizontal));
                    }
                }
            }
            Axis::Horizontal => {
                for dir in [Direction::Left, Direction::Right] {
                    for delta in 1..=3 {
                        let Some(loss) = map
                            .position(pos, dir, delta)
                            .and_then(|other_pos| vert_map.get(other_pos))
                        else {
                            break;
                        };

                        let Some(loss) = loss else {
                            continue;
                        };

                        let mut value = loss + map.get(pos).unwrap();
                        for delta in 1..delta {
                            let Some(loss) = map
                                .position(pos, dir, delta)
                                .and_then(|other_pos| map.get(other_pos))
                            else {
                                continue;
                            };

                            value += loss;
                        }

                        if value < min.unwrap_or(i32::MAX) {
                            min = Some(value);
                        }
                    }
                }

                let loss = hori_map.get_mut(pos).unwrap();

                if min.eq(loss) {
                    continue;
                }

                *loss = min;

                for dir in [Direction::Up, Direction::Down] {
                    for delta in 1..=3 {
                        let Some(next_pos) = map.position(pos, dir, delta) else {
                            continue;
                        };

                        queue.push_back((next_pos, Axis::Vertical));
                    }
                }
            }
        }
    }

    let end = Position::new(map.width - 1, map.height - 1);
    if let (Some(Some(vert)), Some(Some(hori))) = (vert_map.get(end), hori_map.get(end)) {
        Ok(*vert.min(hori))
    } else {
        unreachable!();
    }
}

fn part_two(map: Map<i32>) -> AocResult<i32> {
    let vert_grid = vec![vec![None::<i32>; map.width]; map.height];
    let mut vert_map = Map::new(vert_grid, map.width, map.height);

    let hori_grid = vec![vec![None::<i32>; map.width]; map.height];
    let mut hori_map = Map::new(hori_grid, map.width, map.height);

    *vert_map.get_mut(Position::new(0, 0)).unwrap() = Some(0);
    *hori_map.get_mut(Position::new(0, 0)).unwrap() = Some(0);

    let mut queue = VecDeque::new();
    for delta in 4..=10 {
        queue.push_back((Position::new(0, delta), Axis::Vertical));
        queue.push_back((Position::new(delta, 0), Axis::Horizontal));
    }

    while let Some((pos, axis)) = queue.pop_front() {
        let mut min = match axis {
            Axis::Vertical => *vert_map.get(pos).unwrap(),
            Axis::Horizontal => *hori_map.get(pos).unwrap(),
        };

        match axis {
            Axis::Vertical => {
                for dir in [Direction::Up, Direction::Down] {
                    for delta in 4..=10 {
                        let Some(loss) = map
                            .position(pos, dir, delta)
                            .and_then(|other_pos| hori_map.get(other_pos))
                        else {
                            break;
                        };

                        let Some(loss) = loss else {
                            continue;
                        };

                        let mut value = loss + map.get(pos).unwrap();
                        for delta in 1..delta {
                            let Some(loss) = map
                                .position(pos, dir, delta)
                                .and_then(|other_pos| map.get(other_pos))
                            else {
                                continue;
                            };

                            value += loss;
                        }

                        if value < min.unwrap_or(i32::MAX) {
                            min = Some(value);
                        }
                    }
                }

                let loss = vert_map.get_mut(pos).unwrap();

                if min.eq(loss) {
                    continue;
                }

                *loss = min;

                for dir in [Direction::Left, Direction::Right] {
                    for delta in 4..=10 {
                        let Some(next_pos) = map.position(pos, dir, delta) else {
                            continue;
                        };

                        queue.push_back((next_pos, Axis::Horizontal));
                    }
                }
            }
            Axis::Horizontal => {
                for dir in [Direction::Left, Direction::Right] {
                    for delta in 4..=10 {
                        let Some(loss) = map
                            .position(pos, dir, delta)
                            .and_then(|other_pos| vert_map.get(other_pos))
                        else {
                            break;
                        };

                        let Some(loss) = loss else {
                            continue;
                        };

                        let mut value = loss + map.get(pos).unwrap();
                        for delta in 1..delta {
                            let Some(loss) = map
                                .position(pos, dir, delta)
                                .and_then(|other_pos| map.get(other_pos))
                            else {
                                continue;
                            };

                            value += loss;
                        }

                        if value < min.unwrap_or(i32::MAX) {
                            min = Some(value);
                        }
                    }
                }

                let loss = hori_map.get_mut(pos).unwrap();

                if min.eq(loss) {
                    continue;
                }

                *loss = min;

                for dir in [Direction::Up, Direction::Down] {
                    for delta in 4..=10 {
                        let Some(next_pos) = map.position(pos, dir, delta) else {
                            continue;
                        };

                        queue.push_back((next_pos, Axis::Vertical));
                    }
                }
            }
        }
    }

    let end = Position::new(map.width - 1, map.height - 1);
    if let (Some(Some(vert)), Some(Some(hori))) = (vert_map.get(end), hori_map.get(end)) {
        Ok(*vert.min(hori))
    } else {
        unreachable!();
    }
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

struct Map<T> {
    grid: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T> Map<T> {
    fn new(grid: Vec<Vec<T>>, width: usize, height: usize) -> Self {
        Self {
            grid,
            width,
            height,
        }
    }

    fn get(&self, pos: Position) -> Option<&T> {
        self.grid.get(pos.y)?.get(pos.x)
    }

    fn get_mut(&mut self, pos: Position) -> Option<&mut T> {
        self.grid.get_mut(pos.y)?.get_mut(pos.x)
    }

    fn position(&self, pos: Position, dir: Direction, delta: usize) -> Option<Position> {
        match dir {
            Direction::Up if pos.y >= delta => Some(Position::new(pos.x, pos.y - delta)),
            Direction::Down if pos.y < self.height - delta => {
                Some(Position::new(pos.x, pos.y + delta))
            }
            Direction::Left if pos.x >= delta => Some(Position::new(pos.x - delta, pos.y)),
            Direction::Right if pos.x < self.width - delta => {
                Some(Position::new(pos.x + delta, pos.y))
            }
            _ => None,
        }
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
