use aoc_core::{AocResult, Day};

pub fn day() -> Day {
    let mut solution = Day::new(2);
    solution.part_1(|x| x, part_one);
    solution.part_2(|x| x, part_two);
    solution.add_file("files/input.in");
    solution
}

fn part_one(input: String) -> AocResult<u32> {
    let mut score: u32 = 0;

    for line in input.split_terminator('\n') {
        score += match line {
            "A X" => 1 + 3,
            "A Y" => 2 + 6,
            "A Z" => 3,
            "B X" => 1,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 1 + 6,
            "C Y" => 2,
            "C Z" => 3 + 3,
            _ => unreachable!("impossible combination"),
        };
    }

    Ok(score)
}

fn part_two(input: String) -> AocResult<u32> {
    let mut score: u32 = 0;

    for line in input.split_terminator('\n') {
        score += match line {
            "A X" => 3,
            "A Y" => 1 + 3,
            "A Z" => 2 + 6,
            "B X" => 1,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 2,
            "C Y" => 3 + 3,
            "C Z" => 1 + 6,
            _ => unreachable!("impossible combination"),
        };
    }

    Ok(score)
}
