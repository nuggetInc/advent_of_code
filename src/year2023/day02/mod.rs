use std::collections::HashMap;

use aoc_core::{AocDay, Day};
use regex::Regex;

pub fn day02() -> impl AocDay {
    let mut solution = Day::new(parse);
    solution.add_part("1".into(), part_1);
    solution.add_part("2".into(), part_2);
    solution.add_file("src/year2023/day02/test.txt");
    solution.add_file("src/year2023/day02/input.txt");
    solution
}

fn parse(input: String) -> Vec<Game> {
    let regex = Regex::new(r"^Game (\d+): (.+)$").unwrap();

    let mut games = Vec::new();
    for line in input.split_terminator('\n') {
        let captures = regex.captures(line).unwrap();

        let mut sets = Vec::new();
        let id = captures[1].parse().unwrap();
        for set_raw in captures[2].split("; ") {
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

fn part_1(games: &Vec<Game>) -> String {
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
                    // println!("{color:?}: {amount}");

                    sum -= game.id;
                    break 'outer;
                }
            }
        }
    }

    sum.to_string()
}

fn part_2(games: &Vec<Game>) -> String {
    let mut sum = 0;

    for game in games {
        let mut hashmap = HashMap::new();
        hashmap.insert(Color::Red, 0);
        hashmap.insert(Color::Green, 0);
        hashmap.insert(Color::Blue, 0);

        for set in &game.sets {
            for (color, amount) in &set.cubes {
                if hashmap[color] < *amount {
                    hashmap.insert(color.to_owned(), *amount);
                }
            }
        }

        sum += hashmap[&Color::Red] * hashmap[&Color::Green] * hashmap[&Color::Blue];
    }

    sum.to_string()
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
