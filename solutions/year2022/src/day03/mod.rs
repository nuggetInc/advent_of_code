use std::collections::HashSet;

use aoc_core::Day;

pub fn day() -> Day {
    let mut solution = Day::new(3);
    solution.part_1(|x| x, part_one);
    solution.part_2(|x| x, part_two);
    solution.add_file("input.txt");
    solution
}

fn part_one(input: String) -> String {
    let mut total = 0;

    for line in input.split('\n') {
        let (first, last) = line.split_at(line.len() / 2);
        let first: HashSet<char> = first.chars().collect();
        let second: HashSet<char> = last.chars().collect();

        let intersection = first.intersection(&second).next().unwrap();
        if intersection.is_lowercase() {
            total += *intersection as u32 - 96
        } else {
            total += *intersection as u32 - 38
        };
    }

    total.to_string()
}

fn part_two(input: String) -> String {
    let mut total = 0;

    let mut split = input.split('\n');

    while let (Some(first), Some(second), Some(third)) = (split.next(), split.next(), split.next())
    {
        let first: HashSet<char> = first.chars().collect();
        let second: HashSet<char> = second.chars().collect();
        let third: HashSet<char> = third.chars().collect();

        let intersection: HashSet<char> = first.intersection(&second).map(char::clone).collect();
        let intersection = intersection.intersection(&third).next().unwrap();
        if intersection.is_lowercase() {
            total += *intersection as u32 - 96
        } else {
            total += *intersection as u32 - 38
        };
    }

    total.to_string()
}
