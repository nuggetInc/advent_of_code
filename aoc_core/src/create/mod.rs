use std::fs;

use crate::{AocResult, Day, Id, Problem, ProblemInput, Year};

pub fn create_day(year_id: Id<Year>, day_id: Id<Day>) -> AocResult<()> {
    fs::create_dir(format!("year{year_id}/src/day{day_id}"))?;

    fs::write(
        format!("year{year_id}/src/day{day_id}/mod.rs"),
        format!(include_str!("day.txt"), day_id),
    )?;

    let days = fs::read_to_string(format!("year{year_id}/src/days.txt"))?;
    let mut days: Vec<_> = days.split_terminator('\n').map(str::to_owned).collect();
    days.push(day_id.to_string());
    days.sort();

    let mut mods = String::new();
    let mut add_days = String::new();
    for day in &days {
        mods += &format!("mod day{day};\n");
        add_days += &format!("    year.add_day(day{day}::day());\n");
    }

    fs::write(format!("year{year_id}/src/days.txt"), days.join("\n"))?;

    fs::write(
        format!("year{year_id}/src/lib.rs"),
        format!(include_str!("year.txt"), mods, add_days),
    )?;

    ProblemInput::download(year_id, day_id)?.write(year_id, day_id)?;
    Problem::download(year_id, day_id)?.write_readme(year_id, day_id)?;

    Ok(())
}
