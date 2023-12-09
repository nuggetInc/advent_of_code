mod day;
mod result;
mod year;

use std::{fs::File, io::Write};

pub use day::Day;
use scraper::{Html, Selector};
pub use year::{Year, YearDay};


pub fn download_problem(year: u32, day: YearDay) {
    let client = reqwest::blocking::Client::new();

    let url = format!("https://adventofcode.com/{}/day/{}", year, u32::from(day));
    let response = client.get(&url).header("Cookie", COOKIE).send().unwrap();
    let text = response.text().unwrap();

    let mut file = File::create(format!(
        "solutions/year{}/src/{}/README.md",
        year,
        day.folder_name()
    ))
    .unwrap();

    let document = Html::parse_document(&text);
    let articles_selector = Selector::parse("body > main > article").unwrap();

    writeln!(
        file,
        "view the original on <a href={}>adventofcode.com</a>",
        url
    )
    .unwrap();

    for article in document.select(&articles_selector) {
        writeln!(
            file,
            "{}",
            article
                .inner_html()
                .replace("<em>", "<b>")
                .replace("</em>", "</b>")
        )
        .unwrap();
    }
}
