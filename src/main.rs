use std::env::{self, Args};

use aoc_core::{create_day, download_input, download_problem, YearId};

fn main() {
    let mut args = env::args();
    args.next();

    if let Err(error) = dotenvy::dotenv() {
        eprintln!("Error loading env file: '{}'", error);
    }

    if let Some(command_raw) = args.next() {
        match command_raw.as_str() {
            "run" | "r" => run(args),
            "download" | "d" => download(args),
            "create" | "c" => create(args),
            _ => panic!("The specified command is invalid: '{}'", command_raw),
        }
    }
}

fn run(mut args: Args) {
    if let Some(year_raw) = args.next() {
        let mut year = match year_raw.as_str() {
            "2022" => year2022::year(),
            "2023" => year2023::year(),
            _ => panic!(
                "The specified year to run is invalid or not implemented: '{}'",
                year_raw
            ),
        };

        if let Some(day_raw) = args.next() {
            let Ok(day) = day_raw.parse() else {
                panic!("The specified day to run is invalid: '{}'", day_raw);
            };

            let Some(day) = year.get_day(day) else {
                panic!("The specified day to run is not implemented: '{}'", day_raw);
            };

            let result = day.run().expect("Couldn't run day");
            result.print().unwrap();
        } else {
            let result = year.run().expect("Couldn't run year");
            result.print().unwrap();
        }
    } else {
        let result = year2023::year().run().expect("Couldn't run year");
        result.print().unwrap();
    }
}

fn download(mut args: Args) {
    let Some(year_raw) = args.next() else {
        panic!("No year or day specified");
    };

    let Ok(year) = year_raw.parse::<YearId>() else {
        panic!("The specified year to download is invalid: '{}'", year_raw);
    };

    if *year < 2015 {
        panic!("The specified year must be after 2015");
    }

    let Some(day_raw) = args.next() else {
        panic!("No day specified");
    };

    let Ok(day) = day_raw.parse() else {
        panic!("The specified day to download is invalid: '{}'", day_raw);
    };

    download_input(year, day).unwrap();
    download_problem(year, day).unwrap();
}

fn create(mut args: Args) {
    let Some(year_raw) = args.next() else {
        panic!("No year or day specified");
    };

    let Ok(year) = year_raw.parse::<YearId>() else {
        panic!("The specified year to download is invalid: '{}'", year_raw);
    };

    if *year < 2015 {
        panic!("The specified year must be after 2015");
    }

    let Some(day_raw) = args.next() else {
        panic!("No day specified");
    };

    let Ok(day) = day_raw.parse() else {
        panic!("The specified day to download is invalid: '{}'", day_raw);
    };

    create_day(year, day).unwrap();
}
