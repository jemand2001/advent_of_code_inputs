use error_chain::error_chain;
// use futures::executor::block_on;
use reqwest::{self, header::HeaderMap};
use std::{env, fs::{self, File, read_to_string}, path::{Path, PathBuf}, io::copy};
use tokio::runtime::Builder;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Http(reqwest::Error);
    }
}

fn path_for(day: i32) -> PathBuf {
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
    let cookie = contents.strip_suffix('\n').unwrap();
    let mut headers = HeaderMap::new();
    headers.insert("Cookie", cookie.parse().unwrap());
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

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Invalid number of arguments. Usage: get_inputs <day>");
    }
    let day = args[1].parse::<i32>().expect("Argument 1 isn't an int");
    let path = path_for(day);
    fs::create_dir_all(path.parent().unwrap().as_os_str()).expect("couldn't create inputs directory");
    if path.exists() {
        println!("Path already exists!");
        return Ok(());
    }
    println!("Path doesn't already exist, downloading!");
    get_input(day, path)?;
    Ok(())
}
