use std::collections::HashSet;

use aoc_core::{AocResult, Day};

pub fn day() -> Day {
    let mut solution = Day::new(14);
    solution.part_1(|s: String| part_one(parse(s)));
    solution.part_2(|s: String| part_two(parse(s)));
    solution.add_file("files/input.in");
    solution
}

fn part_one(walls: Vec<Vec<(u32, u32)>>) -> AocResult<i32> {
    let mut occupied = HashSet::new();

    let mut bottom_y = 0;

    for wall in walls {
        for section in wall.windows(2) {
            if let [from, to] = section {
                let min_x = from.0.min(to.0);
                let max_x = from.0.max(to.0);
                let min_y = from.1.min(to.1);
                let max_y = from.1.max(to.1);

                if min_y > bottom_y {
                    bottom_y = min_y;
                }

                for x in min_x..=max_x {
                    for y in min_y..=max_y {
                        occupied.insert((x, y));
                    }
                }
            }
        }
    }

    let mut count = 0;

    'outer: loop {
        let mut x = 500;
        let mut y = 0;

        loop {
            if y >= bottom_y {
                break 'outer;
            }

            if !occupied.contains(&(x, y + 1)) {
                y += 1;
            } else if !occupied.contains(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
            } else if !occupied.contains(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
            } else {
                break;
            }
        }

        count += 1;
        occupied.insert((x, y));
    }

    Ok(count)
}

fn part_two(walls: Vec<Vec<(u32, u32)>>) -> AocResult<i32> {
    let mut occupied = HashSet::new();

    let mut bottom_y = 0;

    for wall in walls {
        for section in wall.windows(2) {
            if let [from, to] = section {
                let min_x = from.0.min(to.0);
                let max_x = from.0.max(to.0);
                let min_y = from.1.min(to.1);
                let max_y = from.1.max(to.1);

                if min_y > bottom_y {
                    bottom_y = min_y;
                }

                for x in min_x..=max_x {
                    for y in min_y..=max_y {
                        occupied.insert((x, y));
                    }
                }
            }
        }
    }

    let mut count = 0;

    loop {
        let mut x = 500;
        let mut y = 0;

        loop {
            if y > bottom_y {
                break;
            } else if !occupied.contains(&(x, y + 1)) {
                y += 1;
            } else if !occupied.contains(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
            } else if !occupied.contains(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
            } else {
                break;
            }
        }

        count += 1;
        occupied.insert((x, y));

        if y == 0 {
            break;
        }
    }

    Ok(count)
}

fn parse(input: String) -> Vec<Vec<(u32, u32)>> {
    let mut walls = Vec::new();

    for line in input.split_terminator('\n') {
        let mut wall = Vec::new();

        for position in line.split(" -> ") {
            let (x, y) = position.split_once(',').unwrap();

            wall.push((x.parse().unwrap(), y.parse().unwrap()));
        }

        walls.push(wall);
    }

    walls
}
