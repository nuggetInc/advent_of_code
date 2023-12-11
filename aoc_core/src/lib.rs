mod client;
mod create;
mod download;
mod execute;
mod ids;

use std::error::Error;

pub use client::*;
pub use create::*;
pub use download::*;
pub use execute::*;
pub use ids::*;

pub type AocResult<T> = Result<T, Box<dyn Error>>;
