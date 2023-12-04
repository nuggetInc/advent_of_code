use aoc_core::{AocDay, Day, YearDay};

pub fn day() -> impl Day {
    let mut solution = AocDay::new(YearDay::Day04, parse);
    solution.part_1(part_one);
    solution.part_2(part_2);
    solution.add_file("test.txt");
    solution.add_file("input.txt");
    solution
}

fn parse(input: String) -> Vec<Card> {
    input
        .split_terminator('\n')
        .map(|line| {
            let card_split = line.split(':');

            let mut numbers_split = card_split.skip(1).next().unwrap().split('|');

            let winning = numbers_split
                .next()
                .unwrap()
                .split(' ')
                .filter_map(|s| s.parse().ok())
                .collect();

            let ours = numbers_split
                .next()
                .unwrap()
                .split(' ')
                .filter_map(|s| s.parse().ok())
                .collect();

            Card::new(winning, ours)
        })
        .collect()
}

fn part_one(cards: &Vec<Card>) -> String {
    let mut total = 0;

    for card in cards {
        let count = card.winning();

        if count > 0 {
            total += 2_i32.pow(count - 1)
        }
    }

    total.to_string()
}

fn part_2(cards: &Vec<Card>) -> String {
    let mut counts = vec![1; cards.len()];
    let mut sum = 0;

    for index in 0..cards.len() {
        sum += counts[index];
        let winning = cards[index].winning();

        if winning == 0 {
            continue;
        }

        for offset in 1..=winning {
            counts[index + offset as usize] += counts[index]
        }
    }

    sum.to_string()
}

#[derive(Debug)]
struct Card {
    winning: Vec<u32>,
    ours: Vec<u32>,
}

impl Card {
    fn new(winning: Vec<u32>, ours: Vec<u32>) -> Self {
        Self { winning, ours }
    }

    fn winning(&self) -> u32 {
        let mut count = 0;

        for number in &self.winning {
            if self.ours.contains(number) {
                count += 1;
            }
        }

        count
    }
}
