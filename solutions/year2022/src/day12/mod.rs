use std::collections::{HashSet, VecDeque};

use aoc_core::Day;

pub fn day() -> Day {
    let mut solution = Day::new(12);
    solution.part_1(parse, part_one);
    solution.part_2(parse, part_two);
    solution.add_file("input.txt");
    solution
}

fn part_one(map: Map) -> String {
    map.get_steps().to_string()
}

fn part_two(map: Map) -> String {
    map.get_fewest_steps().to_string()
}

fn parse(input: String) -> Map {
    let lines = input.split_terminator("\n").collect::<Vec<_>>();

    let mut start = 0;
    let mut end = 0;
    let mut width = 0;
    let height = lines.len();
    let mut grid = Vec::new();

    for (y, line) in lines.into_iter().enumerate() {
        width = line.len();

        for (x, char) in line.chars().enumerate() {
            match char {
                'S' => {
                    start = x + y * width;
                    grid.push('a' as u8);
                }
                'E' => {
                    end = x + y * width;
                    grid.push('z' as u8);
                }
                _ => grid.push(char as u8),
            }
        }
    }
    Map::new(start, end, width, height, grid)
}

struct Map {
    start: usize,
    end: usize,
    width: usize,
    height: usize,
    grid: Vec<u8>,
}

impl Map {
    fn new(start: usize, end: usize, width: usize, height: usize, grid: Vec<u8>) -> Self {
        Self {
            start,
            end,
            width,
            height,
            grid,
        }
    }

    fn get_steps(&self) -> usize {
        let mut distances = vec![usize::MAX / 2; self.width * self.height];
        distances[self.start] = 0;

        let mut queued = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(self.start);
        queued.insert(self.start);

        while let Some(position) = queue.pop_front() {
            queued.remove(&position);

            for neighbor in self.get_neighbors(position) {
                if self.grid[position] + 1 >= self.grid[neighbor]
                    && distances[position] + 1 < distances[neighbor]
                {
                    distances[neighbor] = distances[position] + 1;

                    if queued.insert(neighbor) {
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        distances[self.end]
    }

    fn get_fewest_steps(&self) -> usize {
        let mut distances = vec![usize::MAX / 2; self.width * self.height];
        distances[self.end] = 0;

        let mut queued = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(self.end);
        queued.insert(self.end);

        while let Some(position) = queue.pop_front() {
            queued.remove(&position);

            for neighbor in self.get_neighbors(position) {
                if self.grid[position] <= self.grid[neighbor] + 1
                    && distances[position] + 1 < distances[neighbor]
                {
                    distances[neighbor] = distances[position] + 1;

                    if queued.insert(neighbor) {
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        let mut nearest = usize::MAX;

        for (position, height) in self.grid.iter().enumerate() {
            if *height != 'a' as u8 {
                continue;
            }

            if distances[position] < nearest {
                nearest = distances[position];
            }
        }

        nearest
    }

    fn get_neighbors(&self, position: usize) -> Vec<usize> {
        let mut neighbors = Vec::with_capacity(4);

        if position % self.width > 0 {
            neighbors.push(position - 1);
        }

        if position % self.width < self.width - 1 {
            neighbors.push(position + 1);
        }

        if position / self.width > 0 {
            neighbors.push(position - self.width);
        }

        if position / self.width < self.height - 1 {
            neighbors.push(position + self.width);
        }

        neighbors
    }
}
