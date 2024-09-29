use std::collections::VecDeque;

use aoc_core::{AocResult, Day};
use itertools::Itertools;

pub fn day() -> Day {
    let mut solution = Day::new(21);
    solution.part_1(|s: String| part_one(parse(s)));
    // solution.part_2(|s: String| part_two(parse(s)));
    solution.add_file("files/test.in");
    solution.add_file("files/input.in");
    solution
}

fn parse(input: String) -> (Map, usize, usize) {
    let mut width = 0;
    let mut start_x = 0;
    let mut start_y = 0;

    let grid = input
        .split_terminator('\n')
        .enumerate()
        .map(|(y, line)| {
            width = line.len();
            line.char_indices()
                .map(|(x, char)| match char {
                    '.' => Tile::Plot,
                    '#' => Tile::Rock,
                    'S' => {
                        start_x = x;
                        start_y = y;
                        Tile::Plot
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect_vec();

    let height = grid.len();

    (Map::new(grid, width, height), start_x, start_y)
}

fn part_one((map, start_x, start_y): (Map, usize, usize)) -> AocResult<u32> {
    Ok(find_step_count(&map, start_x, start_y, 0, 64))
}

// fn part_two((map, start_x, start_y): (Map, usize, usize)) -> AocResult<u32> {
//     // const MAX_STEPS: u32 = 26501365;
//     const MAX_STEPS: u32 = 5 + 11 * 3;
//     let map_steps = map.width as u32;
//     let start_steps = start_x as u32;

//     let even_count = find_step_count(&map, start_x, start_y, 0, MAX_STEPS);
//     let odd_count = find_step_count(&map, start_x + 1, start_y, 0, MAX_STEPS);

//     let corner_steps = (MAX_STEPS - start_steps) / map_steps * map_steps + start_steps;

//     let outer_top_count = find_step_count(&map, start_x, map.height - 1, corner_steps, MAX_STEPS);
//     let inner_top_count = find_step_count(
//         &map,
//         start_x,
//         map.height - 1,
//         corner_steps - map_steps,
//         MAX_STEPS,
//     );

//     let outer_bottom_count = find_step_count(&map, start_x, 0, corner_steps, MAX_STEPS);
//     let inner_bottom_count = find_step_count(&map, start_x, 0, corner_steps - map_steps, MAX_STEPS);

//     let outer_left_count = find_step_count(&map, start_x, map.width - 1, corner_steps, MAX_STEPS);
//     let inner_left_count = find_step_count(
//         &map,
//         start_x,
//         map.width - 1,
//         corner_steps - map_steps,
//         MAX_STEPS,
//     );

//     let outer_right_count = find_step_count(&map, start_x, 0, corner_steps, MAX_STEPS);
//     let inner_right_count = find_step_count(&map, start_x, 0, corner_steps - map_steps, MAX_STEPS);

//     let edge_steps =
//         (MAX_STEPS - start_steps * 2) / (map_steps * 2) * (map_steps * 2) + start_steps * 2;

//     Ok(outer_top_count + outer_bottom_count + outer_left_count + outer_right_count)
// }

fn find_step_count(
    map: &Map,
    start_x: usize,
    start_y: usize,
    start_steps: u32,
    max_steps: u32,
) -> u32 {
    if start_steps > max_steps {
        return 0;
    }

    let mut queue = VecDeque::new();
    let mut grid = vec![vec![None::<u32>; map.width]; map.height];
    let mut count = 1;

    grid[start_y][start_x] = Some(start_steps);
    queue.push_back((start_x, start_y));

    while let Some((x, y)) = queue.pop_front() {
        let steps = grid[y][x].unwrap();

        if steps >= max_steps {
            continue;
        }

        if x > 0 && grid[y][x - 1].is_none() && map.grid[y][x - 1] == Tile::Plot {
            grid[y][x - 1] = Some(steps + 1);
            queue.push_back((x - 1, y));

            if steps % 2 == 1 {
                count += 1;
            }
        }

        if y > 0 && grid[y - 1][x].is_none() && map.grid[y - 1][x] == Tile::Plot {
            grid[y - 1][x] = Some(steps + 1);
            queue.push_back((x, y - 1));

            if steps % 2 == 1 {
                count += 1;
            }
        }

        if x < map.width - 1 && grid[y][x + 1].is_none() && map.grid[y][x + 1] == Tile::Plot {
            grid[y][x + 1] = Some(steps + 1);
            queue.push_back((x + 1, y));

            if steps % 2 == 1 {
                count += 1;
            }
        }

        if y < map.height - 1 && grid[y + 1][x].is_none() && map.grid[y + 1][x] == Tile::Plot {
            grid[y + 1][x] = Some(steps + 1);
            queue.push_back((x, y + 1));

            if steps % 2 == 1 {
                count += 1;
            }
        }
    }

    count
}

struct Map {
    grid: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(grid: Vec<Vec<Tile>>, width: usize, height: usize) -> Self {
        Self {
            grid,
            width,
            height,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Plot,
    Rock,
}
