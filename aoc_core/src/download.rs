use std::{
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

pub struct Problem {
    description: String,
    answers: Vec<String>,
}

impl Problem {
    fn new(description: String, answers: Vec<String>) -> Self {
        Self {
            description,
            answers,
        }
    }

    pub fn download(year_id: YearId, day_id: DayId) -> AocResult<Self> {
        let client = AocClient::default();

        let url = format!("https://adventofcode.com/{}/day/{}", *year_id, *day_id);
        let text = client.get(&url)?.text()?;

        let document = Html::parse_document(&text);
        let articles_selector = Selector::parse("body > main > article")?;
        let p_selector = Selector::parse("body > main > p")?;

        let mut description = format!(
            "view the original on <a href={}>adventofcode.com</a>\n",
            url
        );

        for article in document.select(&articles_selector) {
            description += &article
                .inner_html()
                .replace("<em>", "<b>")
                .replace("</em>", "</b>")
                .replace(r#" target="_blank""#, "")
        }

        let mut answers = Vec::new();

        for p in document.select(&p_selector) {
            let mut text = p.text();

            if text
                .next()
                .filter(|s| *s == "Your puzzle answer was ")
                .is_none()
            {
                continue;
            }

            if let Some(answer) = text.next() {
                answers.push(answer.to_owned());
            }
        }

        Ok(Problem::new(description, answers))
    }

    pub fn write_readme(&self, year_id: YearId, day_id: DayId) -> AocResult<()> {
        let mut file = File::create(format!(
            "solutions/{}/src/{}/README.md",
            year_id.folder_name(),
            day_id.folder_name()
        ))?;

        writeln!(file, "{}", self.description)?;

        Ok(())
    }

    pub fn write_out_file(&self, year_id: YearId, day_id: DayId) -> AocResult<()> {
        let mut file = File::create(format!(
            "solutions/{}/src/{}/files/input.out",
            year_id.folder_name(),
            day_id.folder_name()
        ))?;

        writeln!(file, "{}", self.answers.join("\n"))?;

        Ok(())
    }
}
