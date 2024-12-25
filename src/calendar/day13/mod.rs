pub mod part1;
pub mod part2;

use std::io;
use std::io::Write;
use std::fs::File;
use regex::Regex;
use crate::common::types::{Direction, Position, PuzzleAnswer, SimpleGrid, Machine};
use crate::common::utils::get_input_as_grid;

pub fn result(input_path: &str, output_path: &str) -> io::Result<PuzzleAnswer> {
    let raw_input_data = match get_input_as_grid(input_path) {
        Ok(input) => input,
        Err(err) => {
            eprintln!("Input could not be created from file at {}. Details: {}",input_path, err);
            return Err(err)
        }
    };

    let machines = get_machines_from_input(&raw_input_data);

    for machine in &machines {
        println!("{:?}",machine);
    }

    let part1 = part1::result(&machines);
    let part2 = part2::result(&machines);

    
    let answer: PuzzleAnswer = (part1, part2);
    println!("Part 1: {part1} \n Part 2: {part2}");
    write_answer(output_path, answer)?;
    Ok(answer)
}

fn get_machines_from_input(input: &SimpleGrid) -> Vec<Machine> {
    let mut raw_machines: Vec<Vec<String>> = input.split(|x| x.as_str().is_empty()).map(|x| x.to_vec()).collect();
    let mut machines = Vec::new();

    for raw_machine in raw_machines {
        let machine = get_machine(raw_machine);
        machines.push(machine);
    }
    machines
}

fn get_machine(raw_machine: Vec<String>) -> Machine {
    let mut button_a: Direction = (0,0);
    let mut button_b: Direction = (0,0);
    let mut prize_location: Position = (0,0);
    
    let button_pattern_x = r"X\+(\d+)";
    let button_pattern_y = r"Y\+(\d+)";
    let prize_pattern_x = r"X\=(\d+)";
    let prize_pattern_y = r"Y\=(\d+)";

    for i in 0..raw_machine.len() {
        match i {
            0 => {
                let re_x = Regex::new(button_pattern_x).unwrap();
                let re_y = Regex::new(button_pattern_y).unwrap();

                for (_,[value]) in re_x.captures_iter(&raw_machine[i]).map(|c| c.extract()) {
                    let x = value.parse::<isize>().unwrap();
                    button_a.0 = x;
                }
                for (_,[value]) in re_y.captures_iter(&raw_machine[i]).map(|c| c.extract()) {
                    let y = value.parse::<isize>().unwrap();
                    button_a.1 = y;
                }
            },
            1 => {
                let re_x = Regex::new(button_pattern_x).unwrap();
                let re_y = Regex::new(button_pattern_y).unwrap();

                for (_,[value]) in re_x.captures_iter(&raw_machine[i]).map(|c| c.extract()) {
                    let x = value.parse::<isize>().unwrap();
                    button_b.0 = x;
                }
                for (_,[value]) in re_y.captures_iter(&raw_machine[i]).map(|c| c.extract()) {
                    let y = value.parse::<isize>().unwrap();
                    button_b.1 = y;
                }
            },
            2 => {
                let re_x = Regex::new(prize_pattern_x).unwrap();
                let re_y = Regex::new(prize_pattern_y).unwrap();

                for (_,[value]) in re_x.captures_iter(&raw_machine[i]).map(|c| c.extract()) {
                    let x = value.parse::<usize>().unwrap();
                    prize_location.0 = x;
                }
                for (_,[value]) in re_y.captures_iter(&raw_machine[i]).map(|c| c.extract()) {
                    let y = value.parse::<usize>().unwrap();
                    prize_location.1 = y;
                }
            },
            _ => ()
        }
    }
    
    Machine::new(button_a, button_b, prize_location)
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