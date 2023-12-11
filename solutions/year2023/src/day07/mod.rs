use std::fmt::Display;

use aoc_core::{AocResult, Day};
use itertools::Itertools;

pub fn day() -> Day {
    let mut solution = Day::new(7);
    solution.part_1(parse, part_one);
    solution.part_2(parse, part_two);
    solution.add_file("files/test.in");
    solution.add_file("files/input.in");
    solution
}

fn parse(input: String) -> Vec<Play> {
    input
        .split_terminator('\n')
        .map(|line| {
            let mut split = line.split(' ');

            let test = split.next().unwrap();
            let hand = test
                .chars()
                .map(|card| match card {
                    'A' => Card::Ace,
                    'K' => Card::King,
                    'Q' => Card::Queen,
                    'J' => Card::Jack,
                    'T' => Card::Ten,
                    '9' => Card::Nine,
                    '8' => Card::Eight,
                    '7' => Card::Seven,
                    '6' => Card::Six,
                    '5' => Card::Five,
                    '4' => Card::Four,
                    '3' => Card::Three,
                    '2' => Card::Two,
                    _ => unreachable!(),
                })
                .collect();
            let bid = split.next().unwrap().parse().unwrap();

            Play::new(hand, bid)
        })
        .collect()
}

fn part_one(plays: Vec<Play>) -> AocResult<u32> {
    Ok(plays
        .into_iter()
        .map(|play| {
            let hand = play.hand();
            (play, hand)
        })
        .sorted_by(|play, other| {
            play.1
                .cmp(&other.1)
                .then_with(|| play.0.cards.cmp(&other.0.cards))
        })
        .rev()
        .enumerate()
        .map(|(index, play)| (index as u32 + 1) * play.0.bid)
        .sum::<u32>())
}

fn part_two(mut plays: Vec<Play>) -> AocResult<u32> {
    for play in &mut plays {
        for card in &mut play.cards {
            if *card == Card::Jack {
                *card = Card::Joker
            }
        }
    }

    Ok(plays
        .into_iter()
        .map(|play| {
            let hand = play.hand();
            (play, hand)
        })
        .sorted_by(|play, other| {
            play.1
                .cmp(&other.1)
                .then_with(|| play.0.cards.cmp(&other.0.cards))
        })
        .rev()
        .enumerate()
        .map(|(index, play)| (index as u32 + 1) * play.0.bid)
        .sum::<u32>())
}

#[derive(Clone)]
struct Play {
    cards: Vec<Card>,
    bid: u32,
}

impl Play {
    fn new(cards: Vec<Card>, bid: u32) -> Self {
        Self { cards, bid }
    }

    fn hand(&self) -> Hand {
        let mut counts = self.cards.iter().counts();
        let jokers = counts.remove(&Card::Joker).unwrap_or(0);
        let counts = counts.into_values().sorted().rev().collect_vec();

        if counts.first().unwrap_or(&0) + jokers >= 5 {
            Hand::Five
        } else if counts.first().unwrap_or(&0) + jokers >= 4 {
            Hand::Four
        } else if counts.first().unwrap_or(&0) + counts.get(1).unwrap_or(&0) + jokers >= 5 {
            Hand::Full
        } else if counts.first().unwrap_or(&0) + jokers >= 3 {
            Hand::Three
        } else if counts.first().unwrap_or(&0) + counts.get(1).unwrap_or(&0) + jokers >= 4 {
            Hand::Two
        } else if counts.first().unwrap_or(&0) + jokers >= 2 {
            Hand::One
        } else {
            Hand::High
        }
    }
}

impl Display for Play {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for card in &self.cards {
            write!(f, "{}", card)?;
        }

        write!(f, " ({})", self.hand())?;

        write!(f, " {}", self.bid)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    Five,
    Four,
    Full,
    Three,
    Two,
    One,
    High,
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Five => write!(f, "Five of a kind"),
            Self::Four => write!(f, "Four of a kind"),
            Self::Full => write!(f, "Full house"),
            Self::Three => write!(f, "Three of a kind"),
            Self::Two => write!(f, "Two pair"),
            Self::One => write!(f, "One pair"),
            Self::High => write!(f, "High card"),
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ace => write!(f, "A"),
            Self::King => write!(f, "K"),
            Self::Queen => write!(f, "Q"),
            Self::Jack => write!(f, "J"),
            Self::Ten => write!(f, "T"),
            Self::Nine => write!(f, "9"),
            Self::Eight => write!(f, "8"),
            Self::Seven => write!(f, "7"),
            Self::Six => write!(f, "6"),
            Self::Five => write!(f, "5"),
            Self::Four => write!(f, "4"),
            Self::Three => write!(f, "3"),
            Self::Two => write!(f, "2"),
            Self::Joker => write!(f, "J"),
        }
    }
}
