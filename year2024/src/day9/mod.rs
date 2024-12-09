use aoc_core::{AocResult, Day};

pub fn day() -> Day {
    let mut solution = Day::new();
    solution.part_1(part_one);
    solution.part_2(part_two);
    solution.add_file("files/test.in");
    solution.add_file("files/input.in");
    solution
}

fn parse(input: &String) -> Vec<File> {
    let mut files = Vec::new();
    let mut chars = input.chars();

    let mut file_id = 0;
    while let Some(file_size) = chars.next().and_then(|c| c.to_digit(10)) {
        let available = chars.next().and_then(|c| c.to_digit(10)).unwrap_or(0);
        files.push(File {
            id: file_id,
            size: file_size,
            available,
        });

        file_id += 1;
    }

    files
}

fn part_one(input: &String) -> AocResult<u64> {
    let mut files = parse(input);

    let mut start = 0;
    let mut end = files.len() - 1;

    while files[start].available == 0 {
        start += 1;
    }

    while files[end].size == 0 {
        end -= 1;
    }

    while start < end {
        let transfer = files[start].available.min(files[end].size);

        files.insert(
            start + 1,
            File {
                id: files[end].id,
                size: transfer,
                available: files[start].available - transfer,
            },
        );

        end += 1;

        files[start].available = 0;
        files[end].size -= transfer;

        while files[start].available == 0 {
            start += 1;
        }

        while files[end].size == 0 {
            end -= 1;
        }
    }

    let mut sum = 0;
    let mut index = 0;
    for file in &files {
        for _ in 0..file.size {
            sum += index * file.id;
            index += 1;
        }
    }

    Ok(sum)
}

fn part_two(input: &String) -> AocResult<u64> {
    let mut files = parse(input);

    let mut end = files.len() - 1;
    while end > 0 {
        for i in 0..end {
            if files[i].available < files[end].size {
                continue;
            }

            files[end - 1].available += files[end].size + files[end].available;
            files[end].available = files[i].available - files[end].size;
            files[i].available = 0;

            let file = files.remove(end);
            files.insert(i + 1, file);

            end += 1;
            break;
        }

        end -= 1;
    }

    let mut sum = 0;
    let mut index = 0;
    for file in &files {
        for _ in 0..file.size {
            sum += index * file.id;
            index += 1;
        }

        index += file.available as u64
    }

    Ok(sum)
}

struct File {
    id: u64,
    size: u32,
    available: u32,
}
