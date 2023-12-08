use core::fmt;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

impl YearDay {
    pub fn folder_name(&self) -> &'static str {
        match self {
            Self::Day01 => "day01",
            Self::Day02 => "day02",
            Self::Day03 => "day03",
            Self::Day04 => "day04",
            Self::Day05 => "day05",
            Self::Day06 => "day06",
            Self::Day07 => "day07",
            Self::Day08 => "day08",
            Self::Day09 => "day09",
            Self::Day10 => "day10",
            Self::Day11 => "day11",
            Self::Day12 => "day12",
            Self::Day13 => "day13",
            Self::Day14 => "day14",
            Self::Day15 => "day15",
            Self::Day16 => "day16",
            Self::Day17 => "day17",
            Self::Day18 => "day18",
            Self::Day19 => "day19",
            Self::Day20 => "day20",
            Self::Day21 => "day21",
            Self::Day22 => "day22",
            Self::Day23 => "day23",
            Self::Day24 => "day24",
            Self::Day25 => "day25",
        }
    }
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

impl TryFrom<&String> for YearDay {
    type Error = ();

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Day01" | "D01" | "01" | "1" => Ok(Self::Day01),
            "Day02" | "D02" | "02" | "2" => Ok(Self::Day02),
            "Day03" | "D03" | "03" | "3" => Ok(Self::Day03),
            "Day04" | "D04" | "04" | "4" => Ok(Self::Day04),
            "Day05" | "D05" | "05" | "5" => Ok(Self::Day05),
            "Day06" | "D06" | "06" | "6" => Ok(Self::Day06),
            "Day07" | "D07" | "07" | "7" => Ok(Self::Day07),
            "Day08" | "D08" | "08" | "8" => Ok(Self::Day08),
            "Day09" | "D09" | "09" | "9" => Ok(Self::Day09),
            "Day10" | "D10" | "10" => Ok(Self::Day10),
            "Day11" | "D11" | "11" => Ok(Self::Day11),
            "Day12" | "D12" | "12" => Ok(Self::Day12),
            "Day13" | "D13" | "13" => Ok(Self::Day13),
            "Day14" | "D14" | "14" => Ok(Self::Day14),
            "Day15" | "D15" | "15" => Ok(Self::Day15),
            "Day16" | "D16" | "16" => Ok(Self::Day16),
            "Day17" | "D17" | "17" => Ok(Self::Day17),
            "Day18" | "D18" | "18" => Ok(Self::Day18),
            "Day19" | "D19" | "19" => Ok(Self::Day19),
            "Day20" | "D20" | "20" => Ok(Self::Day20),
            "Day21" | "D21" | "21" => Ok(Self::Day21),
            "Day22" | "D22" | "22" => Ok(Self::Day22),
            "Day23" | "D23" | "23" => Ok(Self::Day23),
            "Day24" | "D24" | "24" => Ok(Self::Day24),
            "Day25" | "D25" | "25" => Ok(Self::Day25),
            _ => Err(()),
        }
    }
}

impl From<YearDay> for u32 {
    fn from(value: YearDay) -> Self {
        match value {
            YearDay::Day01 => 1,
            YearDay::Day02 => 2,
            YearDay::Day03 => 3,
            YearDay::Day04 => 4,
            YearDay::Day05 => 5,
            YearDay::Day06 => 6,
            YearDay::Day07 => 7,
            YearDay::Day08 => 8,
            YearDay::Day09 => 9,
            YearDay::Day10 => 10,
            YearDay::Day11 => 11,
            YearDay::Day12 => 12,
            YearDay::Day13 => 13,
            YearDay::Day14 => 14,
            YearDay::Day15 => 15,
            YearDay::Day16 => 16,
            YearDay::Day17 => 17,
            YearDay::Day18 => 18,
            YearDay::Day19 => 19,
            YearDay::Day20 => 20,
            YearDay::Day21 => 21,
            YearDay::Day22 => 22,
            YearDay::Day23 => 23,
            YearDay::Day24 => 24,
            YearDay::Day25 => 25,
        }
    }
}
