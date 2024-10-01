use std::collections::{HashSet, VecDeque};

use aoc_core::{AocResult, Day};

pub fn day() -> Day {
    let mut solution = Day::new();
    solution.part_1(part_one);
    solution.part_2(part_two);
    solution.add_file("files/input.in");
    solution
}

fn part_one(input: &String) -> AocResult<usize> {
    Ok(find_marker(input, 4))
}

fn part_two(input: &String) -> AocResult<usize> {
    Ok(find_marker(input, 14))
}

fn find_marker(input: &String, size: usize) -> usize {
    let mut iter = input.chars();

    let mut queue = VecDeque::with_capacity(4);
    for _ in 0..size - 1 {
        queue.push_back(iter.next().unwrap());
    }

    let mut start = size;

    for next in iter {
        queue.push_back(next);

        let mut uniq = HashSet::new();
        if queue.iter().all(|x| uniq.insert(x)) {
            break;
        }

        queue.pop_front();

        start += 1;
    }

    start
}
