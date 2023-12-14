use std::collections::BTreeSet;

use aoc_core::{AocResult, Day};
use itertools::Itertools;

pub fn day() -> Day {
    let mut solution = Day::new(14);
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

            line.chars()
                .map(|char| match char {
                    'O' => Rock::Round,
                    '#' => Rock::Square,
                    _ => Rock::None,
                })
                .collect()
        })
        .collect_vec();

    let height = grid.len();

    Map::new(grid, width, height)
}

fn part_one(mut map: Map) -> AocResult<usize> {
    Ok(map.roll_north().load())
}

fn part_two(mut map: Map) -> AocResult<usize> {
    let mut previous = BTreeSet::new();
    let mut history = Vec::new();

    while !previous.contains(&map) {
        previous.insert(map.clone());
        history.push(map.clone());
        map.roll_north().roll_west().roll_south().roll_east();
    }

    let position = history.iter().position(|prev| prev.eq(&map)).unwrap();
    let repeat = history.len() - position;
    let end_index = (1_000_000_000 - history.len()) % repeat + position;

    Ok(history[end_index].load())
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Map {
    grid: Vec<Vec<Rock>>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(grid: Vec<Vec<Rock>>, width: usize, height: usize) -> Self {
        Self {
            grid,
            width,
            height,
        }
    }

    fn load(&self) -> usize {
        let mut sum = 0;

        for row in 0..self.height {
            for column in 0..self.width {
                if self.grid[row][column] == Rock::Round {
                    sum += self.height - row;
                }
            }
        }

        sum
    }

    fn roll_north(&mut self) -> &mut Self {
        for row in 1..self.height {
            for column in 0..self.width {
                if self.grid[row][column] != Rock::Round {
                    continue;
                }

                for y in (0..row).rev() {
                    if self.grid[y][column] != Rock::None {
                        self.grid[row][column] = Rock::None;
                        self.grid[y + 1][column] = Rock::Round;
                        break;
                    } else if y == 0 {
                        self.grid[row][column] = Rock::None;
                        self.grid[y][column] = Rock::Round;
                    }
                }
            }
        }

        self
    }

    fn roll_south(&mut self) -> &mut Self {
        for row in (0..self.height - 1).rev() {
            for column in 0..self.width {
                if self.grid[row][column] != Rock::Round {
                    continue;
                }

                for y in row + 1..self.height {
                    if self.grid[y][column] != Rock::None {
                        self.grid[row][column] = Rock::None;
                        self.grid[y - 1][column] = Rock::Round;
                        break;
                    } else if y == self.height - 1 {
                        self.grid[row][column] = Rock::None;
                        self.grid[y][column] = Rock::Round;
                    }
                }
            }
        }

        self
    }

    fn roll_west(&mut self) -> &mut Self {
        for column in 1..self.width {
            for row in 0..self.height {
                if self.grid[row][column] != Rock::Round {
                    continue;
                }

                for x in (0..column).rev() {
                    if self.grid[row][x] != Rock::None {
                        self.grid[row][column] = Rock::None;
                        self.grid[row][x + 1] = Rock::Round;
                        break;
                    } else if x == 0 {
                        self.grid[row][column] = Rock::None;
                        self.grid[row][x] = Rock::Round;
                    }
                }
            }
        }

        self
    }

    fn roll_east(&mut self) -> &mut Self {
        for column in (0..self.width - 1).rev() {
            for row in 0..self.height {
                if self.grid[row][column] != Rock::Round {
                    continue;
                }

                for x in column + 1..self.width {
                    if self.grid[row][x] != Rock::None {
                        self.grid[row][column] = Rock::None;
                        self.grid[row][x - 1] = Rock::Round;
                        break;
                    } else if x == self.width - 1 {
                        self.grid[row][column] = Rock::None;
                        self.grid[row][x] = Rock::Round;
                    }
                }
            }
        }

        self
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Rock {
    Round,
    Square,
    None,
}
