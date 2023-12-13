use aoc_core::Solutions;

pub fn solutions() -> Solutions {
    let mut solutions = Solutions::default();

    solutions.add_year(year2022::year());
    solutions.add_year(year2023::year());

    solutions
}
