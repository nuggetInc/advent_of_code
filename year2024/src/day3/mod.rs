use aoc_core::{AocResult, Day};
use regex::Regex;

pub fn day() -> Day {
    let mut solution = Day::new();
    solution.part_1(part_one);
    solution.part_2(part_two);
    solution.add_file("files/test.in");
    solution.add_file("files/input.in");
    solution
}

fn part_one(input: &String) -> AocResult<u32> {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut sum = 0;

    for captures in regex.captures_iter(input) {
        let a = captures[1].parse::<u32>().unwrap();
        let b = captures[2].parse::<u32>().unwrap();

        sum += a * b;
    }

    Ok(sum)
}

fn part_two(input: &String) -> AocResult<u32> {
    let regex = Regex::new(r"(mul|do|don't)\((?:(\d+),(\d+))?\)").unwrap();

    let mut enable = true;
    let mut sum = 0;

    for captures in regex.captures_iter(input) {
        match &captures[1] {
            "mul" => {
                if !enable {
                    continue;
                }

                let a = captures[2].parse::<u32>().unwrap();
                let b = captures[3].parse::<u32>().unwrap();

                sum += a * b;
            }
            "do" => enable = true,
            "don't" => enable = false,
            _ => unreachable!(),
        }
    }

    Ok(sum)
}
