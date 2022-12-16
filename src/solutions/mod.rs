use colored::Colorize;
use regex::Regex;
use reqwest::Url;

use crate::input::get_input_as_string;

// solutions
pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

// revisions
pub mod day2rev;
pub mod day3rev;
pub mod day5rev;
pub mod day6rev;

// visualizations
pub mod day12vis;
pub mod day14vis;
pub mod day7vis;
pub mod day8vis;
pub mod day9vis;

pub fn input_raw(day: u8) -> String {
    let url = format!("https://adventofcode.com/2022/day/{}/input", day).to_string();
    get_input_as_string(&url)
}

pub fn final_answer<T: std::fmt::Display>(answer: T, submit: bool, day: u8, level: u8) {
    println!(
        "\n{}",
        format!(
            "   Solution {}",
            format!(" {} ", answer).black().on_yellow().bold()
        )
        .bold()
        .on_blue()
    );

    if submit {
        let url = format!("https://adventofcode.com/2022/day/{}/answer", day);
        let request = format!("level={}&answer={}", level, answer);
        let response = perform_submit(&url, request);

        if response.contains("day-success") {
            println!("{}", "Accepted!".bold().on_blue());
        } else if response.contains("Did you already complete it?") {
            println!("{}", "Solution already accepted...".bold().on_white());
        } else if response.contains("left to wait.") {
            // You have 13s left to wait.
            let time_capture_regex = Regex::new(r"You have (.+) left to wait.").unwrap();
            let captures_result = time_capture_regex.captures(&response);
            println!("{}", "    SLOW DOWN    ".bold().on_red());
            match captures_result {
                Some(captures) => {
                    println!(
                        "Please wait {}.",
                        format!("{}", captures.get(1).unwrap().as_str())
                            .bold()
                            .on_red()
                    );
                }
                None => {
                    println!("Could not determine time before next submission...");
                }
            }
        } else {
            println!("{}", "Innaccurate!".bold().on_bright_red());
        }
    }
    println!();
}

fn perform_submit(submit_url: &String, body: String) -> String {
    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

    const KEY: &str = "SESSION";
    let session = dotenv::var(KEY).unwrap();
    let cookie = format!("session={}", session);
    let url = submit_url.parse::<Url>().unwrap();

    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();

    let response = client
        .post(url)
        .header("cookie", cookie)
        .header("content-type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .unwrap();
    let body = response.text().unwrap();

    body
}
