use std::fs;

use crate::{download_input, download_problem, AocResult, DayId, YearId};

pub fn create_day(year: YearId, day: DayId) -> AocResult<()> {
    fs::create_dir(format!(
        "solutions/{}/src/{}",
        year.folder_name(),
        day.folder_name()
    ))?;

    fs::write(
        format!(
            "solutions/{}/src/{}/mod.rs",
            year.folder_name(),
            day.folder_name()
        ),
        format!(include_str!("day.txt"), *day),
    )?;

    let days = fs::read_to_string(format!("solutions/{}/src/days.txt", year.folder_name()))?;
    let mut days: Vec<_> = days.split_terminator('\n').map(str::to_owned).collect();
    days.push(day.folder_name());
    days.sort();

    let mut mods = String::new();
    let mut add_days = String::new();
    for day in &days {
        mods += &format!("mod {day};\n");
        add_days += &format!("    year.add_day({day}::day());\n");
    }

    fs::write(
        format!("solutions/{}/src/days.txt", year.folder_name()),
        days.join("\n"),
    )?;

    fs::write(
        format!("solutions/{}/src/lib.rs", year.folder_name()),
        format!(include_str!("year.txt"), mods, add_days),
    )?;

    download_input(year, day)?;
    download_problem(year, day)?;

    Ok(())
}
