use std::collections::{BTreeMap, BTreeSet};

use aoc_core::{AocResult, Day};
use itertools::Itertools;

pub fn day() -> Day {
    let mut solution = Day::new();
    solution.part_1(part_one);
    solution.part_2(part_two);
    solution.add_file("files/test.in");
    solution.add_file("files/input.in");
    solution
}

fn parse(input: &String) -> (BTreeMap<u32, BTreeSet<u32>>, Vec<Vec<u32>>) {
    let mut rules: BTreeMap<u32, BTreeSet<u32>> = BTreeMap::new();
    let mut updates = Vec::new();

    for line in input.split_terminator('\n') {
        if let Some((a, b)) = line.split_once('|') {
            let a = a.parse::<u32>().unwrap();
            let b = b.parse::<u32>().unwrap();

            if let Some(rule) = rules.get_mut(&a) {
                rule.insert(b);
            } else {
                rules.insert(a, BTreeSet::from([b]));
            }
        } else if line != "" {
            updates.push(
                line.split(',')
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect_vec(),
            );
        }
    }

    (rules, updates)
}

fn part_one(input: &String) -> AocResult<u32> {
    let (rules, updates) = parse(input);

    let mut sum = 0;

    'updates: for update in updates {
        for i in 1..update.len() {
            if let Some(after) = rules.get(&update[i]) {
                if update[0..i].iter().any(|x| after.contains(x)) {
                    continue 'updates;
                }
            }
        }

        sum += update[update.len() / 2];
    }

    Ok(sum)
}

fn part_two(input: &String) -> AocResult<u32> {
    let (rules, updates) = parse(input);

    let mut sum = 0;

    for mut update in updates {
        let mut correct = true;
        for _ in 1..update.len() {
            for i in 1..update.len() {
                if let Some(after) = rules.get(&update[i]) {
                    for j in 0..i {
                        if after.contains(&update[j]) {
                            let removed = update.remove(i);
                            update.insert(j, removed);
                            correct = false;
                        }
                    }
                }
            }
        }

        if !correct {
            sum += update[update.len() / 2];
        }
    }

    Ok(sum)
}
