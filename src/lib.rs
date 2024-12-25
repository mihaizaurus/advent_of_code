use common::types::PuzzleAnswer;

pub mod calendar {
    pub mod day1;
    pub mod day10;
    pub mod day11;
    pub mod day12;
    pub mod day13;
}

pub mod common;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub fn run_day(day: usize) -> Result<PuzzleAnswer, String> {
    let input_path = format!("inputs/day{}.txt", day); 
    let output_path = format!("results/day{}.txt", day); 

    match day {
        1 => calendar::day1::result().map_err(|e| e.to_string()),
        2 => day2::result().map_err(|e| e.to_string()),
        3 => day3::result().map_err(|e| e.to_string()),
        4 => day4::result().map_err(|e| e.to_string()),
        5 => day5::result().map_err(|e| e.to_string()),
        6 => day6::result().map_err(|e| e.to_string()),
        7 => day7::result().map_err(|e| e.to_string()),
        8 => day8::result(&input_path, &output_path).map_err(|e| e.to_string()),
        9 => day9::result(&input_path, &output_path).map_err(|e| e.to_string()),
        10 => calendar::day10::result(&input_path, &output_path).map_err(|e| e.to_string()),
        11 => calendar::day11::result(&input_path, &output_path).map_err(|e| e.to_string()),
        12 => calendar::day12::result(&input_path, &output_path).map_err(|e| e.to_string()),
        13 => calendar::day13::result(&input_path, &output_path).map_err(|e| e.to_string()),
        _ => Err(format!("Day {} is not implemented or understood...", day)),
    }
}