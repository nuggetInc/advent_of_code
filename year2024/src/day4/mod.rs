use std::{error::Error, str::FromStr};

use aoc_core::{AocResult, Day};
use itertools::Itertools;

pub fn day() -> Day {
    let mut solution = Day::new();
    solution.part_1(part_one);
    solution.part_2(part_two);
    solution.add_file("files/test.in");
    solution.add_file("files/input.in");
    solution
}

fn part_one(input: &String) -> AocResult<u32> {
    let grid = Grid::from_str(input)?;

    let deltas = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let mut count = 0;

    for row in 0..grid.height as isize {
        for col in 0..grid.width as isize {
            for (delta_col, delta_row) in deltas {
                if grid.get(col, row) == Some('X')
                    && grid.get(col + delta_col, row + delta_row) == Some('M')
                    && grid.get(col + delta_col * 2, row + delta_row * 2) == Some('A')
                    && grid.get(col + delta_col * 3, row + delta_row * 3) == Some('S')
                {
                    count += 1;
                }
            }
        }
    }

    Ok(count)
}

fn part_two(input: &String) -> AocResult<u32> {
    let grid = Grid::from_str(input)?;

    let deltas = [(-1, -1), (1, -1), (-1, 1), (1, 1)];

    let mut count = 0;

    for row in 0..grid.height as isize {
        for col in 0..grid.width as isize {
            for (delta_col, delta_row) in deltas {
                if grid.get(col, row) == Some('A')
                    && grid.get(col - delta_col, row - delta_row) == Some('M')
                    && grid.get(col + delta_row, row - delta_col) == Some('M')
                    && grid.get(col + delta_col, row + delta_row) == Some('S')
                    && grid.get(col - delta_row, row + delta_col) == Some('S')
                {
                    count += 1;
                }
            }
        }
    }

    Ok(count)
}

struct Grid {
    data: Vec<char>,
    width: usize,
    height: usize,
}

impl Grid {
    fn get(&self, col: isize, row: isize) -> Option<char> {
        if col < 0 || col >= self.width as isize || row < 0 {
            return None;
        }

        self.data
            .get((col + row * self.width as isize) as usize)
            .cloned()
    }
}

impl FromStr for Grid {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut width = 0;
        let data = input
            .split_terminator('\n')
            .flat_map(|line| {
                width = line.len();
                line.chars()
            })
            .collect_vec();

        let height = data.len() / width;
        Ok(Grid {
            data,
            width,
            height,
        })
    }
}
