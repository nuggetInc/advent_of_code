use aoc_core::{AocResult, Day};

pub fn day() -> Day {
    let mut solution = Day::new();
    solution.part_1(part_one);
    solution.part_2(part_two);
    solution.add_file("files/test.in");
    solution.add_file("files/input.in");
    solution
}

fn parse(input: &String) -> (Vec<u32>, Vec<u32>) {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in input.split_terminator('\n') {
        let mut split = line.split_whitespace();
        vec1.push(split.next().unwrap().parse::<u32>().unwrap());
        vec2.push(split.next().unwrap().parse::<u32>().unwrap());
    }

    (vec1, vec2)
}

fn part_one(input: &String) -> AocResult<u32> {
    let (mut vec1, mut vec2) = parse(input);

    vec1.sort();
    vec2.sort();

    Ok(vec1.into_iter().zip(vec2).map(|(a, b)| a.abs_diff(b)).sum())
}

fn part_two(input: &String) -> AocResult<u32> {
    let (vec1, vec2) = parse(input);

    Ok(vec1
        .into_iter()
        .map(|a| a * vec2.iter().filter(|c| **c == a).count() as u32)
        .sum())
}
