use advent_of_code_common::types::PuzzleAnswer;
use advent_of_code_common::utils::{get_puzzle_input_path,get_puzzle_results_path};

pub mod puzzles {
    pub mod day1;
    pub mod day2;
    pub mod day3;
}

pub fn run_day(year: usize, day: usize) -> Result<PuzzleAnswer, String> {
    let input_path_buf = get_puzzle_input_path(year, day); 
    let input_path = input_path_buf.to_str().unwrap();
    let output_path_buf = get_puzzle_results_path(year, day);
    let output_path = output_path_buf.to_str().unwrap();

    match day {
        1 => puzzles::day1::result(&input_path, &output_path).map_err(|e| e.to_string()),
        2 => puzzles::day2::result(&input_path, &output_path).map_err(|e| e.to_string()),
        3 => puzzles::day3::result(&input_path, &output_path).map_err(|e| e.to_string()),
        _ => Err(format!("Day {} is not implemented or understood...", day)),
    }
}