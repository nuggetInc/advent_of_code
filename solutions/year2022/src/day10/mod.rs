use aoc_core::{AocDay, Day, YearDay};

pub fn day() -> impl Day {
    let mut solution = AocDay::new(YearDay::Day10, |x| x);
    solution.part_1(part_one);
    solution.part_2(part_two);
    solution.add_file("input.txt");
    solution
}

fn part_one(input: &String) -> String {
    let mut sum = 0;
    let mut x = 1;
    let mut cycle = 1;

    for line in input.split("\n") {
        let mut split = line.split(" ");

        if let Some("addx") = split.next() {
            let value: i32 = split.next().unwrap().parse().unwrap();
            if cycle % 40 - 20 == 0 {
                sum += x * cycle;
            }

            cycle += 1;

            if cycle % 40 - 20 == 0 {
                sum += x * cycle;
            }

            x += value;
        } else if cycle % 40 - 20 == 0 {
            sum += x * cycle;
        }

        cycle += 1;
    }

    sum.to_string()
}

fn part_two(input: &String) -> String {
    let mut crt = String::new();
    let mut x = 1;
    let mut cycle = 0;

    for line in input.split("\n") {
        let mut split = line.split(" ");

        if let Some("addx") = split.next() {
            let value: i32 = split.next().unwrap().parse().unwrap();

            if cycle % 40 == 0 {
                crt.push('\n');
            }

            if cycle % 40 == x || cycle % 40 == x - 1 || cycle % 40 == x + 1 {
                crt.push('#');
            } else {
                crt.push('.');
            }

            cycle += 1;

            if cycle % 40 == 0 {
                crt.push('\n');
            }

            if cycle % 40 == x || cycle % 40 == x - 1 || cycle % 40 == x + 1 {
                crt.push('#');
            } else {
                crt.push('.');
            }

            x += value;
        } else {
            if cycle % 40 == 0 {
                crt.push('\n');
            }

            if cycle % 40 == x || cycle % 40 == x - 1 || cycle % 40 == x + 1 {
                crt.push('#');
            } else {
                crt.push('.');
            }
        }

        cycle += 1;
    }

    crt.to_string()
}