use clap::{Parser};
use reqwest::blocking::Client;
use reqwest::header::COOKIE;
use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use dotenv::dotenv;

#[derive(Parser, Debug)]
#[clap(version, about = "Fetches Advent of Code inputs")]
struct Args {
    /// The year of the Advent of Code event
    #[clap(short, long)]
    year: u32,

    /// The day of the challenge (1-25)
    #[clap(short, long)]
    day: u32,

    /// The root working directory (for flexibility)
    #[clap(long, default_value = ".")]
    cwd: PathBuf,
}

fn main() {
    dotenv().ok(); // Load .env variables

    let args = Args::parse();
    
    let session_cookie = env::var("SESSION").expect("Missing SESSION in .env");
    let url = format!(
        "https://adventofcode.com/{}/day/{}/input",
        args.year, args.day
    );

    println!("Fetching input for Year {} Day {}...", args.year, args.day);

    let client = Client::new();
    let response = client
        .get(&url)
        .header(COOKIE, format!("session={}", session_cookie))
        .send()
        .expect("Failed to send request");

    if response.status().is_success() {
        let input = response.text().expect("Failed to read response");

        // Construct the correct directory path
        let input_dir = args.cwd.join(format!("advent_of_code_inputs/calendar/{}/puzzles", args.year));
        let input_path = input_dir.join(format!("day{}.txt", args.day));

        // Ensure directory exists
        fs::create_dir_all(&input_dir).expect("Failed to create input directory");
        
        // Write input to file
        let mut file = fs::File::create(&input_path)
            .expect("Failed to create input file");
        file.write_all(input.as_bytes())
            .expect("Failed to write input file");

        println!("✅ Saved input to {}", input_path.display());
    } else {
        eprintln!("❌ Failed to download input: {:?}", response.status());
    }
}
