use aoc_core::{AocResult, Day};
use itertools::Itertools;

pub fn day() -> Day {
    let mut solution = Day::new(13);
    solution.part_1(|s: String| part_one(parse(s)));
    solution.part_2(|s: String| part_two(parse(s)));
    solution.add_file("files/test.in");
    solution.add_file("files/input.in");
    solution
}

fn parse(input: String) -> Vec<Pattern> {
    input
        .split_terminator("\n\n")
        .map(|block| {
            let mut width = 0;

            let grid = block
                .split_terminator('\n')
                .map(|line| {
                    width = line.len();
                    line.chars().map(|char| char == '#').collect()
                })
                .collect_vec();

            let height = grid.len();

            Pattern::new(grid, width, height)
        })
        .collect()
}

fn part_one(patterns: Vec<Pattern>) -> AocResult<usize> {
    Ok(patterns.into_iter().map(|p| p.find_mirror(false)).sum())
}

fn part_two(patterns: Vec<Pattern>) -> AocResult<usize> {
    Ok(patterns.into_iter().map(|p| p.find_mirror(true)).sum())
}

struct Pattern {
    grid: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Pattern {
    fn new(grid: Vec<Vec<bool>>, width: usize, height: usize) -> Self {
        Self {
            grid,
            width,
            height,
        }
    }

    fn find_mirror(&self, allow_smudge: bool) -> usize {
        'outer: for mirror_x in 0..self.width - 1 {
            let mut smudge = !allow_smudge;
            for y in 0..self.height {
                for x in 0..=mirror_x.min(self.width - mirror_x - 2) {
                    if self.grid[y][mirror_x - x] != self.grid[y][mirror_x + x + 1] {
                        if smudge {
                            continue 'outer;
                        }
                        smudge = true;
                    }
                }
            }

            if !smudge {
                continue;
            }

            return mirror_x + 1;
        }

        'outer: for mirror_y in 0..self.height - 1 {
            let mut smudge = !allow_smudge;
            for x in 0..self.width {
                for y in 0..=mirror_y.min(self.height - mirror_y - 2) {
                    if self.grid[mirror_y - y][x] != self.grid[mirror_y + y + 1][x] {
                        if smudge {
                            continue 'outer;
                        }
                        smudge = true;
                    }
                }
            }

            if !smudge {
                continue;
            }

            return mirror_y * 100 + 100;
        }

        0
    }
}
