pub mod part1;
pub mod part2;

use std::io::{self, Write};
use std::fs::File;
use advent_of_code_common::types::PuzzleAnswer;
use advent_of_code_common::utils::get_input_as_grid;

pub fn result(input_path: &str, output_path: &str) -> io::Result<PuzzleAnswer> {
    let raw = match get_input_as_grid(input_path) {
        Ok(input) => input,
        Err(err) => {
            eprintln!("Input could not be created from file at {}. Details: {}",input_path, err);
            return Err(err)
        }
    };

    let raw_line = raw.first().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Puzzle input is empty"))?;

    let id_ranges = get_id_ranges(raw_line)?;

    let part1 = part1::result(&id_ranges);
    let part2 = part2::result(&id_ranges);

    
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

fn get_id_ranges(raw_line: &str) -> io::Result<Vec<(u128,u128)>> {
    raw_line.split(',')
        .filter(|s| !s.is_empty())
        .map(|part| {
            let (start,end) = part
                .split_once('-')
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData,format!("Parse range error {part}")))?;

            Ok((
                start.parse::<u128>()
                    .map_err(|_| io::Error::new(io::ErrorKind::InvalidData,format!("Range start error {start}")))?,
                end.parse::<u128>()
                    .map_err(|_| io::Error::new(io::ErrorKind::InvalidData,format!("Range end error {end}")))?,
            ))
        })
        .collect()
}