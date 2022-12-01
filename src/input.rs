use std::fs::{self, File};
use std::io::{prelude::*, ErrorKind};
use std::path::Path;

use dotenv;
use reqwest::Url;

pub fn get_input_as_string(input_url: &str) -> String {
    let url = input_url.to_string();
    match get_file_path_from_cache(&url) {
        Some(path) => match get_input_as_string_from_cache(&path) {
            Ok(result) => result.trim_end().to_string(),
            Err(_) => get_input_as_string_from_site(&url).trim_end().to_string(),
        },
        None => get_input_as_string_from_site(&url).trim_end().to_string(),
    }
}

const URL_PREFIX: &str = "https://adventofcode.com/";

fn get_input_as_string_from_cache(path: &String) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn get_path_from_input_url(url: &String) -> String {
    let url_postfix = url
        .clone()
        .strip_prefix(URL_PREFIX)
        .expect("Invalid domain.")
        .to_string();
    format!("{}{}{}", "_cache/", url_postfix, ".txt")
}

fn write_new_input_locally(url: &String, input: &String) -> Result<(), std::io::Error> {
    let path = get_path_from_input_url(url);
    let path_obj = Path::new(&path);
    println!("CACHE PATH: {}", path);
    let parent =
        Path::parent(path_obj).ok_or(std::io::Error::new(ErrorKind::Other, "Invalid path."))?;
    fs::create_dir_all(parent)?;
    let mut file = File::create(path.clone())?;
    file.write_all(input.as_bytes())?;

    Ok(())
}

fn get_file_path_from_cache(input_url: &String) -> Option<String> {
    let path = get_path_from_input_url(input_url);
    match File::open(path.clone()) {
        Ok(_) => Some(path),
        Err(_) => None,
    }
}

fn get_input_as_string_from_site(input_url: &String) -> String {
    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

    const KEY: &str = "SESSION";
    let session = dotenv::var(KEY).unwrap();
    let cookie = format!("session={}", session);
    let url = input_url.parse::<Url>().unwrap();

    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();

    let response = client.get(url).header("cookie", cookie).send().unwrap();
    let body = response.text().unwrap();
    write_new_input_locally(input_url, &body).expect("An error occurred while writing the cache.");

    body
}
