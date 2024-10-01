use std::{
    collections::BTreeMap,
    error::Error,
    ffi::OsStr,
    fmt, fs, io,
    panic::{Location, RefUnwindSafe},
    path::PathBuf,
};

use super::{part::Part, result::DayResult};
use crate::{AocResult, Id};

pub struct Day {
    id: Id<Day>,
    parts: BTreeMap<Id<Part>, Part>,
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

    pub fn get_part(&self, index: Id<Part>) -> Option<&Part> {
        self.parts.get(&index)
    }

    pub fn run(&self) -> Result<DayResult, DayError> {
        let mut file_parts = Vec::new();

        for input_file in &self.files {
            let input =
                fs::read_to_string(input_file).map_err(|err| DayError::InputFileError(err))?;

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

                let result = part.run(&input, expected);
                parts.insert(part.id(), result);
            }

            file_parts.push((input_file.to_owned(), parts));
        }

        Ok(DayResult::new(self.id, file_parts))
    }

    pub fn part_1<Answer: fmt::Display + 'static>(
        &mut self,
        solver: impl Fn(&String) -> AocResult<Answer> + RefUnwindSafe + 'static,
    ) {
        self.parts
            .insert(Id::from(1), Part::new(Id::from(1), solver));
    }

    pub fn part_2<Answer: fmt::Display + 'static>(
        &mut self,
        solver: impl Fn(&String) -> AocResult<Answer> + RefUnwindSafe + 'static,
    ) {
        self.parts
            .insert(Id::from(2), Part::new(Id::from(2), solver));
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
    InputFileError(io::Error),
    OutputFileError(io::Error),
}

impl fmt::Display for DayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DayError::Unimplemented => write!(f, "day is not implemented"),
            DayError::InputFileError(err) => write!(f, "cannot read input file: {}", err),
            DayError::OutputFileError(err) => write!(f, "cannot read output file: {}", err),
        }
    }
}

impl Error for DayError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DayError::Unimplemented => None,
            DayError::InputFileError(err) | DayError::OutputFileError(err) => Some(err),
        }
    }
}
