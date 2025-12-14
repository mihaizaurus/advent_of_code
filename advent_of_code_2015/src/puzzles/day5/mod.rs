pub mod part1;
pub mod part2;

use std::io::{self, Write};
use std::fs::File;
use advent_of_code_common::types::PuzzleAnswer;
use advent_of_code_common::utils::get_input_as_grid;

pub fn result(input_path: &str, output_path: &str) -> io::Result<PuzzleAnswer> {
    let raw_input = match get_input_as_grid(input_path) {
        Ok(input) => input,
        Err(err) => {
            eprintln!("Input could not be created from file at {}. Details: {}",input_path, err);
            return Err(err)
        }
    };

    let split_index = raw_input.iter().position(|value| value.is_empty()).unwrap();

    let mut raw_ranges = raw_input.clone();
    let mut raw_ids = raw_ranges.split_off(split_index);
    raw_ids.remove(0); // removes empty line split

    let ranges: Vec<(usize, usize)> = parse_ranges(&raw_ranges);
    let ids: Vec<usize> = parse_ids(&raw_ids);

    let part1 = part1::result(&ranges, &ids);
    let part2 = part2::result(&ranges);
    
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

fn parse_ranges(raw_ranges: &Vec<String>) -> Vec<(usize, usize)> {
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    for raw_range in raw_ranges {
        let (start,end) = raw_range
            .split_once('-')
            .unwrap();
        let start = start.parse::<usize>().unwrap();
        let end = end.parse::<usize>().unwrap();
    ranges.push((start,end));
    }
    ranges.sort_by(|a,b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    ranges
}

fn parse_ids(raw_ids: &Vec<String>) -> Vec<usize> {
    let mut ids: Vec<usize> = Vec::new();
    for raw_id in raw_ids {
        let id = raw_id.parse::<usize>().unwrap();
        ids.push(id);
    }
    ids
}