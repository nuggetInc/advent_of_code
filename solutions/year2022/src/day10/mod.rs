use aoc_core::Day;

pub fn day() -> Day {
    let mut solution = Day::new(10);
    solution.part_1(|x| x, part_one);
    solution.part_2(|x| x, part_two);
    solution.add_file("files/input.in");
    solution
}

fn part_one(input: String) -> String {
    let mut sum = 0;
    let mut x = 1;
    let mut cycle = 1;

    for line in input.split('\n') {
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

    sum.to_string()
}

fn part_two(input: String) -> String {
    let mut crt = String::new();
    let mut x = 1;
    let mut cycle = 0;

    for line in input.split('\n') {
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

    crt.to_string()
}
