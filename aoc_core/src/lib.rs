mod client;
mod create;
mod download;
mod execute;
mod ids;
mod upload;

use core::fmt;
use std::error::Error;

pub use client::*;
pub use create::*;
pub use download::*;
pub use execute::*;
pub use ids::*;
pub use upload::*;

pub type AocResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub enum AocError {
    PanicedPart,

    UnimplementedYear(YearId),
    UnimplementedDay(DayId),
    UnimplementedPart(PartId),

    InvalidCommand(String),
    InvalidYear(String),
    InvalidDay(String),
    InvalidPart(String),
}

impl fmt::Display for AocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PanicedPart => write!(f, "The part paniced during execution"),

            Self::UnimplementedPart(id) => write!(f, "{} is not implemented", id.name()),
            Self::UnimplementedDay(id) => write!(f, "{} is not implemented", id.name()),
            Self::UnimplementedYear(id) => write!(f, "{} is not implemented", id.name()),

            Self::InvalidCommand(s) => write!(f, "'{}' is not a valid command", s),
            Self::InvalidYear(s) => write!(f, "'{}' is not a valid year", s),
            Self::InvalidDay(s) => write!(f, "'{}' is not a valid day", s),
            Self::InvalidPart(s) => write!(f, "'{}' is not a valid part", s),
        }
    }
}

impl Error for AocError {}
