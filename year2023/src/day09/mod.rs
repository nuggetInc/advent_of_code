use aoc_core::{AocResult, Day};
use itertools::Itertools;

pub fn day() -> Day {
    let mut solution = Day::new(9);
    solution.part_1(|s: String| part_one(parse(s)));
    solution.part_2(|s: String| part_two(parse(s)));
    solution.add_file("files/test.in");
    solution.add_file("files/input.in");
    solution
}

fn parse(input: String) -> Vec<Sequence> {
    input
        .split_terminator('\n')
        .map(|line| {
            line.split(' ')
                .map(|number| number.parse().unwrap())
                .collect_vec()
                .into()
        })
        .collect()
}

fn part_one(histories: Vec<Sequence>) -> AocResult<i32> {
    Ok(histories
        .into_iter()
        .map(|history| history.next())
        .sum::<i32>())
}

fn part_two(histories: Vec<Sequence>) -> AocResult<i32> {
    Ok(histories
        .into_iter()
        .map(|history| history.previous())
        .sum::<i32>())
}

struct Sequence(Vec<i32>);

impl Sequence {
    fn next(&self) -> i32 {
        let sub_sequence = self.sub_sequence();

        if sub_sequence.0.iter().all(|num| *num == 0) {
            return *self.0.last().unwrap();
        }

        self.0.last().unwrap() + sub_sequence.next()
    }

    fn previous(&self) -> i32 {
        let sub_sequence = self.sub_sequence();

        if sub_sequence.0.iter().all(|num| *num == 0) {
            return *self.0.first().unwrap();
        }

        self.0.first().unwrap() - sub_sequence.previous()
    }

    fn sub_sequence(&self) -> Self {
        let mut sequence = Vec::with_capacity(self.0.len() - 1);

        for (previous, next) in self.0.iter().tuple_windows::<(_, _)>() {
            sequence.push(*next - *previous);
        }

        sequence.into()
    }
}

impl From<Vec<i32>> for Sequence {
    fn from(value: Vec<i32>) -> Self {
        Self(value)
    }
}
