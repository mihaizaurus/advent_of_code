use crate::common::{types::{Direction, Position, SimpleGrid, DIRECTIONS}, utils::{get_char_at, set}};

pub fn result(warehouse_map: &mut SimpleGrid, robot_start: Position, robot_instructions: String) -> isize {
    let mut robot_position: Position = robot_start;

    /* move robot around while also moving boxes except when they reach a wall
    calculate at the end the position of all crates and their "GPS coordinates" which are 100 * y + 1 * x */

    let instructions: Vec<char> = robot_instructions.chars().collect();

    for instruction in instructions {
        let direction = get_direction_from_instruction(instruction);
        /* Check if robot can move in direction.
        if next tile is a 0 -> crate, could be moved, track it and check next position, 
        if next tile is a . -> empty space, robot plus tracked elements can move there
        if next tile is a # -> wall, cannot move, go to next instruction */
        let mut act = false;
        let mut obstacle_ahead = false;
        let mut next_start = robot_position + direction;

        while !act {
            let Position(next_x, next_y) = next_start;
            match get_char_at(Position(next_x, next_y), warehouse_map).as_str() {
                "O" => {
                    obstacle_ahead = true;
                },
                "." => {
                    if obstacle_ahead == true {
                        set(warehouse_map, Position(next_x, next_y), 'O');
                    }
                    // move robot
                    set(warehouse_map, robot_position + direction, '@');
                    set(warehouse_map, robot_position, '.');
                    robot_position += direction;
                    act = true;
                },
                "#" => {
                    // skip turn
                    act = true;
                },
                _ => {}
            }
            next_start += direction;
        }        
        
    }

    println!("Part 1: \n {:#?}",warehouse_map);
    calculate_score(warehouse_map)
}

fn get_direction_from_instruction(instruction: char) -> Direction{
    let mut direction = Direction(0,0);
    match instruction {
        '^' => {
            direction = DIRECTIONS[0];
        },
        '>' => {
            direction = DIRECTIONS[1];
        },
        'v' => {
            direction = DIRECTIONS[2];
        },
        '<' => {
            direction = DIRECTIONS[3];
        },
        _ => {
            eprintln!("Unknown instruction: {}", instruction);
        }            
    }
    direction
}

fn calculate_score(warehouse_map: &SimpleGrid) -> isize {
    let mut score = 0;
    for row in 0..warehouse_map.len() {
        for col in 0..warehouse_map[0].len() {
            if get_char_at(Position(col,row), warehouse_map) == "O" {
                score += (100 * row) + col;
            }
        }
    }
    score as isize
}