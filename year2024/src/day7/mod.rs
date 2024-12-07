use aoc_core::{AocResult, Day};
use itertools::Itertools;

pub fn day() -> Day {
    let mut solution = Day::new();
    solution.part_1(|input| part(input, false));
    solution.part_2(|input| part(input, true));
    solution.add_file("files/test.in");
    solution.add_file("files/input.in");
    solution
}

fn parse(input: &String) -> Vec<(u64, Vec<u64>)> {
    input
        .split_terminator('\n')
        .map(|line| {
            let (result, numbers) = line.split_once(": ").unwrap();
            let result = result.parse::<u64>().unwrap();
            let numbers = numbers
                .split(" ")
                .map(|num| num.parse::<u64>().unwrap())
                .collect_vec();

            (result, numbers)
        })
        .collect_vec()
}

fn part(input: &String, part_two: bool) -> AocResult<u64> {
    let lines = parse(input);

    let mut sum = 0;
    for (target, numbers) in lines {
        if solve(target, 0, &numbers, part_two) {
            sum += target;
        }
    }

    Ok(sum)
}

fn solve(target: u64, current: u64, remaining: &[u64], part_two: bool) -> bool {
    if current > target {
        return false;
    }

    match remaining {
        [] => current == target,
        [next, remaining @ ..] if part_two => {
            solve(target, current + next, remaining, part_two)
                || solve(target, current * next, remaining, part_two)
                || solve(
                    target,
                    current * 10_u64.pow(next.ilog10() + 1) + next,
                    remaining,
                    part_two,
                )
        }
        [next, remaining @ ..] => {
            solve(target, current + next, remaining, part_two)
                || solve(target, current * next, remaining, part_two)
        }
    }
}
