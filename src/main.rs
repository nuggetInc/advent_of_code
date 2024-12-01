mod solutions;

use std::{collections::VecDeque, env, error::Error, fmt, io};

use aoc_core::{
    create_day, upload_answer, AocResult, Day, DayError, Id, Problem, ProblemInput, Year, YearError,
};
use crossterm::{
    style::{Print, Stylize},
    ExecutableCommand, QueueableCommand,
};
use solutions::get_solutions;

fn main() -> AocResult<()> {
    if let Err(error) = dotenvy::dotenv() {
        eprintln!("Error loading env file: '{}'", error);
    }

    let args: VecDeque<String> = env::args().skip(1).collect();

    if !args.is_empty() {
        if let Err(error) = execute_command(args) {
            io::stderr().execute(Print(format!("{}\n", error).red()))?;
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
            io::stderr().execute(Print(format!("{}\n", error).red()))?;
        }

        io::stdout().execute(Print(">>> "))?;
    }

    Ok(())
}

fn execute_command(mut args: VecDeque<String>) -> AocResult<()> {
    let command_raw = args.pop_front().ok_or(CommandError::InvalidCommand)?;
    let command = match command_raw.as_str() {
        "run" | "r" => Command::Run,
        "create" | "c" => Command::Create,
        "download" | "d" => Command::Download,
        "upload" | "u" => Command::Upload,
        _ => Err(CommandError::InvalidCommand)?,
    };

    let year_id = args
        .pop_front()
        .map(|year_raw| year_raw.parse())
        .unwrap_or(Ok(Id::from(2024)))?;

    if let Some(day_raw) = args.pop_front() {
        let day_id = day_raw.parse()?;

        match command {
            Command::Run => run(year_id, Some(day_id))?,
            Command::Create => create_day(year_id, day_id)?,
            Command::Download => {
                ProblemInput::download(year_id, day_id)?.write(year_id, day_id)?;

                let problem = Problem::download(year_id, day_id)?;
                problem.write_readme(year_id, day_id)?;
                problem.write_out_file(year_id, day_id)?;
            }
            Command::Upload => upload(year_id, day_id)?,
        }
    } else {
        match command {
            Command::Run => run(year_id, None)?,
            _ => Err(CommandError::DayRequired)?,
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

fn run(year_id: Id<Year>, day_id: Option<Id<Day>>) -> AocResult<()> {
    let solutions = get_solutions();

    let Some(year) = solutions.get_year(year_id) else {
        Err(YearError::Unimplemented)?
    };

    if let Some(day) = day_id {
        let Some(day) = year.get_day(day) else {
            Err(DayError::Unimplemented)?
        };

        let result = day.run()?;
        result.print()
    } else {
        let result = year.run();

        io::stdout().queue(Print(format!("Year {year_id}")))?;
        result.print()
    }
}

fn upload(year_id: Id<Year>, day_id: Id<Day>) -> AocResult<()> {
    let solutions = get_solutions();

    let Some(year) = solutions.get_year(year_id) else {
        Err(YearError::Unimplemented)?
    };

    let Some(day) = year.get_day(day_id) else {
        Err(DayError::Unimplemented)?
    };

    upload_answer(year_id, day_id, day)
}

#[derive(Debug)]
enum CommandError {
    InvalidCommand,
    DayRequired,
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::InvalidCommand => write!(f, "not a valid command"),
            CommandError::DayRequired => write!(f, "command requires day"),
        }
    }
}

impl Error for CommandError {}
