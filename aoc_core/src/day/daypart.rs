use core::fmt;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DayPart {
    Part1,
    Part2,
}

impl fmt::Display for DayPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::Part1 => "Part 1",
            Self::Part2 => "Part 2",
        };

        write!(f, "{}", text)
    }
}

impl TryFrom<&String> for DayPart {
    type Error = ();

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Part1" | "1" => Ok(Self::Part1),
            "Part2" | "2" => Ok(Self::Part2),

            _ => Err(()),
        }
    }
}
