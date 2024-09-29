use std::{
    collections::BTreeMap,
    error::Error,
    ffi::OsStr,
    fmt, fs, io,
    panic::{Location, RefUnwindSafe},
    path::PathBuf,
};

use super::{
    part::{AocPart, Part},
    result::DayResult,
};
use crate::{AocResult, Id};

pub struct Day {
    id: Id<Day>,
    parts: BTreeMap<Id<AocPart>, Box<dyn Part>>,
    files: Vec<PathBuf>,
}

impl Day {
    pub fn new(id: impl Into<Id<Day>>) -> Self {
        Self {
            id: id.into(),
            parts: BTreeMap::new(),
            files: Vec::new(),
        }
    }

    pub fn id(&self) -> Id<Day> {
        self.id
    }

    pub fn part_count(&self) -> usize {
        self.parts.len()
    }

    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    pub fn get_part(&self, index: Id<AocPart>) -> Option<&dyn Part> {
        self.parts.get(&index).map(Box::as_ref)
    }

    pub fn run(&self) -> Result<DayResult, DayError> {
        let mut file_parts = Vec::new();

        for input_file in &self.files {
            let mut output_file = input_file.clone();
            output_file.set_extension("out");

            let output = if output_file.exists() {
                fs::read_to_string(output_file).map_err(|err| DayError::OutputFileError(err))?
            } else {
                String::new()
            };

            let mut expected_answers = output.split_terminator('\n');

            let mut parts = BTreeMap::new();

            for part in self.parts.values() {
                let expected = expected_answers.next().map(str::to_owned);

                let result = part.run(input_file, expected);
                parts.insert(part.id(), result);
            }

            file_parts.push((input_file.to_owned(), parts));
        }

        Ok(DayResult::new(self.id, file_parts))
    }

    pub fn part_1<Parsed: 'static, Answer: fmt::Display + 'static>(
        &mut self,
        parser: impl Fn(String) -> Parsed + RefUnwindSafe + 'static,
        part: impl Fn(Parsed) -> AocResult<Answer> + RefUnwindSafe + 'static,
    ) {
        self.parts.insert(
            Id::from(1),
            Box::new(AocPart::new(Id::from(1), parser, part)),
        );
    }

    pub fn part_2<Parsed: 'static, Answer: fmt::Display + 'static>(
        &mut self,
        parser: impl Fn(String) -> Parsed + RefUnwindSafe + 'static,
        part: impl Fn(Parsed) -> AocResult<Answer> + RefUnwindSafe + 'static,
    ) {
        self.parts.insert(
            Id::from(2),
            Box::new(AocPart::new(Id::from(2), parser, part)),
        );
    }

    #[track_caller]
    pub fn add_file(&mut self, path: impl AsRef<OsStr>) {
        let mut full_path = PathBuf::from(Location::caller().file());
        full_path.set_file_name(path);

        if !full_path.exists() {
            eprintln!("The given path '{full_path:?}' does not exist");
            return;
        }

        self.files.push(full_path);
    }
}

#[derive(Debug)]
pub enum DayError {
    Unimplemented,
    OutputFileError(io::Error),
}

impl fmt::Display for DayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DayError::Unimplemented => write!(f, "day is not implemented"),
            DayError::OutputFileError(err) => write!(f, "cannot read output file: {}", err),
        }
    }
}

impl Error for DayError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DayError::Unimplemented => None,
            DayError::OutputFileError(err) => Some(err),
        }
    }
}
