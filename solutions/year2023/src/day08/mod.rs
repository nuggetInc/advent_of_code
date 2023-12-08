use std::collections::HashMap;

use aoc_core::{Day, YearDay};
use num::Integer;

pub fn day() -> Day {
    let mut solution = Day::new(YearDay::Day05);
    solution.part_1(parse, part_one);
    solution.part_2(parse, part_two);
    solution.add_file("test.txt");
    solution.add_file("input.txt");
    solution
}

fn parse(input: String) -> (Vec<Direction>, HashMap<String, (String, String)>) {
    let mut lines = input.split_terminator('\n');

    let directions = lines.next().unwrap().chars().map(Direction::from).collect();

    let nodes = lines
        .skip(1)
        .map(|line| {
            let (name, to) = line.split_once(" = (").unwrap();
            let (left, right) = to.split_once(", ").unwrap();

            (name.to_owned(), (left.to_owned(), right[0..3].to_owned()))
        })
        .collect();

    (directions, nodes)
}

fn part_one((directions, nodes): (Vec<Direction>, HashMap<String, (String, String)>)) -> String {
    let mut position = &"AAA".to_owned();
    let mut steps = 0;

    for direction in directions.iter().cycle() {
        match direction {
            Direction::Left => position = &nodes[position].0,
            Direction::Right => position = &nodes[position].1,
        }

        steps += 1;
        if position == "ZZZ" {
            break;
        }
    }

    steps.to_string()
}

fn part_two((directions, nodes): (Vec<Direction>, HashMap<String, (String, String)>)) -> String {
    let mut step_counts = Vec::new();

    for mut position in nodes.keys().filter(|key| key.ends_with('A')) {
        let mut steps = 0;

        for direction in directions.iter().cycle() {
            match direction {
                Direction::Left => position = &nodes[position].0,
                Direction::Right => position = &nodes[position].1,
            }

            steps += 1;
            if position.ends_with('Z') {
                break;
            }
        }

        step_counts.push(steps);
    }

    let mut multiple = 1_u64;
    for steps in step_counts {
        multiple = multiple.lcm(&steps);
    }

    multiple.to_string()
}

enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!(),
        }
    }
}
