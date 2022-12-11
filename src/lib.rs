use error_chain::error_chain;
// use futures::executor::block_on;
use reqwest::{self, header::{HeaderMap, InvalidHeaderValue}};
use std::{fs::{self, File, read_to_string}, path::{Path, PathBuf}, io::copy};
use tokio::runtime::Builder;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Http(reqwest::Error);
        Header(InvalidHeaderValue);
        ParseInt(std::num::ParseIntError);
    }
}

pub fn path_for(day: i32) -> PathBuf {
    let p = Path::new("inputs/");
    return p.join(day.to_string());
}

fn get_url(day: i32) -> String {
    return String::from("https://adventofcode.com/2022/day/") + &day.to_string() + "/input"
}

async fn download(url: String, target: PathBuf) -> Result<()> {
    let env_path = Path::new(".env");
    if !env_path.exists() || !env_path.is_file() {
        // return Err(Error(ErrorKind::Io(), "No cookie file found! (expected as .env)"));
        panic!("No cookie file found! (expected as .env)");
    }
    let contents = read_to_string(env_path)?;
    let cookie = contents.trim();
    let mut headers = HeaderMap::new();
    let cookie_parsed = cookie.parse()?;
    headers.insert("Cookie", cookie_parsed);
    let client = reqwest::Client::new();
    let response = client.get(url).headers(headers).send().await?;
    let mut file = File::create(target)?;
    let content = response.text().await?;
    copy(&mut content.as_bytes(), &mut file)?;
    Ok(())
}

fn get_input(day: i32, target: PathBuf) -> Result<()> {
    Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()?
        .block_on(download(get_url(day), target))
}

pub fn read_input(day: i32) -> Result<String> {
    // Ok(String::new())
    let path = path_for(day);
    fs::create_dir_all(path.parent().unwrap().as_os_str()).expect("couldn't create inputs directory");
    if path.exists() {
        println!("Path already exists!");
    }
    else {
        println!("Path doesn't already exist, downloading!");
        get_input(day, path.to_owned())?;
    }
    let content = read_to_string(path)?;
    Ok(content)
}

pub fn run_on_input<T, F, P>(day: i32, runner: F, parser: P) -> Result<()> where F : Fn(T), P : Fn(String) -> Result<T> {
    let input = read_input(day)?;
    let parsed = parser(input)?;
    runner(parsed);
    Ok(())
}
