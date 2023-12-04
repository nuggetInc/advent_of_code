use std::env;

fn main() {
    let mut year = year2023::year2023();

    let mut args = env::args();
    args.next();

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
}
