use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use reqwest::blocking::Client;
use reqwest::header::COOKIE;
use scraper::Html;
use scraper::Selector;
use std::collections::HashMap;
use std::env;
use std::fs;

pub use crate::UtilArgs as Args;
pub use clap::Parser;
pub extern crate utils_derive as derive;

const YEAR: i32 = 2024;

#[derive(Debug, Parser)]
#[command(long_about = None)]
pub struct UtilArgs {
    /// input file, AOC_SESSION env must be set if not specified
    #[arg(short, long)]
    input: Option<String>,
    /// run part one, will run both parts if --one and --two not specified
    #[arg(short, long)]
    one: bool,
    /// run part two, will run both parts if --one and --two not specified
    #[arg(short, long)]
    two: bool,
    /// submit the answers to AOC
    #[arg(short, long)]
    submit: bool,
}
impl UtilArgs {
    pub fn get_input(&self, day: i32) -> Result<String> {
        if let Some(file) = &self.input {
            Ok(fs::read_to_string(file)?)
        } else if let Some(session) = env::var_os("AOC_SESSION") {
            let client = Client::new();
            Ok(client
                .get(format!("https://adventofcode.com/{YEAR}/day/{day}/input"))
                .header(
                    COOKIE,
                    format!(
                        "session={}",
                        session
                            .to_str()
                            .ok_or_else(|| anyhow!("cannot convert env to str"))?
                    ),
                )
                .send()?
                .text()?)
        } else {
            bail!("no input file provided or AOC_SESSION set");
        }
    }

    pub fn run_one(&self) -> bool {
        self.one || !self.two
    }

    pub fn run_two(&self) -> bool {
        self.two || !self.one
    }

    pub fn submit_one(&self, day: i32, answer: String) {
        self.submit(day, 1, answer);
    }
    pub fn submit_two(&self, day: i32, answer: String) {
        self.submit(day, 2, answer);
    }
    fn submit(&self, day: i32, level: i32, answer: String) {
        if self.submit {
            let Some(session) = env::var_os("AOC_SESSION") else {
                println!("cannot submit answer no AOC_SESSION env set");
                return;
            };
            let Some(session) = session.to_str() else {
                println!("AOC_SESSION env not a str");
                return;
            };

            let client = Client::new();
            match client
                .post(format!("https://adventofcode.com/{YEAR}/day/{day}/answer"))
                .header(COOKIE, format!("session={session}",))
                .form(&HashMap::from([
                    ("level", level.to_string()),
                    ("answer", answer),
                ]))
                .send()
                .map_err(|e| anyhow!(e))
                .and_then(|resp| {
                    let document = Html::parse_document(&resp.text().map_err(|e| anyhow!(e))?);
                    let main_selector =
                        Selector::parse("main").map_err(|e| anyhow!(format!("{e}")))?;
                    Ok(document
                        .select(&main_selector)
                        .flat_map(|m| m.text())
                        .filter(|s| !s.trim_matches('\n').is_empty())
                        .collect::<String>()
                        .replace("  ", " "))
                }) {
                Ok(response) => println!("Submitted answer:\n{response}"),
                Err(err) => println!("Error submitting answer - {err}"),
            }
        }
    }
}
