use std::collections::{HashSet, VecDeque};

use aoc_core::{Day, YearDay};

pub fn day() -> Day {
    let mut solution = Day::new(YearDay::Day06);
    solution.part_1(|x| x, part_one);
    solution.part_2(|x| x, part_two);
    solution.add_file("input.txt");
    solution
}

fn part_one(input: String) -> String {
    find_marker(input, 4).to_string()
}

fn part_two(input: String) -> String {
    find_marker(input, 14).to_string()
}

fn find_marker(input: String, size: usize) -> usize {
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
