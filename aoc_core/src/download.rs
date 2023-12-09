use std::{
    env,
    fs::{self, File},
    io::Write,
};

use scraper::{Html, Selector};

use crate::YearDay;

pub fn download_input(year: u32, day: YearDay) {
    let client = reqwest::blocking::Client::new();

    let cookie = env::var("AOC_SESSION").expect("AOC_SESSION was not set");

    let url = format!(
        "https://adventofcode.com/{}/day/{}/input",
        year,
        u32::from(day)
    );
    let response = client.get(&url).header("Cookie", cookie).send().unwrap();
    let text = response.text().unwrap();

    fs::write(
        format!("solutions/year{}/src/{}/input.txt", year, day.folder_name()),
        text,
    )
    .unwrap();
}

pub fn download_problem(year: u32, day: YearDay) {
    let client = reqwest::blocking::Client::new();

    let cookie = env::var("AOC_SESSION").expect("AOC_SESSION was not set");

    let url = format!("https://adventofcode.com/{}/day/{}", year, u32::from(day));
    let response = client.get(&url).header("Cookie", cookie).send().unwrap();
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
                .replace(r#" target="_blank""#, "")
        )
        .unwrap();
    }
}
