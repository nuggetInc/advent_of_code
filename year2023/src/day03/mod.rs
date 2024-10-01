use std::collections::HashSet;

use aoc_core::{AocResult, Day};

pub fn day() -> Day {
    let mut solution = Day::new(3);
    solution.part_1(part_one);
    solution.part_2(part_two);
    solution.add_file("files/test.in");
    solution.add_file("files/input.in");
    solution
}

fn parse(input: &String) -> (Vec<u32>, Vec<Vec<Cell>>) {
    let mut numbers = Vec::new();
    let mut map = Vec::new();

    for line in input.split_terminator('\n') {
        let mut row = Vec::with_capacity(line.len());

        let mut in_number = false;
        for char in line.chars() {
            if let Some(digit) = char.to_digit(10) {
                if in_number {
                    let current = numbers.last_mut().unwrap();
                    *current *= 10;
                    *current += digit;
                } else {
                    numbers.push(digit);
                    in_number = true
                }

                row.push(Cell::new(Some(numbers.len() - 1), None))
            } else if char != '.' {
                row.push(Cell::new(None, Some(char)));
                in_number = false;
            } else {
                row.push(Cell::new(None, None));
                in_number = false;
            }
        }

        map.push(row)
    }

    (numbers, map)
}

fn part_one(input: &String) -> AocResult<u32> {
    let (numbers, map) = parse(input);

    let mut marked = HashSet::new();

    for (row_index, row) in map.iter().enumerate() {
        for (cell_index, cell) in row.iter().enumerate() {
            let Some(number_index) = cell.number_index else {
                continue;
            };

            for x in (cell_index.max(1) - 1)..=(cell_index + 1).min(row.len() - 1) {
                for map_y in map
                    .iter()
                    .take((row_index + 1).min(map.len() - 1) + 1)
                    .skip(row_index.max(1) - 1)
                {
                    if map_y[x].symbol.is_some() {
                        marked.insert(number_index);
                    }
                }
            }
        }
    }

    Ok(marked.into_iter().map(|index| numbers[index]).sum::<u32>())
}

fn part_two(input: &String) -> AocResult<u32> {
    let (numbers, map) = parse(input);

    let mut sum = 0;
    for (row_index, row) in map.iter().enumerate() {
        for (cell_index, cell) in row.iter().enumerate() {
            let Some(symbol) = cell.symbol else {
                continue;
            };

            if symbol != '*' {
                continue;
            }

            let mut neighbors = HashSet::new();

            for x in (cell_index.max(1) - 1)..=(cell_index + 1).min(row.len() - 1) {
                for map_y in map
                    .iter()
                    .take((row_index + 1).min(map.len() - 1) + 1)
                    .skip(row_index.max(1) - 1)
                {
                    let Some(number_index) = map_y[x].number_index else {
                        continue;
                    };

                    neighbors.insert(number_index);
                }
            }

            if neighbors.len() == 2 {
                sum += neighbors
                    .into_iter()
                    .map(|index| numbers[index])
                    .product::<u32>();
            }
        }
    }

    Ok(sum)
}

struct Cell {
    number_index: Option<usize>,
    symbol: Option<char>,
}

impl Cell {
    fn new(number_index: Option<usize>, symbol: Option<char>) -> Self {
        Self {
            number_index,
            symbol,
        }
    }
}
