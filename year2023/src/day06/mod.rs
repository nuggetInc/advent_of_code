use aoc_core::{AocResult, Day};

pub fn day() -> Day {
    let mut solution = Day::new(6);
    solution.part_1(parse, part_one);
    solution.part_2(parse, part_two);
    solution.add_file("files/test.in");
    solution.add_file("files/input.in");
    solution
}

fn parse(input: String) -> Vec<(u64, u64)> {
    let mut lines = input.split_terminator('\n');
    let time = lines.next().unwrap();
    let distance = lines.next().unwrap();

    time.split_whitespace()
        .zip(distance.split_whitespace())
        .skip(1)
        .map(|(time, dist)| (time.parse().unwrap(), dist.parse().unwrap()))
        .collect()
}

fn part_one(values: Vec<(u64, u64)>) -> AocResult<u64> {
    Ok(values
        .into_iter()
        .map(|(time, distance)| {
            let t = time as f64;
            let delta = t * t - 4. * distance as f64;
            let d = delta.sqrt();
            let (m0, m1) = ((t - d) / 2., (t + d) / 2.);
            let (i0, i1) = (m0.ceil(), m1.floor());
            let min = i0 as u64;
            let max = i1 as u64;
            max - min + 1
        })
        .product::<u64>())
}

fn part_two(values: Vec<(u64, u64)>) -> AocResult<u64> {
    let (time, distance) = values.into_iter().fold((0_u64, 0_u64), |acc, num| {
        (
            (acc.0 * 10_u64.pow(num.0.ilog10() + 1)) + num.0,
            (acc.1 * 10_u64.pow(num.1.ilog10() + 1)) + num.1,
        )
    });

    let t = time as f64;
    let delta = t * t - 4. * distance as f64;
    let d = delta.sqrt();
    let (m0, m1) = ((t - d) / 2., (t + d) / 2.);
    let (i0, i1) = (m0.ceil(), m1.floor());
    let min = i0 as u64;
    let max = i1 as u64;
    Ok(max - min + 1)
}
