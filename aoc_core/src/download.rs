use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use scraper::{Html, Selector};

use crate::{AocClient, AocResult, DayId, YearId};

pub fn download_input(year: YearId, day: DayId) -> AocResult<()> {
    let client = AocClient::default();

    let url = format!("https://adventofcode.com/{}/day/{}/input", *year, *day);
    let text = client.get(url)?.text()?;

    let files_dir = PathBuf::from(format!(
        "solutions/{}/src/{}/files",
        year.folder_name(),
        day.folder_name()
    ));
    if !files_dir.exists() {
        fs::create_dir(files_dir)?;
    }

    fs::write(
        format!(
            "solutions/{}/src/{}/files/input.in",
            year.folder_name(),
            day.folder_name()
        ),
        text,
    )?;

    Ok(())
}

pub fn download_problem(year: YearId, day: DayId) -> Result<(), Box<dyn Error>> {
    let client = AocClient::default();

    let url = format!("https://adventofcode.com/{}/day/{}", *year, *day);
    let text = client.get(&url)?.text()?;

    let mut file = File::create(format!(
        "solutions/{}/src/{}/README.md",
        year.folder_name(),
        day.folder_name()
    ))?;

    let document = Html::parse_document(&text);
    let articles_selector = Selector::parse("body > main > article")?;

    writeln!(
        file,
        "view the original on <a href={}>adventofcode.com</a>",
        url
    )?;

    for article in document.select(&articles_selector) {
        writeln!(
            file,
            "{}",
            article
                .inner_html()
                .replace("<em>", "<b>")
                .replace("</em>", "</b>")
                .replace(r#" target="_blank""#, "")
        )?;
    }

    Ok(())
}
