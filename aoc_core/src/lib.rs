mod client;
mod create;
mod download;
mod execute;
mod id;
mod upload;

use std::error::Error;

pub use client::*;
pub use create::*;
pub use download::*;
pub use execute::*;
pub use id::*;
pub use upload::*;

pub type AocResult<T> = Result<T, Box<dyn Error>>;
