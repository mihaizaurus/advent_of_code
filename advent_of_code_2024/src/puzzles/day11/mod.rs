pub mod part1;
pub mod part2;

use std::io;
use std::io::Write;
use std::fs::File;
use advent_of_code_common::types::PuzzleAnswer;
use advent_of_code_common::utils::get_input_as_vector_128;

pub fn result(input_path: &str, output_path: &str) -> io::Result<PuzzleAnswer> {
    let input_stones = match get_input_as_vector_128(input_path) {
        Ok(input) => input,
        Err(err) => {
            eprintln!("Input could not be created from file at {}. Details: {}",input_path, err);
            return Err(err)
        }
    };

    let part1 = part1::result(&input_stones);
    let part2 = part2::result(&input_stones) as isize;

    
    let answer: PuzzleAnswer = (part1, part2);
    write_answer(output_path, answer)?;
    Ok(answer)
}

fn write_answer(output_path: &str, answer:PuzzleAnswer) -> io::Result<()> {
    let mut file = match File::create(output_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error: Unable to create file at path {}. Details: {}",output_path,error);
            return Err(error);
        }
    };

    writeln!(file,"Puzzle 1 answer: {}", answer.0)?;
    writeln!(file,"Puzzle 2 answer: {}", answer.1)?;

    Ok(())
}