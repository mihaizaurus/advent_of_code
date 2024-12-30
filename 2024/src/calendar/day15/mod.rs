pub mod part1;
pub mod part2;

use std::io;
use std::io::Write;
use std::fs::File;
use crate::common::types::{Position, PuzzleAnswer, SimpleGrid};
use crate::common::utils::get_input_as_grid;

pub fn result(input_path: &str, output_path: &str) -> io::Result<PuzzleAnswer> {
    let raw_input_data = match get_input_as_grid(input_path) {
        Ok(input) => input,
        Err(err) => {
            eprintln!("Input could not be created from file at {}. Details: {}",input_path, err);
            return Err(err)
        }
    };

    let (mut warehouse_map, robot_start, robot_instructions) = split_raw_input(&raw_input_data);
    let mut warehouse_map_2 = warehouse_map.clone();
    let robot_instructions_2 = robot_instructions.clone();

    let part1 = part1::result(&mut warehouse_map, robot_start, robot_instructions);
    let part2 = part2::result(&mut warehouse_map_2, robot_instructions_2);

    
    let answer: PuzzleAnswer = (part1, part2);
    println!("Part 1: {part1} \n Part 2: {part2}");
    write_answer(output_path, answer)?;
    Ok(answer)
}

fn split_raw_input(raw_input_data: &Vec<String>) -> (SimpleGrid, Position, String) {
    let mut warehouse_map: SimpleGrid = Vec::new();
    let mut raw_robot_instructions: Vec<String> = Vec::new();

    let mut split_index = 0;
    for line_index in 0..raw_input_data.len() {
        if raw_input_data[line_index].as_str().is_empty() {
            split_index = line_index;
            break;
        }
    }

    warehouse_map.extend_from_slice(&raw_input_data[0..split_index]);
    raw_robot_instructions.extend_from_slice(&raw_input_data[split_index+1..]);

    let mut robot_instructions_vec: Vec<char> = raw_robot_instructions[0].chars().collect();
    for i in 1..raw_robot_instructions.len() {
        let mut instructions_vec: Vec<char> = raw_robot_instructions[i].chars().collect();
        robot_instructions_vec.append(&mut instructions_vec);
    }

    let robot_instructions: String = robot_instructions_vec.into_iter().collect();

    let robot_start: Position = get_robot_start(&warehouse_map);
    (warehouse_map, robot_start, robot_instructions)
}

fn get_robot_start(warehouse_map: &Vec<String>) -> Position {
    let mut position: Position = (0,0);
    for row in 0..warehouse_map.len() {
        for col in 0..warehouse_map[0].len() {
            if warehouse_map[row].chars().nth(col).unwrap() == '@' {
                position = (col, row)
            }
        }
    }
    position
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