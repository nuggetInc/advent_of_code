use std::collections::BTreeSet;

use aoc_core::{AocResult, Day};
use itertools::Itertools;

pub fn day() -> Day {
    let mut solution = Day::new(14);
    solution.part_1(|s: String| part_one(parse(s)));
    solution.part_2(|s: String| part_two(parse(s)));
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

            line.chars().map(|char| match char {
                'O' => Rock::Round,
                '#' => Rock::Square,
                _ => Rock::None,
            })
        })
        .collect_vec();

    let height = grid.len() / width;

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
    grid: Vec<Rock>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(grid: Vec<Rock>, width: usize, height: usize) -> Self {
        Self {
            grid,
            width,
            height,
        }
    }

    #[inline]
    fn get(&self, x: usize, y: usize) -> Rock {
        self.grid[x + y * self.width]
    }

    #[inline]
    fn set(&mut self, x: usize, y: usize, rock: Rock) {
        self.grid[x + y * self.width] = rock
    }

    fn load(&self) -> usize {
        let mut sum = 0;

        for row in 0..self.height {
            for column in 0..self.width {
                if self.get(column, row) == Rock::Round {
                    sum += self.height - row;
                }
            }
        }

        sum
    }

    fn roll_north(&mut self) -> &mut Self {
        for column in 0..self.width {
            let mut count = 0;
            for row in (0..self.height).rev() {
                match self.get(column, row) {
                    Rock::Round => {
                        self.set(column, row, Rock::None);
                        count += 1
                    }
                    Rock::Square => {
                        for delta_y in 1..=count {
                            self.set(column, row + delta_y, Rock::Round)
                        }

                        count = 0;
                    }
                    Rock::None => (),
                }
            }

            for y in 0..count {
                self.set(column, y, Rock::Round)
            }
        }

        self
    }

    fn roll_south(&mut self) -> &mut Self {
        for column in 0..self.width {
            let mut count = 0;
            for row in 0..self.height {
                match self.get(column, row) {
                    Rock::Round => {
                        self.set(column, row, Rock::None);
                        count += 1
                    }
                    Rock::Square => {
                        for delta_y in 1..=count {
                            self.set(column, row - delta_y, Rock::Round)
                        }

                        count = 0;
                    }
                    Rock::None => (),
                }
            }

            for y in 1..=count {
                self.set(column, self.height - y, Rock::Round)
            }
        }

        self
    }

    fn roll_west(&mut self) -> &mut Self {
        for row in 0..self.height {
            let mut count = 0;
            for column in (0..self.width).rev() {
                match self.get(column, row) {
                    Rock::Round => {
                        self.set(column, row, Rock::None);
                        count += 1
                    }
                    Rock::Square => {
                        for delta_x in 1..=count {
                            self.set(column + delta_x, row, Rock::Round)
                        }

                        count = 0;
                    }
                    Rock::None => (),
                }
            }

            for x in 0..count {
                self.set(x, row, Rock::Round)
            }
        }

        self
    }

    fn roll_east(&mut self) -> &mut Self {
        for row in 0..self.height {
            let mut count = 0;
            for column in 0..self.width {
                match self.get(column, row) {
                    Rock::Round => {
                        self.set(column, row, Rock::None);
                        count += 1
                    }
                    Rock::Square => {
                        for delta_x in 1..=count {
                            self.set(column - delta_x, row, Rock::Round)
                        }

                        count = 0;
                    }
                    Rock::None => (),
                }
            }

            for y in 1..=count {
                self.set(self.width - y, row, Rock::Round)
            }
        }

        self
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Rock {
    Round,
    Square,
    None,
}
