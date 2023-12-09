use std::env::{self, Args};

use aoc_core::download_problem;

fn main() {
    let mut args = env::args();
    args.next();

    if let Some(command_raw) = args.next() {
        match command_raw.as_str() {
            "run" | "r" => run(args),
            "Download" | "d" => download(args),
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

        if let Some(yearday_raw) = args.next() {
            let Ok(yearday) = (&yearday_raw).try_into() else {
                panic!("The specified day to run is invalid: '{}'", yearday_raw);
            };

            let Some(day) = year.get_day(&yearday) else {
                panic!(
                    "The specified day to run is not implemented: '{}'",
                    yearday_raw
                );
            };

            let result = day.run().expect("Couldn't run day");
            println!("{result}");
        } else {
            let result = year.run().expect("Couldn't run year");
            println!("{result}");
        }
    } else {
        let result = year2023::year().run().expect("Couldn't run year");
        println!("{result}");
    }
}

fn download(mut args: Args) {
    let Some(year_raw) = args.next() else {
        panic!("No year or day specified");
    };

    let Ok(year) = year_raw.parse() else {
        panic!("The specified year to download is invalid: '{}'", year_raw);
    };

    if year < 2015 {
        panic!("The specified year must be after 2015");
    }

    let Some(yearday_raw) = args.next() else {
        panic!("No day specified");
    };

    let Ok(yearday) = (&yearday_raw).try_into() else {
        panic!(
            "The specified day to download is invalid: '{}'",
            yearday_raw
        );
    };

    download_problem(year, yearday);
}
