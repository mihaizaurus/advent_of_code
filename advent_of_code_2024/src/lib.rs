use advent_of_code_common::types::PuzzleAnswer;

pub mod puzzles {
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

pub fn run_day(day: usize) -> Result<PuzzleAnswer, String> {
    let input_path = format!("advent_of_code_inputs/2024/day{}.txt", day); 
    let output_path = format!("advent_of_code_results/2024/day{}.txt", day); 

    match day {
        1 => puzzles::day1::result().map_err(|e| e.to_string()),
        2 => puzzles::day2::result().map_err(|e| e.to_string()),
        3 => puzzles::day3::result().map_err(|e| e.to_string()),
        4 => puzzles::day4::result().map_err(|e| e.to_string()),
        5 => puzzles::day5::result().map_err(|e| e.to_string()),
        6 => puzzles::day6::result().map_err(|e| e.to_string()),
        7 => puzzles::day7::result().map_err(|e| e.to_string()),
        8 => puzzles::day8::result(&input_path, &output_path).map_err(|e| e.to_string()),
        9 => puzzles::day9::result(&input_path, &output_path).map_err(|e| e.to_string()),
        10 => puzzles::day10::result(&input_path, &output_path).map_err(|e| e.to_string()),
        11 => puzzles::day11::result(&input_path, &output_path).map_err(|e| e.to_string()),
        12 => puzzles::day12::result(&input_path, &output_path).map_err(|e| e.to_string()),
        13 => puzzles::day13::result(&input_path, &output_path).map_err(|e| e.to_string()),
        14 => puzzles::day14::result(&input_path, &output_path).map_err(|e| e.to_string()),
        15 => puzzles::day15::result(&input_path, &output_path).map_err(|e| e.to_string()),
        _ => Err(format!("Day {} is not implemented or understood...", day)),
    }
}