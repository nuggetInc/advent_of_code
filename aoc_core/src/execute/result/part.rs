use std::{
    io::{self, Write},
    time::Duration,
};

use crossterm::{
    style::{Print, Stylize},
    QueueableCommand,
};

use crate::{AocPart, AocResult, Id};

pub struct PartResult {
    id: Id<AocPart>,
    answer: String,
    expected: Option<String>,
    elapsed: Duration,
}

impl PartResult {
    pub fn new(
        id: Id<AocPart>,
        answer: String,
        expected: Option<String>,
        elapsed: Duration,
    ) -> Self {
        Self {
            id,
            answer,
            expected,
            elapsed,
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }

    pub fn answer(self) -> String {
        self.answer
    }

    pub fn print(&self) -> AocResult<()> {
        if let Some(expected) = &self.expected {
            if self.answer.eq(expected) {
                io::stdout()
                    .queue(Print(" V ".green()))?
                    .queue(Print(&self.answer))?
                    .queue(Print(" - ".dark_grey()))?
                    .queue(Print(format!("Part {}", self.id)))?
                    .queue(Print(format!(" - {:?}", self.elapsed).dark_grey()))?
                    .queue(Print("\n"))?
                    .flush()?;
            } else {
                io::stdout()
                    .queue(Print(" X ".red()))?
                    .queue(Print(&self.answer))?
                    .queue(Print(" - ".dark_grey()))?
                    .queue(Print(expected))?
                    .queue(Print(" - ".dark_grey()))?
                    .queue(Print(format!("Part {}", self.id)))?
                    .queue(Print(format!(" - {:?}", self.elapsed).dark_grey()))?
                    .queue(Print("\n"))?
                    .flush()?;
            }
        } else {
            io::stdout()
                .queue(Print(" - ".dark_grey()))?
                .queue(Print(&self.answer))?
                .queue(Print(" - ".dark_grey()))?
                .queue(Print(format!("Part {}", self.id)))?
                .queue(Print(format!(" - {:?}", self.elapsed).dark_grey()))?
                .queue(Print("\n"))?
                .flush()?;
        }

        Ok(())
    }
}
