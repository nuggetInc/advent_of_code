use aoc_core::{Day, YearDay};

pub fn day() -> Day {
    let mut solution = Day::new(YearDay::Day08);
    solution.part_1(parse, part_one);
    solution.part_2(parse, part_two);
    solution.add_file("input.txt");
    solution
}

fn parse(input: String) -> Grid {
    let grid: Vec<Vec<u8>> = input
        .split("\n")
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    let width = grid[0].len();
    let height = grid.len();

    Grid::new(grid, width, height)
}

fn part_one(grid: Grid) -> String {
    let mut visible = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.is_visible(x, y) {
                visible += 1;
            }
        }
    }

    visible.to_string()
}

fn part_two(grid: Grid) -> String {
    let mut highest_score = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            let score = grid.scenic_score(x, y);

            if score > highest_score {
                highest_score = score;
            }
        }
    }

    highest_score.to_string()
}

struct Grid {
    grid: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(grid: Vec<Vec<u8>>, width: usize, height: usize) -> Self {
        Self {
            grid,
            width,
            height,
        }
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        let Some(maxx1) = self.max_x(0..x, y) else {
            return true;
        };

        let Some(maxx2) = self.max_x(x + 1..self.width, y) else {
            return true;
        };
        let Some(maxy1) = self.max_y(x, 0..y) else {
            return true;
        };

        let Some(maxy2) = self.max_y(x, y + 1..self.height) else {
            return true;
        };

        return self.grid[y][x] > [maxx1, maxx2, maxy1, maxy2].into_iter().min().unwrap();
    }

    fn max_x(&self, x: impl Iterator<Item = usize>, y: usize) -> Option<u8> {
        let mut max = None;

        for x in x {
            let Some(temp) = max else {
                max = Some(self.grid[y][x]);
                continue;
            };

            if temp >= self.grid[y][x] {
                continue;
            }

            max = Some(self.grid[y][x]);
        }

        max
    }

    fn max_y(&self, x: usize, y: impl Iterator<Item = usize>) -> Option<u8> {
        let mut max = None;

        for y in y {
            let Some(temp) = max else {
                max = Some(self.grid[y][x]);
                continue;
            };

            if temp >= self.grid[y][x] {
                continue;
            }

            max = Some(self.grid[y][x]);
        }

        max
    }

    fn scenic_score(&self, x: usize, y: usize) -> u32 {
        let mut score = 1;

        score *= self.score_x((0..=x).rev(), y);
        score *= self.score_x(x..self.width, y);
        score *= self.score_y(x, (0..=y).rev());
        score *= self.score_y(x, y..self.height);

        score
    }

    fn score_x(&self, mut x: impl Iterator<Item = usize>, y: usize) -> u32 {
        let mut score = 0;

        let height = self.grid[y][x.next().unwrap()];

        for i in x {
            score += 1;

            if height <= self.grid[y][i] {
                break;
            }
        }

        score
    }

    fn score_y(&self, x: usize, mut y: impl Iterator<Item = usize>) -> u32 {
        let mut score = 0;

        let height = self.grid[y.next().unwrap()][x];

        for i in y {
            score += 1;

            if height <= self.grid[i][x] {
                break;
            }
        }

        score
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let grid: Vec<Vec<u8>> = value
            .split("\n")
            .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
            .collect();

        let width = grid[0].len();
        let height = grid.len();

        Grid::new(grid, width, height)
    }
}
