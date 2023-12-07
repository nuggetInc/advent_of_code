use std::collections::HashMap;

use aoc_core::{Day, YearDay};

pub fn day() -> Day {
    let mut solution = Day::new(YearDay::Day07);
    solution.part_1(parse, part_one);
    solution.part_2(parse, part_two);
    solution.add_file("input.txt");
    solution
}

fn part_one(root: Directory) -> String {
    let mut sum = 0;

    let mut directories = Vec::new();
    directories.push(&root);

    while let Some(directory) = directories.pop() {
        directories.extend(directory.items.iter().filter_map(|(_, item)| match item {
            Item::Directory(directory) => Some(directory),
            Item::File(_) => None,
        }));

        let size = directory.get_size();

        if size <= 100000 {
            sum += size;
        }
    }

    sum.to_string()
}

fn part_two(root: Directory) -> String {
    let mut sizes = Vec::new();

    let mut directories = Vec::new();
    directories.push(&root);

    while let Some(directory) = directories.pop() {
        directories.extend(directory.items.iter().filter_map(|(_, item)| match item {
            Item::Directory(directory) => Some(directory),
            Item::File(_) => None,
        }));

        sizes.push(directory.get_size());
    }

    let required = sizes[0] - 40000000;

    let mut smallest_size = sizes[0];

    for size in sizes {
        if size < smallest_size && size >= required {
            smallest_size = size;
        }
    }

    smallest_size.to_string()
}

fn parse(input: String) -> Directory {
    let mut root = Directory::default();

    let mut path: Vec<String> = Vec::new();

    let mut lines = input.split("\n").peekable();

    while let Some(line) = lines.next() {
        if line.starts_with("$ cd") {
            let directory = &line[5..];

            match directory {
                "/" => path.clear(),
                ".." => {
                    path.pop();
                }
                _ => path.push(directory.into()),
            }
        } else {
            while lines.peek().is_some() && !lines.peek().unwrap().starts_with('$') {
                let line = lines.next().unwrap();

                let mut split = line.split(" ");
                let first = split.next().unwrap();
                let second = split.next().unwrap();

                if first == "dir" {
                    let directory = Item::new_directory();

                    root.get_directory_mut(path.iter())
                        .unwrap()
                        .items
                        .insert(second.into(), directory)
                        .and_then::<(), _>(|_| unreachable!());
                } else {
                    let file = Item::new_file(first.parse().unwrap());

                    root.get_directory_mut(path.iter())
                        .unwrap()
                        .items
                        .insert(second.into(), file)
                        .and_then::<(), _>(|_| unreachable!());
                }
            }
        }
    }

    root
}

#[derive(Debug)]
enum Item {
    Directory(Directory),
    File(File),
}

impl Item {
    fn new_directory() -> Self {
        Self::Directory(Directory::default())
    }

    fn new_file(size: usize) -> Self {
        Self::File(File::new(size))
    }

    fn get_size(&self) -> usize {
        match self {
            Item::Directory(value) => value.get_size(),
            Item::File(value) => value.size,
        }
    }
}

#[derive(Debug, Default)]
struct Directory {
    items: HashMap<String, Item>,
}

impl Directory {
    fn get_directory_mut<'a>(
        &mut self,
        mut path: impl Iterator<Item = &'a String>,
    ) -> Option<&mut Directory> {
        let Some(next) = path.next() else {
            return Some(self);
        };

        let Some(Item::Directory(directory)) = self.items.get_mut(next) else {
            return None;
        };

        directory.get_directory_mut(path)
    }

    fn get_size(&self) -> usize {
        let mut sum = 0;

        for (_, item) in &self.items {
            sum += item.get_size();
        }

        sum
    }
}

#[derive(Debug)]
struct File {
    size: usize,
}

impl File {
    fn new(size: usize) -> Self {
        Self { size }
    }
}
