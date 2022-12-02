#[tokio::main]
async fn main() {
    let base_url: String = "https://adventofcode.com/2022/day".to_string();
    for day in 1..=25 {
        print!("Fetching Day {:2>}... ", day);
        let fetch_url = format!("{base_url}/{day}/input");
        if !std::path::Path::exists(format!("data/day{day}.txt").as_ref()) {
            if let Ok(resp) = reqwest::get(fetch_url.clone()).await {
                if resp.status() == 400 {
                    open::that(fetch_url).unwrap();
                    println!("OK");
                } else {
                    println!("ERROR, not yet open, aborting...");
                    break;
                }
            }
        } else {
            println!("OK (already fetched)");
        }
    }
}
