use aoc_core::Day;

pub fn day() -> Day {
    let mut solution = Day::new(1);
    solution.part_1(parse, part_one);
    solution.part_2(parse, part_two);
    solution.add_file("test.txt");
    solution.add_file("input.txt");
    solution
}

fn parse(input: String) -> Vec<String> {
    input.split_terminator('\n').map(str::to_owned).collect()
}

fn part_one(lines: Vec<String>) -> String {
    let mut total = 0;

    for line in lines {
        let numbers: Vec<_> = line.chars().filter(|c| c.is_numeric()).collect();
        let mut number = String::new();
        number.push(*numbers.first().unwrap());
        number.push(*numbers.last().unwrap());

        total += number.parse::<i32>().unwrap();
    }

    total.to_string()
}

fn part_two(lines: Vec<String>) -> String {
    let mut total = 0;

    const REPLACE: [(&str, char); 10] = [
        ("zero", '0'),
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    for line in lines {
        'outer: for (index, char) in line.char_indices() {
            if char.is_ascii_digit() {
                total += char.to_digit(10).unwrap() * 10;
                break;
            } else {
                for (from, to) in REPLACE {
                    if line[index..].starts_with(from) {
                        total += to.to_digit(10).unwrap() * 10;
                        break 'outer;
                    }
                }
            }
        }

        'outer: for (index, char) in line.char_indices().rev() {
            if char.is_ascii_digit() {
                total += char.to_digit(10).unwrap();
                break;
            } else {
                for (from, to) in REPLACE {
                    if line[index..].starts_with(from) {
                        total += to.to_digit(10).unwrap();
                        break 'outer;
                    }
                }
            }
        }
    }

    total.to_string()
}
