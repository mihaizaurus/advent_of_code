use advent_of_code_common::types::{DIAGONALS, DIRECTIONS, Direction, Position};

pub fn result(paper_roll_grid: &Vec<String>) -> isize {
    let mut positions: Vec<Position> = Vec::new();
    
    for y in 0..paper_roll_grid.len() {
        for x in 0..paper_roll_grid[y].len() {
            let position = Position(x, y);
            if is_a_roll(paper_roll_grid, position) {
                let mut adjacent_rolls = 0;
                for direction in DIRECTIONS {
                    // if pos + direction is in bounds, check if roll
                    if is_in_bounds(paper_roll_grid, position, direction) {
                        if is_a_roll(paper_roll_grid, position+direction) {
                            adjacent_rolls += 1;
                        }
                    }
                }
                for diagonal in DIAGONALS {
                    // if pos + direction is in bounds, check if roll
                    if is_in_bounds(paper_roll_grid, position, diagonal) {
                        if is_a_roll(paper_roll_grid, position+diagonal) {
                            adjacent_rolls += 1;
                        }
                    }
                }
                if adjacent_rolls < 4 {
                    positions.push(position);
                }
            }
            else {continue;}
        }
    }
    
    positions.len() as isize
}

fn is_a_roll(paper_roll_grid: &Vec<String>, position: Position) -> bool {
    let Position(x,y) = position;
    paper_roll_grid[y].chars().nth(x).unwrap() == '@'
}

fn is_in_bounds(paper_roll_grid: &Vec<String>, position: Position, direction: Direction) -> bool {
    let Position(x,y) = position;
    let Direction(dx, dy) = direction;
    let (cx,cy) = (x as isize + dx, y as isize + dy);

    if cx < 0 || cy < 0 || cy >= paper_roll_grid.len() as isize || cx >= paper_roll_grid[cy as usize].len() as isize {
        return false
    }
    true
}