use aoc_core::{AocResult, Day};
use regex::Regex;

pub fn day() -> Day {
    let mut solution = Day::new();
    solution.part_1(part_one);
    solution.part_2(part_two);
    solution.add_file("files/input.in");
    solution
}

fn part_one(input: &String) -> AocResult<String> {
    let (crates, procedures) = input.split_once("\n\n").unwrap();

    let mut lines: Vec<&str> = crates.split_terminator('\n').collect();

    let mut stacks: Vec<Vec<char>> = Vec::new();

    let stack_count = lines.pop().unwrap().len() / 4 + 1;
    for _ in 0..stack_count {
        stacks.push(Vec::new());
    }

    while let Some(line) = lines.pop() {
        let line: Vec<char> = line.chars().collect();

        for i in 0..stack_count {
            if line[i * 4 + 1] == ' ' {
                continue;
            }

            stacks[i].push(line[i * 4 + 1]);
        }
    }

    let regex = Regex::new(r"^move (?P<amount>\d+) from (?P<from>\d+) to (?P<to>\d+)$").unwrap();

    for procedure in procedures.split_terminator('\n') {
        let captures = regex.captures(procedure).unwrap();

        let amount: usize = captures["amount"].parse().unwrap();
        let from: usize = captures["from"].parse().unwrap();
        let to: usize = captures["to"].parse().unwrap();

        for _ in 0..amount {
            let pop = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(pop);
        }
    }

    let mut output = "".to_string();

    for mut stack in stacks {
        let pop = stack.pop().unwrap();
        output.push(pop);
    }

    Ok(output)
}

fn part_two(input: &String) -> AocResult<String> {
    let (crates, procedures) = input.split_once("\n\n").unwrap();

    let mut lines: Vec<&str> = crates.split_terminator('\n').collect();

    let mut stacks: Vec<Vec<char>> = Vec::new();

    let stack_count = lines.pop().unwrap().len() / 4 + 1;
    for _ in 0..stack_count {
        stacks.push(Vec::new());
    }

    while let Some(line) = lines.pop() {
        let line: Vec<char> = line.chars().collect();

        for i in 0..stack_count {
            if line[i * 4 + 1] == ' ' {
                continue;
            }

            stacks[i].push(line[i * 4 + 1]);
        }
    }

    let regex = Regex::new(r"^move (?P<amount>\d+) from (?P<from>\d+) to (?P<to>\d+)$").unwrap();

    for procedure in procedures.split_terminator('\n') {
        let captures = regex.captures(procedure).unwrap();

        let amount: usize = captures["amount"].parse().unwrap();
        let from: usize = captures["from"].parse().unwrap();
        let to: usize = captures["to"].parse().unwrap();

        let mut temp = Vec::new();

        for _ in 0..amount {
            let pop = stacks[from - 1].pop().unwrap();
            temp.push(pop);
        }

        for _ in 0..amount {
            let pop = temp.pop().unwrap();
            stacks[to - 1].push(pop);
        }
    }

    let mut output = "".to_string();

    for mut stack in stacks {
        let pop = stack.pop().unwrap();
        output.push(pop);
    }

    Ok(output)
}
