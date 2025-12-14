use std::env;
use advent_of_code_2024::run_day;

fn main() {
    println!("Hello, advent of code 2024!");
    
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <day>", args[0]);
        return;
    }

    match args[1].parse::<usize>() {
        Ok(day) => {
            if let Err(err) = run_day(2024, day) {
                eprintln!("Error running day {}: {}", day, err);
            }
        },
        Err(_) => eprintln!("Invalid Input. Please enter a valid day number. (Range 1-25)")
    }
}