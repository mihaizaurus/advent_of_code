pub mod part1;
pub mod part2;

use std::io;
use std::io::Write;
use std::fs::File;
use regex::Regex;
use advent_of_code_common::types::{Robot, PuzzleAnswer, Position, Direction};
use advent_of_code_common::utils::get_input_as_grid;

pub fn result(input_path: &str, output_path: &str) -> io::Result<PuzzleAnswer> {
    let raw_input_data = match get_input_as_grid(input_path) {
        Ok(input) => input,
        Err(err) => {
            eprintln!("Input could not be created from file at {}. Details: {}",input_path, err);
            return Err(err)
        }
    };

    let map_size = (101,103);
    // let map_size = (11,7); //  for tests only

    let mut robots = get_robots_from_input(raw_input_data);
    let mut robots2 = robots.clone();

    let part1 = part1::result(&mut robots, map_size);
    let part2 = part2::result(&mut robots2, map_size, output_path);

    
    let answer: PuzzleAnswer = (part1, part2);
    println!("Part 1: {part1} \n Part 2: {part2}");
    // write_answer(output_path, answer)?;
    Ok(answer)
}

pub fn get_robots_from_input(raw_input_data: Vec<String>) -> Vec<Robot> {
    let mut robots = Vec::new();

    let robot_pattern = r"p=(-?\d+),(-?\d+).v=(-?\d+),(-?\d+)";

    for raw_input in raw_input_data {
        let re = Regex::new(robot_pattern).unwrap();

        for (_,[px,py,vx,vy]) in re.captures_iter(&raw_input).map(|c| c.extract()) {
            let position = Position(px.parse::<usize>().unwrap(), py.parse::<usize>().unwrap());
            let velocity = Direction(vx.parse::<isize>().unwrap(), vy.parse::<isize>().unwrap());
            let robot = Robot::new(position, velocity);
            robots.push(robot);
        }
    }

    robots
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