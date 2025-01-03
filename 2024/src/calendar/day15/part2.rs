use crate::common::{types::{Direction, Position, SimpleGrid, DIRECTIONS}, utils::{get_char_at, set}};

pub fn result(warehouse_map: &mut SimpleGrid, robot_instructions: String) -> isize {
    widen_map(warehouse_map);
    let mut robot_position: Position = get_updated_robot_position(warehouse_map);

    // println!("Widened Map: \n {:#?}",warehouse_map);

    let instructions: Vec<char> = robot_instructions.chars().collect();

    for instruction in instructions {
        let direction = get_direction_from_instruction(instruction);

        let mut act = false;
        
        let mut next_start = robot_position + direction;

        if direction == Direction(-1,0) || direction == Direction(1,0) { // Left or Right, easy logic like part 1.
            let mut obstacle_ahead = false;             
            while !act {
                let Position(next_x, next_y) = next_start;
                match get_char_at(Position(next_x, next_y), warehouse_map).as_str() {
                    "[" | "]" => {
                        obstacle_ahead = true
                    },
                    "." => {
                        if obstacle_ahead == true {
                            if direction == Direction(-1,0) {
                                set(warehouse_map, Position(next_x, next_y), ']');
                                set(warehouse_map, Position(next_x - 1, next_y), '[');
                                // set(warehouse_map, Position(next_x + 1, next_y), '.');
                                // .[][][]@
                            }
                            else if direction == Direction(1,0) {
                                set(warehouse_map, Position(next_x, next_y), '[');
                                set(warehouse_map, Position(next_x + 1, next_y), ']');
                            }
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
               
        
    }

    println!("Part 2: \n {:#?}",warehouse_map);
    calculate_score(warehouse_map)
}

#[derive(Debug)]
struct Crate {
    left: Position,
    right: Position
}

fn can_move(warehouse_map: &SimpleGrid, crate_to_check: Crate, direction: Direction) -> bool {
    match direction {
        Direction(-1,0) | Direction(1,0) => {
            let crate_edge =  if direction == Direction (-1,0) {crate_to_check.left} else {crate_to_check.right};
            match get_char_at(crate_edge + direction, warehouse_map).as_str() {
                "[" | "]" => {
                    let next_crate = Crate {
                        left: crate_to_check.left + direction + direction,
                        right: crate_to_check.right + direction + direction
                    };
                    return can_move(warehouse_map, next_crate, direction)
                },
                "." => return true,
                "#" => return false,
                _ => panic!("Left Panicked!")
            }
        },
        Direction(0,1) | Direction(0,-1) => {
            let mut was_checked = false;
            let can_move_left = match get_char_at(crate_to_check.left + direction, warehouse_map).as_str() {
                "[" => {
                    let next_crate = Crate {
                        left: crate_to_check.left + direction,
                        right: crate_to_check.right + direction
                    };
                    was_checked = true;
                    can_move(warehouse_map, next_crate, direction)
                }, 
                "]" => {
                    let next_crate = Crate {
                        left: crate_to_check.left + direction + Direction(-1,0),
                        right: crate_to_check.left + direction
                    };
                    can_move(warehouse_map, next_crate, direction)
                },
                "." => true,
                "#" => false,
                _ => panic!("Left Panicked!")
            };
            let can_move_right = match get_char_at(crate_to_check.right + direction, warehouse_map).as_str() {
                "[" => {
                    let next_crate = Crate {
                        left: crate_to_check.right + direction,
                        right: crate_to_check.right + direction + Direction(1,0)
                    };
                    can_move(warehouse_map, next_crate, direction)
                }, 
                "]" => {
                    if was_checked == true {println!("Crate to check: {:?} HELP!!!!!",crate_to_check); return true}
                    let next_crate = Crate {
                        left: crate_to_check.left + direction,
                        right: crate_to_check.right + direction
                    };
                    can_move(warehouse_map, next_crate, direction)
                },
                "." => true,
                "#" => false,
                _ => panic!("Right Panicked!")
            };
            println!("Crate to check: {:?} !!!!!",crate_to_check);
            if can_move_left == true && can_move_right == true {return true} else {return false}
        },
        _ => panic!("Direction Undefined!")
    } 
        
    
}

fn get_updated_robot_position(warehouse_map: &SimpleGrid) -> Position {
    let mut robot_position: Position = Position(0,0);

    for row in 0..warehouse_map.len() {
        for col in 0..warehouse_map[0].len() {
            if warehouse_map[row].chars().nth(col).unwrap() == '@' {
                robot_position = Position(col, row)
            }
        }
    }
    robot_position
}

fn widen_map(warehouse_map: &mut SimpleGrid) {
    for row in warehouse_map.iter_mut() {
        *row = row.chars()
        .flat_map(|c| match c {
            '#' => "##".chars(),  // Replace '#' with "##"
            'O' => "[]".chars(), // Replace 'O' with "[]"
            '.' => "..".chars(), // Replace '.' with ".."
            '@' => "@.".chars(), // Replace '@' with "@."
            _ => "..".chars(), // Default: duplicate the character
        })
        .collect();
    }
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
            if get_char_at(Position(col,row), warehouse_map) == "[" {
                score += (100 * row) + col;
            }
        }
    }
    score as isize
}