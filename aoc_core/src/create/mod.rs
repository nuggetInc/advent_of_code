use std::fs;

use crate::{AocResult, DayId, Problem, ProblemInput, YearId};

pub fn create_day(year_id: YearId, day_id: DayId) -> AocResult<()> {
    fs::create_dir(format!(
        "solutions/{}/src/{}",
        year_id.folder_name(),
        day_id.folder_name()
    ))?;

    fs::write(
        format!(
            "solutions/{}/src/{}/mod.rs",
            year_id.folder_name(),
            day_id.folder_name()
        ),
        format!(include_str!("day.txt"), *day_id),
    )?;

    let days = fs::read_to_string(format!("solutions/{}/src/days.txt", year_id.folder_name()))?;
    let mut days: Vec<_> = days.split_terminator('\n').map(str::to_owned).collect();
    days.push(day_id.folder_name());
    days.sort();

    let mut mods = String::new();
    let mut add_days = String::new();
    for day in &days {
        mods += &format!("mod {day};\n");
        add_days += &format!("    year.add_day({day}::day());\n");
    }

    fs::write(
        format!("solutions/{}/src/days.txt", year_id.folder_name()),
        days.join("\n"),
    )?;

    fs::write(
        format!("solutions/{}/src/lib.rs", year_id.folder_name()),
        format!(include_str!("year.txt"), mods, add_days),
    )?;

    ProblemInput::download(year_id, day_id)?.write(year_id, day_id)?;
    Problem::download(year_id, day_id)?.write_readme(year_id, day_id)?;

    Ok(())
}
