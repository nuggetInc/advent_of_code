mod client;
mod create;
mod download;
mod execute;
mod ids;

use core::fmt;
use std::error::Error;

pub use client::*;
pub use create::*;
pub use download::*;
pub use execute::*;
pub use ids::*;

pub type AocResult<T> = Result<T, Box<dyn Error + Send>>;

#[derive(Debug)]
pub enum AocError {
    Paniced,
}

impl fmt::Display for AocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Paniced => write!(f, "The part paniced during execution"),
        }
    }
}

impl Error for AocError {}
