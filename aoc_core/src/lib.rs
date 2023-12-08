mod day;
mod result;
mod year;

use std::{
    fs::File,
    io::{self, Write},
};

pub use day::Day;
use ego_tree::NodeRef;
use reqwest::Url;
use scraper::{Html, Node, Selector};
pub use year::{Year, YearDay};


pub fn download_problem(year: u32, day: YearDay) {
    let client = reqwest::blocking::Client::new();

    let url = Url::parse(&format!(
        "https://adventofcode.com/{}/day/{}",
        year,
        u32::from(day)
    ))
    .unwrap();
    let response = client.get(url).header("Cookie", COOKIE).send().unwrap();
    let text = response.text().unwrap();

    let mut file = File::create(format!(
        "solutions/year{}/src/{}/README.md",
        year,
        day.folder_name()
    ))
    .unwrap();

    let document = Html::parse_document(&text);
    let articles_selector = Selector::parse("body > main > article").unwrap();
    for article in document.select(&articles_selector) {
        write_children(&mut file, *article).unwrap();
    }
}

fn write_children(file: &mut File, node: NodeRef<'_, Node>) -> io::Result<()> {
    for child in node.children() {
        match child.value() {
            Node::Element(element) => match element.name() {
                "h2" => {
                    write!(file, "## ")?;
                    write_children(file, child)?;
                    writeln!(file)?;
                }
                "p" => {
                    write_children(file, child)?;
                    writeln!(file)?;
                }
                "pre" => {
                    writeln!(file, "```")?;
                    write_children(file, child)?;
                    writeln!(file, "```")?;
                }
                "code" => match node.value() {
                    Node::Element(element) => {
                        if element.name() == "pre" {
                            write_children(file, child)?;
                        } else {
                            write!(file, "`")?;
                            write_children(file, child)?;
                            write!(file, "`")?;
                        }
                    }
                    _ => (),
                },
                "em" => {
                    write!(file, "**")?;
                    write_children(file, child)?;
                    write!(file, "**")?;
                }
                "ul" => {
                    write_children(file, child)?;
                }
                "li" => {
                    write!(file, "- ")?;
                    write_children(file, child)?;
                }
                _ => (),
            },
            Node::Text(text) => write!(file, "{}", text.to_string())?,
            _ => (),
        }
    }

    Ok(())
}
