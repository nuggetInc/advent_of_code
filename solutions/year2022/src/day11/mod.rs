use std::collections::VecDeque;

use aoc_core::{AocResult, Day};
use regex::Regex;

pub fn day() -> Day {
    let mut solution = Day::new(11);
    solution.part_1(parse, part_one);
    solution.part_2(parse, part_two);
    solution.add_file("files/input.in");
    solution
}

fn part_one(monkeys: Vec<Monkey>) -> AocResult<i32> {
    let mut monkeys = monkeys.clone();

    let mut inspects = vec![0; monkeys.len()];

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(mut item) = monkeys[i].items.pop_front() {
                match monkeys[i].operation {
                    Operation::Add(value) => item += value,
                    Operation::Multiply(value) => item *= value,
                    Operation::Square => item *= item,
                }

                item /= 3;

                if item % monkeys[i].test == 0 {
                    let index = monkeys[i].success;
                    monkeys[index].items.push_back(item);
                } else {
                    let index = monkeys[i].fail;
                    monkeys[index].items.push_back(item);
                }

                inspects[i] += 1;
            }
        }
    }

    inspects.sort_by(|a, b| b.partial_cmp(a).unwrap());

    Ok(inspects[0] * inspects[1])
}

fn part_two(monkeys: Vec<Monkey>) -> AocResult<i64> {
    let mut monkeys = monkeys.clone();

    let mut inspects = vec![0_i64; monkeys.len()];

    let product = monkeys.iter().map(|m| m.test).product::<u64>();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while let Some(mut item) = monkeys[i].items.pop_front() {
                match monkeys[i].operation {
                    Operation::Add(value) => item += value,
                    Operation::Multiply(value) => item *= value,
                    Operation::Square => item *= item,
                }

                item %= product;

                if item % monkeys[i].test == 0 {
                    let index = monkeys[i].success;
                    monkeys[index].items.push_back(item);
                } else {
                    let index = monkeys[i].fail;
                    monkeys[index].items.push_back(item);
                }

                inspects[i] += 1;
            }
        }
    }

    inspects.sort_by(|a, b| b.partial_cmp(a).unwrap());

    Ok(inspects[0] * inspects[1])
}

fn parse(input: String) -> Vec<Monkey> {
    let mut monkeys = Vec::new();

    // let monkey_regex = Regex::new(r"^Monkey (?P<ident>\d+):$").unwrap();
    let starting_items_regex = Regex::new(r"^  Starting items: (?P<items>\d+(, \d+)*)$").unwrap();
    let operation_regex =
        Regex::new(r"^  Operation: new = old (?P<operator>[\*\+]) (?P<amount>\w+)$").unwrap();
    let test_regex = Regex::new(r"^  Test: divisible by (?P<amount>\d+)$").unwrap();
    let true_regex = Regex::new(r"^    If true: throw to monkey (?P<ident>\d+)$").unwrap();
    let false_regex = Regex::new(r"^    If false: throw to monkey (?P<ident>\d+)$").unwrap();

    for lines in input.split("\n\n") {
        let mut split = lines.split_terminator('\n');

        split.next();

        let line = split.next().unwrap();
        let captures = starting_items_regex.captures(line).unwrap();
        let items = captures["items"]
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect();

        let line = split.next().unwrap();
        let captures = operation_regex.captures(line).unwrap();
        let operation = match (&captures["operator"], &captures["amount"]) {
            ("*", "old") => Operation::Square,
            ("*", amount) => Operation::Multiply(amount.parse().unwrap()),
            ("+", amount) => Operation::Add(amount.parse().unwrap()),
            _ => unimplemented!(),
        };

        let line = split.next().unwrap();
        let captures = test_regex.captures(line).unwrap();
        let test = captures["amount"].parse().unwrap();

        let line = split.next().unwrap();
        let captures = true_regex.captures(line).unwrap();
        let success = captures["ident"].parse().unwrap();

        let line = split.next().unwrap();
        let captures = false_regex.captures(line).unwrap();
        let fail = captures["ident"].parse().unwrap();

        let monkey = Monkey::new(items, operation, test, success, fail);
        monkeys.push(monkey);
    }

    monkeys
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: u64,
    success: usize,
    fail: usize,
}

impl Monkey {
    fn new(
        items: VecDeque<u64>,
        operation: Operation,
        test: u64,
        success: usize,
        fail: usize,
    ) -> Self {
        Self {
            items,
            operation,
            test,
            success,
            fail,
        }
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}
