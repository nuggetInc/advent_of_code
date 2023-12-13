use std::{
    collections::VecDeque,
    env,
    io::{self, Write},
};

use aoc_core::{
    create_day, download_input, download_problem, upload_answer, AocError, AocResult, DayId, YearId,
};
use crossterm::{
    style::{Print, Stylize},
    ExecutableCommand, QueueableCommand,
};

fn main() -> AocResult<()> {
    if let Err(error) = dotenvy::dotenv() {
        eprintln!("Error loading env file: '{}'", error);
    }

    let args: VecDeque<String> = env::args().skip(1).collect();

    if !args.is_empty() {
        if let Err(error) = execute_command(args) {
            io::stderr()
                .queue(Print(format!("{}\n", error).red()))?
                .flush()?;
        }
        return Ok(());
    }

    io::stdout().execute(Print(">>> "))?;

    for line in io::stdin().lines() {
        let args = line?
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .collect();

        if let Err(error) = execute_command(args) {
            io::stderr()
                .queue(Print(format!("{}\n", error).red()))?
                .flush()?;
        }

        io::stdout().execute(Print(">>> "))?;
    }

    Ok(())
}

fn execute_command(mut args: VecDeque<String>) -> AocResult<()> {
    let command_raw = args
        .pop_front()
        .ok_or(AocError::InvalidCommand(String::new()))?;
    let command = match command_raw.as_str() {
        "run" | "r" => Command::Run,
        "create" | "c" => Command::Create,
        "download" | "d" => Command::Download,
        "upload" | "u" => Command::Upload,
        _ => Err(AocError::InvalidCommand(command_raw))?,
    };

    let year_id: YearId = args
        .pop_front()
        .map(|year_raw| {
            year_raw
                .parse()
                .map_err(|_| AocError::InvalidYear(year_raw))
        })
        .unwrap_or(Ok(YearId::from(2023)))?;

    if let Some(day_raw) = args.pop_front() {
        let day_id = day_raw.parse().map_err(|_| AocError::InvalidDay(day_raw))?;

        match command {
            Command::Run => run(year_id, Some(day_id))?,
            Command::Create => create_day(year_id, day_id)?,
            Command::Download => {
                download_input(year_id, day_id)?;
                download_problem(year_id, day_id)?;
            }
            Command::Upload => upload(year_id, day_id)?,
        }
    } else {
        match command {
            Command::Run => run(year_id, None)?,
            _ => Err(AocError::InvalidDay(String::new()))?,
        }
    }

    Ok(())
}

enum Command {
    Run,
    Create,
    Download,
    Upload,
}

fn run(year_id: YearId, day_id: Option<DayId>) -> AocResult<()> {
    let solutions = solutions::solutions();

    let Some(year) = solutions.get_year(year_id) else {
        Err(AocError::UnimplementedYear(year_id))?
    };

    if let Some(day) = day_id {
        let Some(day) = year.get_day(day) else {
            Err(AocError::UnimplementedDay(day))?
        };

        let result = day.run()?;
        result.print()
    } else {
        let result = year.run()?;
        result.print()
    }
}

fn upload(year_id: YearId, day_id: DayId) -> AocResult<()> {
    let solutions = solutions::solutions();

    let Some(year) = solutions.get_year(year_id) else {
        Err(AocError::UnimplementedYear(year_id))?
    };

    let Some(day) = year.get_day(day_id) else {
        Err(AocError::UnimplementedDay(day_id))?
    };

    upload_answer(year_id, day)
}
