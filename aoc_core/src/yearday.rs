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
