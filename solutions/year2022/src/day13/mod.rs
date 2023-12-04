use std::cmp::Ordering;

use aoc_core::{AocDay, Day, YearDay};

pub fn day() -> impl Day {
    let mut solution = AocDay::new(YearDay::Day13, |x| x);
    solution.part_1(part_one);
    solution.part_2(part_two);
    solution.add_file("input.txt");
    solution
}

fn part_one(input: &String) -> String {
    let pairs = parse_input1(input);

    let mut sum = 0;
    let mut index = 0;

    for (left, right) in pairs {
        index += 1;

        match left.partial_cmp(&right) {
            Some(Ordering::Less) => {
                sum += index;
            }
            Some(Ordering::Greater) => continue,
            _ => (),
        }
    }

    sum.to_string()
}

fn part_two(input: &String) -> String {
    let mut values = parse_input2(input);

    let value2 = Value::List(vec![Value::List(vec![Value::Integer(2)])]);
    let value6 = Value::List(vec![Value::List(vec![Value::Integer(6)])]);

    values.push(value2.clone());
    values.push(value6.clone());

    values.sort();

    let mut product = 1;
    let mut index = 0;

    for value in values {
        index += 1;

        if value == value2 || value == value6 {
            product *= index;
        }
    }

    product.to_string()
}

fn parse_input1(input: &String) -> Vec<(Value, Value)> {
    let mut pairs = Vec::new();

    for pair in input.split("\n\n") {
        let mut pair = pair.split_terminator("\n");

        let left = pair.next().unwrap();
        let right = pair.next().unwrap();

        let (left, _) = parse_value(left);
        let (right, _) = parse_value(right);

        pairs.push((left, right));
    }

    pairs
}

fn parse_input2(input: &String) -> Vec<Value> {
    let mut values = Vec::new();

    for line in input.split("\n") {
        if line == "" {
            continue;
        }

        let (value, _) = parse_value(line);

        values.push(value);
    }

    values
}

fn parse_value(input: &str) -> (Value, &str) {
    if input.starts_with('[') {
        let mut values = Vec::new();
        let mut input = &input[1..input.len()];

        loop {
            if input.starts_with(']') {
                break;
            } else if input.starts_with(',') {
                let result = parse_value(&input[1..input.len()]);
                values.push(result.0);
                input = result.1;
            } else {
                let result = parse_value(input);
                values.push(result.0);
                input = result.1;
            }
        }

        (Value::List(values), &input[1..input.len()])
    } else {
        let end = input.find(&[',', ']']).unwrap();
        let integer = input[0..end].parse().unwrap();

        (Value::Integer(integer), &input[end..input.len()])
    }
}

#[derive(Debug, Eq, PartialEq, Ord, Clone)]
enum Value {
    List(Vec<Value>),
    Integer(u32),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Integer(left), Value::Integer(right)) => {
                if left < right {
                    Some(Ordering::Less)
                } else if left > right {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Equal)
                }
                // left.partial_cmp(right),
            }
            (Value::List(left), Value::List(right)) => {
                let mut left = left.into_iter();
                let mut right = right.into_iter();

                loop {
                    match (left.next(), right.next()) {
                        (Some(left), Some(right)) => match left.partial_cmp(right) {
                            Some(Ordering::Equal) => (),
                            value => return value,
                        },
                        (None, Some(_)) => return Some(Ordering::Less),
                        (Some(_), None) => return Some(Ordering::Greater),
                        (None, None) => return Some(Ordering::Equal),
                    }
                }
            }
            (Value::List(_), Value::Integer(right)) => {
                Value::partial_cmp(self, &Value::List(vec![Value::Integer(*right)]))
                // self.partial_cmp(&Value::List(vec![Value::Integer(*right)]))
            }
            (Value::Integer(left), Value::List(_)) => {
                Value::partial_cmp(&Value::List(vec![Value::Integer(*left)]), other)
                // Value::List(vec![Value::Integer(*left)]).partial_cmp(other)
            }
        }
    }
}
