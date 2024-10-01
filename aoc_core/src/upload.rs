use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    io::{self, Write},
    ops::Deref,
    path::PathBuf,
};

use crossterm::{
    style::{Print, Stylize},
    QueueableCommand,
};
use scraper::{Html, Selector};

use crate::{AocClient, AocResult, Day, Id, PartError, Problem, Year};

pub fn upload_answer(year_id: Id<Year>, day_id: Id<Day>, day: &Day) -> AocResult<()> {
    let input_file = PathBuf::from(format!("year{year_id}/src/day{day_id}/files/input.in"));

    let out_path = PathBuf::from(format!("{year_id}/src/day{day_id}/files/input.out"));
    let part_id = if out_path.exists() && fs::metadata(&out_path)?.len() > 0 {
        Id::from(2)
    } else {
        Id::from(1)
    };

    let part = day.get_part(part_id).ok_or(PartError::Unimplemented)?;

    let input = fs::read_to_string(input_file)?;
    let answer = part.run(&input, None)?.answer();

    let client = AocClient::default();

    let mut params = HashMap::new();
    params.insert("level".into(), part_id.deref().to_string());
    params.insert("answer".into(), answer.clone());

    let url = format!("https://adventofcode.com/{year_id}/day/{day_id}/answer");
    let text = client.post(&url, params)?.text()?;

    let document = Html::parse_document(&text);
    let articles_selector = Selector::parse("body > main > article")?;

    let article = document.select(&articles_selector).next().unwrap();

    let response = article.text().collect::<String>();
    if response.starts_with("That's the right answer") {
        io::stdout()
            .queue(Print(" V ".green()))?
            .queue(Print(response))?
            .queue(Print("\n"))?
            .flush()?;

        let mut out_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(out_path)?;

        writeln!(out_file, "{}", answer)?;

        Problem::download(year_id, day_id)?.write_readme(year_id, day_id)?;
    } else if response.starts_with("That's not the right answer") {
        io::stdout()
            .queue(Print(" X ".red()))?
            .queue(Print(response))?
            .queue(Print("\n"))?
            .flush()?;
    } else if response.starts_with("You gave an answer too recently") {
        io::stdout()
            .queue(Print(" / ".yellow()))?
            .queue(Print(response))?
            .queue(Print("\n"))?
            .flush()?;
    } else {
        io::stdout()
            .queue(Print(" - ".dark_grey()))?
            .queue(Print(response))?
            .queue(Print("\n"))?
            .flush()?;
    }

    Ok(())
}
