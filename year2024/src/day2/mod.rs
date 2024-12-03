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

fn parse(input: &String) -> Vec<Vec<i32>> {
    input
        .split_terminator('\n')
        .map(|line| {
            line.split(' ')
                .map(|cell| cell.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

fn part_one(input: &String) -> AocResult<u32> {
    let levels = parse(input);

    let mut count = 0;

    for level in levels {
        let level_diff = level
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect_vec();

        if level_diff.iter().all(|diff| *diff >= 1 && *diff <= 3)
            || level_diff.iter().all(|diff| *diff >= -3 && *diff <= -1)
        {
            count += 1;
        }
    }

    Ok(count)
}

fn part_two(input: &String) -> AocResult<u32> {
    let levels = parse(input);

    let mut count = 0;

    'levels: for mut level in levels {
        let level_diff = level
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect_vec();

        if level_diff.iter().all(|diff| *diff >= 1 && *diff <= 3)
            || level_diff.iter().all(|diff| *diff >= -3 && *diff <= -1)
        {
            count += 1;
            continue 'levels;
        }

        for level_variation in 0..level.len() {
            let removed = level.remove(level_variation);

            let level_diff = level
                .iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect_vec();

            if level_diff.iter().all(|diff| *diff >= 1 && *diff <= 3)
                || level_diff.iter().all(|diff| *diff >= -3 && *diff <= -1)
            {
                count += 1;
                continue 'levels;
            }

            level.insert(level_variation, removed);
        }
    }

    Ok(count)
}
