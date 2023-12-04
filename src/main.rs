use std::env;

fn main() {
    let mut year2022 = year2022::year();
    let mut year2023 = year2023::year();

    let mut args = env::args();
    args.next();

    if let Some(year_raw) = args.next() {
        let mut year = match year_raw.as_str() {
            "2022" => year2022,
            "2023" => year2023,
            _ => panic!(
                "The specified day to run is invalid or not implemented: '{}'",
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

            let result = day.run();
            println!("{result}");
        } else {
            let result = year.run();
            println!("{result}");
        }
    } else {
        let result = year2023.run();
        println!("{result}");
    }
}
