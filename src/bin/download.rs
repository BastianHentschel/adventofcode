use std::fs::File;
use std::io::Read;
use clap::{Parser};

#[derive(Parser)]
struct Args {
    #[clap()]
    year: u32,
    #[clap()]
    day: u32,
}


fn main() {
    let mut cookie = String::new();
    File::open("cookie").unwrap().read_to_string(&mut cookie).expect("Missing `cookie` file");
    let args = Args::parse();
    let year = args.year;
    let day = args.day;
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let client = reqwest::blocking::Client::new();
    let resp = client.get(&url)
        .header("Cookie", format!("session={}", cookie))
        .send()
        .expect("Failed to send request");
    let body = resp.text().expect("Failed to get body");
    let path = format!("data/year{}/day{:02}.txt", year, day);
    std::fs::write(&path, body).expect("Failed to write file");
    println!("Wrote to {}", path);
}