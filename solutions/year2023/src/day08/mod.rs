use aoc_core::{Day, YearDay};
use num::Integer;

pub fn day() -> Day {
    let mut solution = Day::new(YearDay::Day08);
    solution.part_1(parse, part_one);
    solution.part_2(parse, part_two);
    solution.add_file("test.txt");
    solution.add_file("input.txt");
    solution
}

fn parse(input: String) -> (Vec<Direction>, Vec<(usize, usize)>) {
    let mut lines = input.split_terminator('\n');

    let directions = lines.next().unwrap().chars().map(Direction::from).collect();

    let mut nodes = vec![(0, 0); parse_node("ZZZ") + 1];

    for line in lines.skip(1) {
        let node = parse_node(&line[0..3]);
        let left = parse_node(&line[7..10]);
        let right = parse_node(&line[12..15]);

        nodes[node] = (left, right)
    }

    (directions, nodes)
}

fn parse_node(input: &str) -> usize {
    usize::from_str_radix(input, 36).unwrap()
}

fn part_one((directions, nodes): (Vec<Direction>, Vec<(usize, usize)>)) -> String {
    let mut position = parse_node("AAA");
    let mut steps = 0;

    for direction in directions.iter().cycle() {
        position = match direction {
            Direction::Left => nodes[position].0,
            Direction::Right => nodes[position].1,
        };

        steps += 1;
        // Same as parse_node("ZZZ")
        if position == 46655 {
            break;
        }
    }

    steps.to_string()
}

fn part_two((directions, nodes): (Vec<Direction>, Vec<(usize, usize)>)) -> String {
    let mut multiple = 1_u64;

    for mut position in (parse_node("11A")..=parse_node("ZZA")).step_by(36) {
        if nodes[position] == (0, 0) {
            continue;
        }

        let mut steps = 0;

        for direction in directions.iter().cycle() {
            position = match direction {
                Direction::Left => nodes[position].0,
                Direction::Right => nodes[position].1,
            };

            steps += 1;
            // Same as checking if it ends with a 'Z'
            if position % 36 == 35 {
                break;
            }
        }

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
