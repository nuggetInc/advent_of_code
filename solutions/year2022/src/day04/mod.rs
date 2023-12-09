use aoc_core::Day;

pub fn day() -> Day {
    let mut solution = Day::new(4);
    solution.part_1(|x| x, part_one);
    solution.part_2(|x| x, part_two);
    solution.add_file("input.txt");
    solution
}

fn part_one(input: String) -> String {
    let mut total = 0;

    for line in input.split("\n") {
        let (first, second) = line.split_once(",").unwrap();
        let (first_start, first_end) = first.split_once("-").unwrap();
        let (second_start, second_end) = second.split_once("-").unwrap();

        let first_start: u32 = first_start.parse().unwrap();
        let first_end: u32 = first_end.parse().unwrap();
        let second_start: u32 = second_start.parse().unwrap();
        let second_end: u32 = second_end.parse().unwrap();

        let first = first_start..=first_end;
        let second = second_start..=second_end;

        if (first.contains(&second_start) && first.contains(&second_end))
            || (second.contains(&first_start) && second.contains(&first_end))
        {
            total += 1;
        } else {
        }
    }

    total.to_string()
}

fn part_two(input: String) -> String {
    let mut total = 0;

    for line in input.split("\n") {
        let (first, second) = line.split_once(",").unwrap();
        let (first_start, first_end) = first.split_once("-").unwrap();
        let (second_start, second_end) = second.split_once("-").unwrap();

        let first_start: u32 = first_start.parse().unwrap();
        let first_end: u32 = first_end.parse().unwrap();
        let second_start: u32 = second_start.parse().unwrap();
        let second_end: u32 = second_end.parse().unwrap();

        let first = first_start..=first_end;
        let second = second_start..=second_end;

        if first.contains(&second_start)
            || first.contains(&second_end)
            || second.contains(&first_start)
            || second.contains(&first_end)
        {
            total += 1;
        } else {
        }
    }

    total.to_string()
}
