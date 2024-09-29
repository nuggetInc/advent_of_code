use std::cmp::Ordering;

use aoc_core::{AocResult, Day};

pub fn day() -> Day {
    let mut solution = Day::new(13);
    solution.part_1(|s: String| part_one(parse_one(s)));
    solution.part_2(|s: String| part_two(parse_two(s)));
    solution.add_file("files/input.in");
    solution
}

fn part_one(pairs: Vec<(Value, Value)>) -> AocResult<i32> {
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

    Ok(sum)
}

fn part_two(mut values: Vec<Value>) -> AocResult<i32> {
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

    Ok(product)
}

fn parse_one(input: String) -> Vec<(Value, Value)> {
    let mut pairs = Vec::new();

    for pair in input.split("\n\n") {
        let mut pair = pair.split_terminator('\n');

        let left = pair.next().unwrap();
        let right = pair.next().unwrap();

        let (left, _) = parse_value(left);
        let (right, _) = parse_value(right);

        pairs.push((left, right));
    }

    pairs
}

fn parse_two(input: String) -> Vec<Value> {
    let mut values = Vec::new();

    for line in input.split_terminator('\n') {
        if line.is_empty() {
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
        let end = input.find([',', ']']).unwrap();
        let integer = input[0..end].parse().unwrap();

        (Value::Integer(integer), &input[end..input.len()])
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Value {
    List(Vec<Value>),
    Integer(u32),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Integer(left), Value::Integer(right)) => left.cmp(right),
            (Value::List(left), Value::List(right)) => {
                let mut left = left.iter();
                let mut right = right.iter();

                loop {
                    match (left.next(), right.next()) {
                        (Some(left), Some(right)) => match left.cmp(right) {
                            Ordering::Equal => (),
                            value => return value,
                        },
                        (None, Some(_)) => return Ordering::Less,
                        (Some(_), None) => return Ordering::Greater,
                        (None, None) => return Ordering::Equal,
                    }
                }
            }
            (Value::List(_), Value::Integer(right)) => {
                Value::cmp(self, &Value::List(vec![Value::Integer(*right)]))
                // self.partial_cmp(&Value::List(vec![Value::Integer(*right)]))
            }
            (Value::Integer(left), Value::List(_)) => {
                Value::cmp(&Value::List(vec![Value::Integer(*left)]), other)
                // Value::List(vec![Value::Integer(*left)]).partial_cmp(other)
            }
        }
    }
}
