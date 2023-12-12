use aoc_core::{AocResult, Day};

pub fn day() -> Day {
    let mut solution = Day::new(12);
    solution.part_1(parse, part_one);
    solution.part_2(parse, part_two);
    solution.add_file("files/test.in");
    solution.add_file("files/input.in");
    solution
}

fn parse(input: String) -> Vec<Record> {
    input
        .split_terminator('\n')
        .map(|line| {
            let (operationals, numbers) = line.split_once(' ').unwrap();

            let operationals = operationals.chars().map(State::from).collect();
            let numbers = numbers.split(',').map(|s| s.parse().unwrap()).collect();

            Record::new(operationals, numbers)
        })
        .collect()
}

fn part_one(records: Vec<Record>) -> AocResult<u64> {
    let mut sum = 0;

    for mut record in records {
        let mut cache = vec![None; record.operationals.len() * record.numbers.len()];

        let num = try_number(&mut record, 0, 0, &mut cache);
        sum += num;
    }

    Ok(sum)
}

fn part_two(records: Vec<Record>) -> AocResult<u64> {
    let mut sum = 0;

    for mut record in records {
        let operationals = record.operationals.clone();
        let numbers = record.numbers.clone();

        for _ in 0..4 {
            record.operationals.push(State::Unknown);
            record.operationals.extend(&operationals);

            record.numbers.extend(&numbers);
        }

        let mut cache = vec![None; record.operationals.len() * record.numbers.len()];

        sum += try_number(&mut record, 0, 0, &mut cache);
    }

    Ok(sum)
}

fn try_number(
    record: &mut Record,
    index: usize,
    num_index: usize,
    cache: &mut Vec<Option<u64>>,
) -> u64 {
    let operationals_len = record.operationals.len();

    // Return 0 if the number wont fit
    if index + record.numbers[num_index] - 1 >= operationals_len {
        return 0;
    }

    if let Some(cached) = cache[index + num_index * operationals_len] {
        return cached;
    }

    // Return 0 if in bounds and there is another broken part before
    if index > 0 && record.operationals[index - 1] == State::Damaged {
        return if record.operationals[index] == State::Damaged {
            0
        } else {
            try_number(record, index + 1, num_index, cache)
        };
    }

    // Return 0 if in bounds and there is another broken part after
    if (index + record.numbers[num_index]) < operationals_len
        && record.operationals[index + record.numbers[num_index]] == State::Damaged
    {
        return if record.operationals[index] == State::Damaged {
            0
        } else {
            try_number(record, index + 1, num_index, cache)
        };
    }

    // Return 0 if there is a working part
    for i in 0..record.numbers[num_index] {
        if record.operationals[index + i] == State::Operational {
            return if record.operationals[index] == State::Damaged {
                0
            } else {
                try_number(record, index + 1, num_index, cache)
            };
        }
    }

    if num_index + 1 == record.numbers.len() {
        // Return 0 if there are any remaining broken parts
        for i in (index + record.numbers[num_index])..operationals_len {
            if record.operationals[i] == State::Damaged {
                return if record.operationals[index] == State::Damaged {
                    0
                } else {
                    try_number(record, index + 1, num_index, cache)
                };
            }
        }

        if record.operationals[index] == State::Damaged {
            cache[index + num_index * operationals_len] = Some(1);
            1
        } else {
            let num = try_number(record, index + 1, num_index, cache);
            cache[index + num_index * operationals_len] = Some(num + 1);

            num + 1
        }
    } else {
        let mut sum = try_number(
            record,
            index + record.numbers[num_index] + 1,
            num_index + 1,
            cache,
        );

        if record.operationals[index] != State::Damaged {
            sum += try_number(record, index + 1, num_index, cache);
        }

        cache[index + num_index * operationals_len] = Some(sum);

        sum
    }
}

struct Record {
    operationals: Vec<State>,
    numbers: Vec<usize>,
}

impl Record {
    fn new(operationals: Vec<State>, numbers: Vec<usize>) -> Self {
        Self {
            operationals,
            numbers,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for State {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            _ => Self::Unknown,
        }
    }
}
