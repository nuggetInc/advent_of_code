use aoc_core::{AocDay, Day};

pub fn day04() -> impl Day {
    let mut solution = AocDay::new(parse);
    solution.add_part("Part 1".into(), part_1);
    solution.add_part("Part 2".into(), part_2);
    solution.add_file("src/year2023/day04/test.txt");
    solution.add_file("src/year2023/day04/input.txt");
    solution
}

fn parse(input: String) -> Vec<Card> {
    let mut cards = Vec::new();

    for line in input.split_terminator('\n') {
        let mut card_split = line.split(':');
        card_split.next();

        let mut numbers_split = card_split.next().unwrap().split('|');

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

        cards.push(Card::new(winning, ours));
    }

    cards
}

fn part_1(cards: &Vec<Card>) -> String {
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

    for index in 0..cards.len() {
        let winning = cards[index].winning();

        if winning == 0 {
            continue;
        }

        for offset in 1..=winning {
            counts[index + offset as usize] += counts[index]
        }
    }

    counts.into_iter().sum::<u32>().to_string()
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
