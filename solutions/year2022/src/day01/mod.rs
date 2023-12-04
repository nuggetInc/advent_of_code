use aoc_core::{AocDay, Day, YearDay};

pub fn day() -> impl Day {
    let mut solution = AocDay::new(YearDay::Day01, |x| x);
    solution.part_1(part_one);
    solution.part_2(part_two);
    solution.add_file("input.txt");
    solution
}

fn part_one(input: &String) -> String {
    let mut highest = 0;
    for inventory in input.split("\n\n") {
        let calories: u32 = inventory
            .split("\n")
            .map(|a| a.parse::<u32>().unwrap())
            .sum();

        if calories > highest {
            highest = calories;
        }
    }

    highest.to_string()
}

fn part_two(input: &String) -> String {
    let mut calories: Vec<u32> = input
        .split("\n\n")
        .map(|inventory| {
            inventory
                .split("\n")
                .map(|a| a.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();
    calories.sort();

    calories.into_iter().rev().take(3).sum::<u32>().to_string()
}
