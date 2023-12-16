use std::{
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
};

use scraper::{Html, Selector};

use crate::{AocClient, DayId, YearId};

pub struct ProblemInput(String);

impl ProblemInput {
    pub fn download(year_id: YearId, day_id: DayId) -> reqwest::Result<ProblemInput> {
        let client = AocClient::default();

        let url = format!(
            "https://adventofcode.com/{}/day/{}/input",
            *year_id, *day_id
        );
        let text = client.get(url)?.text()?;

        Ok(Self(text))
    }

    pub fn write(&self, year_id: YearId, day_id: DayId) -> io::Result<()> {
        let mut path = PathBuf::from(format!(
            "solutions/{}/src/{}/files/",
            year_id.folder_name(),
            day_id.folder_name()
        ));
        if !path.exists() {
            fs::create_dir(&path)?;
        }

        path.push("input.in");
        fs::write(path, &self.0)
    }
}

pub struct Problem {
    description: String,
    answers: Vec<String>,
}

impl Problem {
    pub fn download(year_id: YearId, day_id: DayId) -> reqwest::Result<Problem> {
        let client = AocClient::default();

        let url = format!("https://adventofcode.com/{}/day/{}", *year_id, *day_id);
        let text = client.get(&url)?.text()?;

        let document = Html::parse_document(&text);
        let articles_selector = Selector::parse("body > main > article").unwrap();
        let p_selector = Selector::parse("body > main > p").unwrap();

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

        Ok(Self {
            description,
            answers,
        })
    }

    pub fn write_readme(&self, year_id: YearId, day_id: DayId) -> io::Result<()> {
        let mut file = File::create(format!(
            "solutions/{}/src/{}/README.md",
            year_id.folder_name(),
            day_id.folder_name()
        ))?;

        writeln!(file, "{}", self.description)
    }

    pub fn write_out_file(&self, year_id: YearId, day_id: DayId) -> io::Result<()> {
        if self.answers.is_empty() {
            return Ok(());
        }

        let mut file = File::create(format!(
            "solutions/{}/src/{}/files/input.out",
            year_id.folder_name(),
            day_id.folder_name()
        ))?;

        writeln!(file, "{}", self.answers.join("\n"))
    }
}
