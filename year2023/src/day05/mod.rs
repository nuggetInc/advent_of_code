use std::ops::Range;

use aoc_core::{AocResult, Day};
use itertools::Itertools;

pub fn day() -> Day {
    let mut solution = Day::new();
    solution.part_1(part_one);
    solution.part_2(part_two);
    solution.add_file("files/test.in");
    solution.add_file("files/input.in");
    solution
}

fn parse(input: &String) -> Almanac {
    let mut lines = input.split_terminator("\n\n");

    let seeds: Vec<_> = lines
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    let maps = lines
        .map(|input| {
            input
                .split_terminator('\n')
                .skip(1)
                .map(|line| {
                    let mut numbers = line.split(' ').map(|s| s.parse().unwrap());
                    let dst = numbers.next().unwrap();
                    let src = numbers.next().unwrap();
                    let range = numbers.next().unwrap();
                    (dst..dst + range, src..src + range)
                })
                .collect()
        })
        .collect();

    Almanac::new(seeds, maps)
}

fn part_one(input: &String) -> AocResult<i64> {
    let almanac = parse(input);

    let mut values = almanac.seeds.clone();

    for map in &almanac.maps {
        values = values
            .into_iter()
            .map(|mut value| {
                for (dst, src) in map {
                    if src.contains(&value) {
                        value = value - src.start + dst.start;
                        return value;
                    }
                }

                value
            })
            .collect()
    }

    Ok(values.into_iter().min().unwrap())
}

fn part_two(input: &String) -> AocResult<i64> {
    let almanac = parse(input);

    let mut values: Vec<_> = almanac
        .seeds
        .iter()
        .tuples()
        .map(|(x, y)| *x..*x + *y)
        .collect();

    let mut new_values = Vec::with_capacity(values.len());

    for map in &almanac.maps {
        'outer: while let Some(value) = values.pop() {
            for (dst, src) in map {
                if value.start >= src.start && value.end <= src.end {
                    new_values
                        .push(value.start - src.start + dst.start..value.end - src.end + dst.end);
                    continue 'outer;
                } else if value.start >= src.start && value.start < src.end && value.end >= src.end
                {
                    new_values.push(value.start - src.start + dst.start..dst.end);
                    values.push(src.end..value.end);
                    continue 'outer;
                } else if value.start < src.start && value.end > src.start && value.end <= src.end {
                    new_values.push(dst.start..value.end - src.end + dst.end);
                    values.push(value.start..src.start);
                    continue 'outer;
                }
            }

            new_values.push(value);
        }

        (values, new_values) = (new_values, values)
    }

    Ok(values.into_iter().map(|range| range.start).min().unwrap())
}

struct Almanac {
    seeds: Vec<i64>,
    maps: Vec<Vec<(Range<i64>, Range<i64>)>>,
}

impl Almanac {
    fn new(seeds: Vec<i64>, maps: Vec<Vec<(Range<i64>, Range<i64>)>>) -> Self {
        Self { seeds, maps }
    }
}
