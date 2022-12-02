use colored::Colorize;
use reqwest::Url;

use crate::input::get_input_as_string;

pub mod day1;
pub mod day2;

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
