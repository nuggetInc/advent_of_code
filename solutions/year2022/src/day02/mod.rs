use aoc_core::{Day, YearDay};

pub fn day() -> Day {
    let mut solution = Day::new(YearDay::Day02);
    solution.part_1(|x| x, part_one);
    solution.part_2(|x| x, part_two);
    solution.add_file("input.txt");
    solution
}

fn part_one(input: String) -> String {
    let mut score: u32 = 0;

    for line in input.split("\n") {
        score += match line {
            "A X" => 1 + 3,
            "A Y" => 2 + 6,
            "A Z" => 3 + 0,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 1 + 6,
            "C Y" => 2 + 0,
            "C Z" => 3 + 3,
            _ => unreachable!("impossible combination"),
        };
    }

    score.to_string()
}

fn part_two(input: String) -> String {
    let mut score: u32 = 0;

    for line in input.split("\n") {
        score += match line {
            "A X" => 3 + 0,
            "A Y" => 1 + 3,
            "A Z" => 2 + 6,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 2 + 0,
            "C Y" => 3 + 3,
            "C Z" => 1 + 6,
            _ => unreachable!("impossible combination"),
        };
    }

    score.to_string()
}
