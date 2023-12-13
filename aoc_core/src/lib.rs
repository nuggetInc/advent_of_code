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
    Paniced,
    PartUnimplemented(PartId),
}

impl fmt::Display for AocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Paniced => write!(f, "The part paniced during execution"),
            Self::PartUnimplemented(id) => write!(f, "{} is not implemented", id.name()),
        }
    }
}

impl Error for AocError {}
