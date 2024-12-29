use common::types::PuzzleAnswer;

pub mod calendar {
    pub mod day1;
    pub mod day2;
    pub mod day3;
    pub mod day4;
    pub mod day5;
    pub mod day6;
    pub mod day7;
    pub mod day8;
    pub mod day9;
    pub mod day10;
    pub mod day11;
    pub mod day12;
    pub mod day13;
    pub mod day14;
    pub mod day15;
}

pub mod common;


pub fn run_day(day: usize) -> Result<PuzzleAnswer, String> {
    let input_path = format!("inputs/day{}.txt", day); 
    let output_path = format!("results/day{}.txt", day); 

    match day {
        1 => calendar::day1::result().map_err(|e| e.to_string()),
        2 => calendar::day2::result().map_err(|e| e.to_string()),
        3 => calendar::day3::result().map_err(|e| e.to_string()),
        4 => calendar::day4::result().map_err(|e| e.to_string()),
        5 => calendar::day5::result().map_err(|e| e.to_string()),
        6 => calendar::day6::result().map_err(|e| e.to_string()),
        7 => calendar::day7::result().map_err(|e| e.to_string()),
        8 => calendar::day8::result(&input_path, &output_path).map_err(|e| e.to_string()),
        9 => calendar::day9::result(&input_path, &output_path).map_err(|e| e.to_string()),
        10 => calendar::day10::result(&input_path, &output_path).map_err(|e| e.to_string()),
        11 => calendar::day11::result(&input_path, &output_path).map_err(|e| e.to_string()),
        12 => calendar::day12::result(&input_path, &output_path).map_err(|e| e.to_string()),
        13 => calendar::day13::result(&input_path, &output_path).map_err(|e| e.to_string()),
        14 => calendar::day14::result(&input_path, &output_path).map_err(|e| e.to_string()),
        15 => calendar::day15::result(&input_path, &output_path).map_err(|e| e.to_string()),
        _ => Err(format!("Day {} is not implemented or understood...", day)),
    }
}