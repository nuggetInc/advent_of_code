use aoc_core::{AocResult, Day};
use fxhash::FxHashMap;

pub fn day() -> Day {
    let mut solution = Day::new();
    solution.part_1(|input| part(input, 25));
    solution.part_2(|input| part(input, 75));
    solution.add_file("files/test.in");
    solution.add_file("files/input.in");
    solution
}

fn parse(input: &String) -> FxHashMap<u64, u64> {
    let mut stones = FxHashMap::default();
    for stone in input.split(' ') {
        insert_or_increment(&mut stones, stone.trim().parse().unwrap(), 1);
    }

    stones
}

fn part(input: &String, blinks: u32) -> AocResult<u64> {
    let mut stones = parse(input);
    let mut new_stones = FxHashMap::default();

    for _ in 0..blinks {
        for (stone, amount) in &stones {
            if *stone == 0 {
                insert_or_increment(&mut new_stones, 1, *amount);
            } else if stone.ilog10() % 2 == 1 {
                let power = 10_u64.pow((stone.ilog10() + 1) / 2);

                insert_or_increment(&mut new_stones, stone / power, *amount);
                insert_or_increment(&mut new_stones, stone % power, *amount);
            } else {
                insert_or_increment(&mut new_stones, stone * 2024, *amount);
            }
        }

        (stones, new_stones) = (new_stones, stones);
        new_stones.clear();
    }

    Ok(stones.values().sum::<u64>())
}

fn insert_or_increment(map: &mut FxHashMap<u64, u64>, key: u64, value: u64) {
    if let Some(map_value) = map.get_mut(&key) {
        *map_value += value;
    } else {
        map.insert(key, value);
    }
}
