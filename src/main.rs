use std::env;

use aoc_core::{create_day, download_input, download_problem, DayId, YearId};

fn main() {
    let mut args = env::args();
    args.next();

    if let Err(error) = dotenvy::dotenv() {
        eprintln!("Error loading env file: '{}'", error);
    }

    let command = args
        .next()
        .map(|command_raw| match command_raw.as_str() {
            "run" | "r" => Command::Run,
            "create" | "c" => Command::Create,
            "download" | "d" => Command::Download,
            _ => panic!("The specified command is invalid: '{}'", command_raw),
        })
        .unwrap_or(Command::Run);

    let year: YearId = args
        .next()
        .map(|year_raw| {
            year_raw
                .parse()
                .unwrap_or_else(|_| panic!("The specified year to run is invalid: '{}'", year_raw))
        })
        .unwrap_or(YearId::from(2023));

    if let Some(day) = args.next().map(|day_raw| {
        day_raw
            .parse()
            .unwrap_or_else(|_| panic!("The specified day to run is invalid: '{}'", day_raw))
    }) {
        match command {
            Command::Run => run(year, Some(day)),
            Command::Create => create_day(year, day).unwrap(),
            Command::Download => {
                download_input(year, day).unwrap();
                download_problem(year, day).unwrap();
            }
        }
    } else {
        match command {
            Command::Run => run(year, None),
            Command::Create => panic!("Both year and day must be specified for this command"),
            Command::Download => panic!("Both year and day must be specified for this command"),
        }
    }
}

enum Command {
    Run,
    Create,
    Download,
}

fn run(year: YearId, day: Option<DayId>) {
    let solutions = solutions::solutions();

    let Some(year) = solutions.get_year(year) else {
        panic!("{} is not implemented", year.name());
    };

    if let Some(day) = day {
        let Some(day) = year.get_day(day) else {
            panic!("{} is not implemented", day.name());
        };

        let result = day.run().expect("Couldn't run day");
        result.print().unwrap();
    } else {
        let result = year.run().expect("Couldn't run year");
        result.print().unwrap();
    }
}
