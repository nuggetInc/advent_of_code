use aoc_core::{AocResult, Day};

pub fn day() -> Day {
    let mut solution = Day::new(10);
    solution.part_1(part_one);
    solution.part_2(part_two);
    solution.add_file("files/input.in");
    solution
}

fn part_one(input: String) -> AocResult<i32> {
    let mut sum = 0;
    let mut x = 1;
    let mut cycle = 1;

    for line in input.split_terminator('\n') {
        let mut split = line.split(' ');

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

    Ok(sum)
}

fn part_two(input: String) -> AocResult<String> {
    let mut crt = String::new();
    let mut x = 1;
    let mut cycle = 0;

    for line in input.split_terminator('\n') {
        let mut split = line.split(' ');

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

    Ok(crt)
}
