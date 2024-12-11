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

    let mut cache = FxHashMap::default();
    cache.insert(0, (1, None));

    for _ in 0..blinks {
        let mut new_stones = FxHashMap::default();

        for (stone, amount) in &stones {
            if let Some((stone_1, stone_2)) = cache.get(stone) {
                insert_or_increment(&mut new_stones, *stone_1, *amount);
                if let Some(stone_2) = stone_2 {
                    insert_or_increment(&mut new_stones, *stone_2, *amount);
                }
            } else if stone.ilog10() % 2 == 1 {
                let power = 10_u64.pow((stone.ilog10() + 1) / 2);

                cache.insert(*stone, (stone / power, Some(stone % power)));
                insert_or_increment(&mut new_stones, stone / power, *amount);
                insert_or_increment(&mut new_stones, stone % power, *amount);
            } else {
                cache.insert(*stone, (stone * 2024, None));
                insert_or_increment(&mut new_stones, stone * 2024, *amount);
            }
        }

        stones = new_stones;
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
