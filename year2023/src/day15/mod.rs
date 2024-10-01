use aoc_core::{AocResult, Day};

pub fn day() -> Day {
    let mut solution = Day::new(15);
    solution.part_1(part_one);
    solution.part_2(part_two);
    solution.add_file("files/test.in");
    solution.add_file("files/input.in");
    solution
}

fn parse_one(input: &String) -> Vec<String> {
    input.trim().split(',').map(|s| s.to_owned()).collect()
}

fn part_one(input: &String) -> AocResult<u32> {
    let values = parse_one(input);

    let mut sum = 0;
    for value in values {
        sum += hash(&value);
    }

    Ok(sum)
}

fn parse_two(input: &String) -> Vec<Operation> {
    input
        .trim()
        .split(',')
        .map(|value| {
            if value.ends_with('-') {
                Operation::Remove(value.trim_end_matches('-').to_owned())
            } else {
                let (label, length) = value.split_once('=').unwrap();

                Operation::Add(label.to_owned(), length.parse().unwrap())
            }
        })
        .collect()
}

fn part_two(input: &String) -> AocResult<u32> {
    let values = parse_two(input);

    let mut boxes = vec![Vec::<(String, u32)>::new(); 256];

    for value in values {
        match value {
            Operation::Remove(label) => {
                let hash = hash(&label) as usize;

                if let Some(position) = boxes[hash].iter().position(|lens| lens.0 == label) {
                    boxes[hash].remove(position);
                }
            }
            Operation::Add(label, length) => {
                let hash = hash(&label) as usize;

                if let Some(position) = boxes[hash].iter().position(|lens| lens.0 == label) {
                    boxes[hash][position].1 = length;
                } else {
                    boxes[hash].push((label.to_owned(), length));
                }
            }
        }
    }

    let mut sum = 0;

    for (box_index, r#box) in boxes.into_iter().enumerate() {
        for (lens_index, (_, length)) in r#box.into_iter().enumerate() {
            sum += (box_index as u32 + 1) * (lens_index as u32 + 1) * length;
        }
    }

    Ok(sum)
}

fn hash(value: &String) -> u32 {
    let mut hash = 0;
    for char in value.chars() {
        hash += char as u32;
        hash *= 17;
        hash %= 256;
    }
    hash
}

enum Operation {
    Remove(String),
    Add(String, u32),
}
