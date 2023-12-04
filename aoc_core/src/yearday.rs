use core::fmt;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum YearDay {
    Day01,
    Day02,
    Day03,
    Day04,
    Day05,
    Day06,
    Day07,
    Day08,
    Day09,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
}

impl fmt::Display for YearDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::Day01 => "Day 01",
            Self::Day02 => "Day 02",
            Self::Day03 => "Day 03",
            Self::Day04 => "Day 04",
            Self::Day05 => "Day 05",
            Self::Day06 => "Day 06",
            Self::Day07 => "Day 07",
            Self::Day08 => "Day 08",
            Self::Day09 => "Day 09",
            Self::Day10 => "Day 10",
            Self::Day11 => "Day 11",
            Self::Day12 => "Day 12",
            Self::Day13 => "Day 13",
            Self::Day14 => "Day 14",
            Self::Day15 => "Day 15",
            Self::Day16 => "Day 16",
            Self::Day17 => "Day 17",
            Self::Day18 => "Day 18",
            Self::Day19 => "Day 19",
            Self::Day20 => "Day 20",
            Self::Day21 => "Day 21",
            Self::Day22 => "Day 22",
            Self::Day23 => "Day 23",
            Self::Day24 => "Day 24",
            Self::Day25 => "Day 25",
        };

        write!(f, "{}", text)
    }
}
