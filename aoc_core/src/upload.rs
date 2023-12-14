use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{self, Write},
    ops::Deref,
    path::PathBuf,
};

use crossterm::{
    style::{Print, Stylize},
    QueueableCommand,
};
use scraper::{Html, Selector};

use crate::{AocClient, AocResult, Day, PartError, PartId, YearId};

pub fn upload_answer(year_id: YearId, day: &Day) -> AocResult<()> {
    let in_path = PathBuf::from(format!(
        "solutions/{}/src/{}/files/input.in",
        year_id.folder_name(),
        day.id().folder_name()
    ));

    let out_path = PathBuf::from(format!(
        "solutions/{}/src/{}/files/input.out",
        year_id.folder_name(),
        day.id().folder_name()
    ));
    let part_id = if out_path.exists() {
        PartId::from(2)
    } else {
        PartId::from(1)
    };

    let part = day.get_part(part_id).ok_or(PartError::Unimplemented)?;

    let answer = part.run(&in_path, None)?.answer();

    let client = AocClient::default();

    let mut params = HashMap::new();
    params.insert("level".into(), part_id.deref().to_string());
    params.insert("answer".into(), answer.clone());

    let url = format!(
        "https://adventofcode.com/{}/day/{}/answer",
        *year_id,
        *day.id()
    );
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
