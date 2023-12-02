mod parser;
mod part;

use std::{collections::HashMap, fs::File, io::Read, path::Path};

use self::{parser::Parser, part::Part};

pub trait AocDay {
    fn run(&mut self, part: Option<String>);
}

impl<T> AocDay for Day<T> {
    fn run(&mut self, part: Option<String>) {
        for file in &mut self.files {
            let mut input = String::new();
            file.read_to_string(&mut input).unwrap();
            let parsed = self.parser.run(input);

            if let Some(name) = &part {
                let answer = self.parts[name].run(&parsed);
                println!("{name}: {answer}");
            } else {
                for (name, part) in &self.parts {
                    let answer = part.run(&parsed);
                    println!("{name}: {answer}");
                }
            }
        }
    }
}

pub struct Day<T> {
    parser: Parser<T>,
    parts: HashMap<String, Part<T>>,
    files: Vec<File>,
}

impl<T> Day<T> {
    pub fn new(parser: impl Fn(String) -> T + 'static) -> Self {
        Self {
            parser: Parser::new(parser),
            parts: HashMap::new(),
            files: Vec::new(),
        }
    }

    pub fn add_part(&mut self, name: String, part: impl Fn(&T) -> String + 'static) {
        self.parts.insert(name, Part::new(part));
    }

    pub fn add_file(&mut self, path: impl AsRef<Path>) {
        self.files.push(File::open(path).unwrap());
    }
}
