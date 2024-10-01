use aoc_core::{Id, Solutions};

pub fn get_solutions() -> Solutions {
    let mut solutions = Solutions::default();

    solutions.add_year(Id::from(2022), year2022::year());
    solutions.add_year(Id::from(2023), year2023::year());

    solutions
}
