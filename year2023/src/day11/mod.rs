use aoc_core::{AocResult, Day};
use itertools::Itertools;

pub fn day() -> Day {
    let mut solution = Day::new(11);
    solution.part_1(part_one);
    solution.part_2(part_two);
    solution.add_file("files/test.in");
    solution.add_file("files/input.in");
    solution
}

fn parse(input: &String) -> (Vec<Galaxy>, Vec<bool>, Vec<bool>) {
    let mut galaxies = Vec::new();
    let mut galaxies_x = Vec::new();
    let mut galaxies_y = Vec::new();

    for (y, line) in input.split_terminator('\n').enumerate() {
        galaxies_y.push(false);
        if y == 0 {
            galaxies_x = vec![false; line.len()];
        }

        for (x, char) in line.char_indices() {
            if char != '#' {
                continue;
            }

            let galaxy = Galaxy::new(x, y);
            galaxies.push(galaxy);
            galaxies_x[x] = true;
            galaxies_y[y] = true;
        }
    }

    (galaxies, galaxies_x, galaxies_y)
}

fn part_one(input: &String) -> AocResult<u64> {
    let (galaxies, galaxies_x, galaxies_y) = parse(input);

    Ok(calculate_distances(galaxies, galaxies_x, galaxies_y, 1))
}

fn part_two(input: &String) -> AocResult<u64> {
    let (galaxies, galaxies_x, galaxies_y) = parse(input);

    Ok(calculate_distances(
        galaxies, galaxies_x, galaxies_y, 999999,
    ))
}

fn calculate_distances(
    galaxies: Vec<Galaxy>,
    galaxies_x: Vec<bool>,
    galaxies_y: Vec<bool>,
    size_increase: u64,
) -> u64 {
    let mut cache_x = vec![0; galaxies_x.len() * galaxies_x.len()];

    for from in 0..(galaxies_x.len() - 1) {
        for to in (from + 1)..galaxies_x.len() {
            let mut sum = 0;

            for x in galaxies_x.iter().take(to).skip(from) {
                if !*x {
                    sum += size_increase;
                }
            }

            cache_x[from + to * galaxies_x.len()] = sum;
        }
    }

    let mut cache_y = vec![0; galaxies_y.len() * galaxies_y.len()];

    for from in 0..(galaxies_y.len() - 1) {
        for to in (from + 1)..galaxies_y.len() {
            let mut sum = 0;

            for y in galaxies_y.iter().take(to).skip(from) {
                if !*y {
                    sum += size_increase;
                }
            }

            cache_y[from + to * galaxies_y.len()] = sum;
        }
    }

    galaxies
        .iter()
        .tuple_combinations()
        .map(|(from, to)| {
            let mut sum = from.x.abs_diff(to.x) as u64 + from.y.abs_diff(to.y) as u64;

            sum += cache_x[from.x.min(to.x) + from.x.max(to.x) * galaxies_x.len()];
            sum += cache_y[from.y + to.y * galaxies_y.len()];

            sum
        })
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Galaxy {
    x: usize,
    y: usize,
}

impl Galaxy {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
