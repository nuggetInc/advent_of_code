use std::ops::RangeInclusive;

use aoc_core::{AocResult, Day};

pub fn day() -> Day {
    let mut solution = Day::new();
    solution.part_1(part_one);
    solution.part_2(part_two);
    solution.add_file("files/test.in");
    solution.add_file("files/input.in");
    solution
}

fn parse(input: &String) -> Vec<(Direction, i32, Direction, i32)> {
    input
        .split_terminator('\n')
        .map(|line| {
            let mut split = line.split(' ');

            let dir = match split.next().unwrap() {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => unreachable!(),
            };

            let meters = split.next().unwrap().parse().unwrap();

            let color = split.next().unwrap();

            let dir2 = match &color[7..8] {
                "0" => Direction::Right,
                "1" => Direction::Down,
                "2" => Direction::Left,
                "3" => Direction::Up,
                _ => unreachable!(),
            };

            let meters2 = i32::from_str_radix(&color[2..7], 16).unwrap();

            (dir, meters, dir2, meters2)
        })
        .collect()
}

fn part_one(input: &String) -> AocResult<usize> {
    let commands = parse(input);

    let mut map = Map::new();

    let mut start = None;
    let mut last = None;

    let mut x = 0;
    let mut y = 0;

    for (dir, meters, _, _) in commands {
        if start.is_none() {
            start = Some(dir);
        }

        let (columns, rows) = match dir {
            Direction::Up => {
                let column = map.get_column_index(x);
                let start_row = map.split_row_around(y - meters);
                let end_row = map.get_row_index(y);

                (column..=column, start_row..=end_row)
            }
            Direction::Down => {
                let column = map.get_column_index(x);
                let start_row = map.get_row_index(y);
                let end_row = map.split_row_around(y + meters);

                (column..=column, start_row..=end_row)
            }
            Direction::Left => {
                let start_column = map.split_column_around(x - meters);
                let end_column = map.get_column_index(x);
                let row = map.get_row_index(y);

                (start_column..=end_column, row..=row)
            }
            Direction::Right => {
                let start_column = map.get_column_index(x);
                let end_column = map.split_column_around(x + meters);
                let row = map.get_row_index(y);

                (start_column..=end_column, row..=row)
            }
        };

        for row_index in rows {
            for column_index in columns.clone() {
                map.grid[row_index][column_index] = Some((dir, dir));
            }
        }

        if let Some(last) = last {
            map.insert(x..=x, y..=y, Some((last, dir)));
        }

        match dir {
            Direction::Up => y -= meters,
            Direction::Down => y += meters,
            Direction::Left => x -= meters,
            Direction::Right => x += meters,
        }

        last = Some(dir);
    }

    map.insert(x..=x, y..=y, Some((last.unwrap(), start.unwrap())));

    let mut count = 0;
    for y in 0..map.height {
        let mut do_count = false;
        let mut last_open: Option<Direction> = None;

        for x in 0..map.width {
            if let Some((from, to)) = map.grid[y][x] {
                match (from, to) {
                    (Direction::Up, Direction::Up) | (Direction::Down, Direction::Down) => {
                        do_count = !do_count
                    }
                    (Direction::Left, Direction::Left) | (Direction::Right, Direction::Right) => (),

                    (Direction::Down, Direction::Right) | (Direction::Left, Direction::Up) => {
                        last_open = Some(Direction::Down);
                    }
                    (Direction::Up, Direction::Right) | (Direction::Left, Direction::Down) => {
                        last_open = Some(Direction::Up);
                    }

                    (Direction::Down, Direction::Left) | (Direction::Right, Direction::Up) => {
                        if last_open == Some(Direction::Up) {
                            do_count = !do_count
                        }
                    }
                    (Direction::Up, Direction::Left) | (Direction::Right, Direction::Down) => {
                        if last_open == Some(Direction::Down) {
                            do_count = !do_count
                        }
                    }

                    (Direction::Up, Direction::Down) => (),
                    (Direction::Down, Direction::Up) => (),
                    (Direction::Left, Direction::Right) => (),
                    (Direction::Right, Direction::Left) => (),
                }

                count += map.column_ranges[x].clone().count() * map.row_ranges[y].clone().count();
            } else if do_count {
                count += map.column_ranges[x].clone().count() * map.row_ranges[y].clone().count();
            }
        }
    }

    Ok(count)
}

fn part_two(input: &String) -> AocResult<usize> {
    let commands = parse(input);

    let mut map = Map::new();

    let mut start = None;
    let mut last = None;

    let mut x = 0;
    let mut y = 0;

    for (_, _, dir, meters) in commands {
        if start.is_none() {
            start = Some(dir);
        }

        let (columns, rows) = match dir {
            Direction::Up => {
                let column = map.get_column_index(x);

                let start_row = map.split_row_around(y - meters);
                let end_row = map.get_row_index(y);

                (column..=column, start_row..=end_row)
            }
            Direction::Down => {
                let column = map.get_column_index(x);

                let start_row = map.get_row_index(y);
                let end_row = map.split_row_around(y + meters);

                (column..=column, start_row..=end_row)
            }
            Direction::Left => {
                let start_column = map.split_column_around(x - meters);
                let end_column = map.get_column_index(x);

                let row = map.get_row_index(y);

                (start_column..=end_column, row..=row)
            }
            Direction::Right => {
                let start_column = map.get_column_index(x);
                let end_column = map.split_column_around(x + meters);

                let row = map.get_row_index(y);

                (start_column..=end_column, row..=row)
            }
        };

        for row_index in rows {
            for column_index in columns.clone() {
                map.grid[row_index][column_index] = Some((dir, dir));
            }
        }

        if let Some(last) = last {
            map.insert(x..=x, y..=y, Some((last, dir)));
        }

        match dir {
            Direction::Up => y -= meters,
            Direction::Down => y += meters,
            Direction::Left => x -= meters,
            Direction::Right => x += meters,
        }

        last = Some(dir);
    }

    map.insert(x..=x, y..=y, Some((last.unwrap(), start.unwrap())));

    let mut count = 0;
    for y in 0..map.height {
        let mut do_count = false;
        let mut last_open: Option<Direction> = None;

        for x in 0..map.width {
            if let Some((from, to)) = map.grid[y][x] {
                match (from, to) {
                    (Direction::Up, Direction::Up) | (Direction::Down, Direction::Down) => {
                        do_count = !do_count
                    }
                    (Direction::Left, Direction::Left) | (Direction::Right, Direction::Right) => (),

                    (Direction::Down, Direction::Right) | (Direction::Left, Direction::Up) => {
                        last_open = Some(Direction::Down);
                    }
                    (Direction::Up, Direction::Right) | (Direction::Left, Direction::Down) => {
                        last_open = Some(Direction::Up);
                    }

                    (Direction::Down, Direction::Left) | (Direction::Right, Direction::Up) => {
                        if last_open == Some(Direction::Up) {
                            do_count = !do_count
                        }
                    }
                    (Direction::Up, Direction::Left) | (Direction::Right, Direction::Down) => {
                        if last_open == Some(Direction::Down) {
                            do_count = !do_count
                        }
                    }

                    (Direction::Up, Direction::Down) => (),
                    (Direction::Down, Direction::Up) => (),
                    (Direction::Left, Direction::Right) => (),
                    (Direction::Right, Direction::Left) => (),
                }

                count += map.column_ranges[x].clone().count() * map.row_ranges[y].clone().count();
            } else if do_count {
                count += map.column_ranges[x].clone().count() * map.row_ranges[y].clone().count();
            }
        }
    }

    Ok(count)
}

struct Map {
    grid: Vec<Vec<Option<(Direction, Direction)>>>,
    column_ranges: Vec<RangeInclusive<i32>>,
    row_ranges: Vec<RangeInclusive<i32>>,
    width: usize,
    height: usize,
}

impl Map {
    fn new() -> Self {
        Map {
            grid: vec![vec![None]],
            column_ranges: vec![0..=0],
            row_ranges: vec![0..=0],
            width: 1,
            height: 1,
        }
    }

    fn get_column_index(&self, x: i32) -> usize {
        for (index, range) in self.column_ranges.iter().enumerate() {
            if range.contains(&x) {
                return index;
            }
        }

        unimplemented!()
    }

    fn get_row_index(&self, y: i32) -> usize {
        for (index, range) in self.row_ranges.iter().enumerate() {
            if range.contains(&y) {
                return index;
            }
        }

        unimplemented!()
    }

    fn insert(
        &mut self,
        x: RangeInclusive<i32>,
        y: RangeInclusive<i32>,
        value: Option<(Direction, Direction)>,
    ) {
        let columns = self.split_column_around(*x.start())..=self.split_column_around(*x.end());
        let rows = self.split_row_around(*y.start())..=self.split_row_around(*y.end());

        for row_index in rows {
            for column_index in columns.clone() {
                self.grid[row_index][column_index] = value;
            }
        }
    }

    fn split_column_around(&mut self, x: i32) -> usize {
        let first = self.column_ranges.first().unwrap();
        if x < *first.start() {
            if x < first.start() - 1 {
                self.column_ranges.insert(0, x + 1..=first.start() - 1);

                for y in 0..self.height {
                    self.grid[y].insert(0, None);
                }

                self.width += 1;
            }

            self.column_ranges.insert(0, x..=x);
            for y in 0..self.height {
                self.grid[y].insert(0, None);
            }

            self.width += 1;
            return 0;
        }

        for index in 0..self.column_ranges.len() {
            if x == *self.column_ranges[index].start() && x == *self.column_ranges[index].end() {
                return index;
            } else if x > *self.column_ranges[index].start() && x < *self.column_ranges[index].end()
            {
                let range = self.column_ranges[index].clone();

                self.column_ranges[index] = x + 1..=*range.end();
                self.column_ranges.insert(index, x..=x);
                self.column_ranges.insert(index, *range.start()..=x - 1);

                for y in 0..self.height {
                    let value = self.grid[y][index];
                    self.grid[y].insert(index, value);
                    self.grid[y].insert(index, value);
                }

                self.width += 2;
                return index + 1;
            } else if x == *self.column_ranges[index].start() {
                let range = self.column_ranges[index].clone();

                self.column_ranges[index] = x + 1..=*range.end();
                self.column_ranges.insert(index, x..=x);

                for y in 0..self.height {
                    let value = self.grid[y][index];
                    self.grid[y].insert(index, value);
                }

                self.width += 1;
                return index;
            } else if x == *self.column_ranges[index].end() {
                let range = self.column_ranges[index].clone();

                self.column_ranges[index] = x..=x;
                self.column_ranges.insert(index, *range.start()..=x - 1);

                for y in 0..self.height {
                    let value = self.grid[y][index];
                    self.grid[y].insert(index, value);
                }

                self.width += 1;
                return index + 1;
            }
        }

        let last = self.column_ranges.last().unwrap();

        if x > *last.end() + 1 {
            self.column_ranges.push(last.end() + 1..=x - 1);

            for y in 0..self.height {
                self.grid[y].push(None);
            }

            self.width += 1;
        }

        self.column_ranges.push(x..=x);

        for y in 0..self.height {
            self.grid[y].push(None);
        }

        self.width += 1;
        self.width - 1
    }

    fn split_row_around(&mut self, y: i32) -> usize {
        let first = self.row_ranges.first().unwrap();
        if y < *first.start() {
            if y < first.start() - 1 {
                self.row_ranges.insert(0, y + 1..=first.start() - 1);
                self.grid.insert(0, vec![None; self.width]);

                self.height += 1;
            }

            self.row_ranges.insert(0, y..=y);
            self.grid.insert(0, vec![None; self.width]);

            self.height += 1;
            return 0;
        }

        for index in 0..self.row_ranges.len() {
            if y == *self.row_ranges[index].start() && y == *self.row_ranges[index].end() {
                return index;
            } else if y > *self.row_ranges[index].start() && y < *self.row_ranges[index].end() {
                let range = self.row_ranges[index].clone();

                self.row_ranges[index] = y + 1..=*range.end();
                self.row_ranges.insert(index, y..=y);
                self.row_ranges.insert(index, *range.start()..=y - 1);

                self.grid.insert(index, self.grid[index].clone());
                self.grid.insert(index, self.grid[index].clone());

                self.height += 2;
                return index + 1;
            } else if y == *self.row_ranges[index].start() {
                let range = self.row_ranges[index].clone();

                self.row_ranges[index] = y + 1..=*range.end();
                self.row_ranges.insert(index, y..=y);

                self.grid.insert(index, self.grid[index].clone());

                self.height += 1;
                return index;
            } else if y == *self.row_ranges[index].end() {
                let range = self.row_ranges[index].clone();

                self.row_ranges[index] = y..=y;
                self.row_ranges.insert(index, *range.start()..=y - 1);

                self.grid.insert(index, self.grid[index].clone());

                self.height += 1;
                return index + 1;
            }
        }

        let last = self.row_ranges.last().unwrap();
        if y > *last.end() + 1 {
            self.row_ranges.push(last.end() + 1..=y - 1);
            self.grid.push(vec![None; self.width]);

            self.height += 1;
        }

        self.row_ranges.push(y..=y);

        self.grid.push(vec![None; self.width]);

        self.height += 1;
        self.height - 1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
