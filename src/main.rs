use std::ffi::OsStr;

use termion::color::{Black, Fg, Reset};

mod year2023;
fn main() {
    let mut year = year2023::year2023();
    let result = year.run();

    println!(
        "{: <40}{}{: >20?}{}",
        result.name,
        Fg(Black),
        result.elapsed,
        Fg(Reset),
    );

    for (day_name, day) in result.days {
        println!();
        println!(
            "{: <40}{}{: >20?}{}",
            day_name,
            Fg(Black),
            day.elapsed,
            Fg(Reset),
        );

        for parser in day.parsers {
            println!(
                "{: <40}{}{: >20?}{}",
                parser.file.file_name().and_then(OsStr::to_str).unwrap(),
                Fg(Black),
                parser.elapsed,
                Fg(Reset),
            );
        }

        for part in day.parts {
            println!(
                "{: <15}{}{: <15}{}{: <15}{}{: >15?}{}",
                part.name,
                Fg(Black),
                part.file.file_name().and_then(OsStr::to_str).unwrap(),
                Fg(Reset),
                part.result,
                Fg(Black),
                part.elapsed,
                Fg(Reset),
            );
        }
    }
}
