use aoc_core::{AocResult, Day};

pub fn day() -> Day {
    let mut solution = Day::new();
    solution.part_1(part_one);
    solution.part_2(part_two);
    solution.add_file("files/input.in");
    solution
}

fn part_one(input: &String) -> AocResult<u32> {
    let mut highest = 0;
    for inventory in input.split("\n\n") {
        let calories: u32 = inventory
            .split_terminator('\n')
            .map(|a| a.parse::<u32>().unwrap())
            .sum();

        if calories > highest {
            highest = calories;
        }
    }

    Ok(highest)
}

fn part_two(input: &String) -> AocResult<u32> {
    let mut calories: Vec<u32> = input
        .split("\n\n")
        .map(|inventory| {
            inventory
                .split_terminator('\n')
                .map(|a| a.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();
    calories.sort();

    Ok(calories.into_iter().rev().take(3).sum::<u32>())
}
