use std::collections::HashMap;

use aoc_core::{AocResult, Day};

pub fn day() -> Day {
    let mut solution = Day::new(2);
    solution.part_1(part_one);
    solution.part_2(part_two);
    solution.add_file("files/test.in");
    solution.add_file("files/input.in");
    solution
}

fn parse(input: &String) -> Vec<Game> {
    let mut games = Vec::new();
    for line in input.split_terminator('\n') {
        let mut split = line.split(": ");

        let mut sets = Vec::new();
        let id = split.next().unwrap()[5..].parse().unwrap();
        for set_raw in split.next().unwrap().split("; ") {
            let mut cubes = HashMap::new();

            for cube_raw in set_raw.split(", ") {
                let mut split = cube_raw.split(' ');

                let amount = split.next().unwrap().parse().unwrap();
                let color = match split.next().unwrap() {
                    "red" => Color::Red,
                    "green" => Color::Green,
                    "blue" => Color::Blue,
                    _ => unreachable!(),
                };

                cubes.insert(color, amount);
            }

            sets.push(Set::new(cubes));
        }

        games.push(Game::new(id, sets))
    }

    games
}

fn part_one(input: &String) -> AocResult<u32> {
    let games = parse(input);

    let mut sum = 0;

    for game in games {
        sum += game.id;
        'outer: for set in &game.sets {
            for (color, amount) in &set.cubes {
                let possible = match color {
                    Color::Red => *amount <= 12,
                    Color::Green => *amount <= 13,
                    Color::Blue => *amount <= 14,
                };

                if !possible {
                    sum -= game.id;
                    break 'outer;
                }
            }
        }
    }

    Ok(sum)
}

fn part_two(input: &String) -> AocResult<u32> {
    let games = parse(input);

    let mut sum = 0;

    for game in games {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for set in &game.sets {
            for (color, amount) in &set.cubes {
                match color {
                    Color::Red => red = red.max(*amount),
                    Color::Green => green = green.max(*amount),
                    Color::Blue => blue = blue.max(*amount),
                };
            }
        }

        sum += red * green * blue;
    }

    Ok(sum)
}

struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Game {
    fn new(id: u32, sets: Vec<Set>) -> Self {
        Self { id, sets }
    }
}

struct Set {
    cubes: HashMap<Color, u32>,
}

impl Set {
    fn new(cubes: HashMap<Color, u32>) -> Self {
        Self { cubes }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}
