use advent_of_code_common::{types::{Position, SimpleGrid, DIRECTIONS}, utils::{get_char_at, move_from}};
use std::collections::HashSet;

pub fn result(farm: &SimpleGrid) -> isize {
    // get plots as contiguous range of same char value (plant type)
    // get perimeter and area of each

    let regions = farm_as_map(farm);
    let mut total_price = 0;
    
    for (_,region) in regions {
        let area = region.len();
        let perimeter = get_perimeter(&region, &farm);
        let price = area * perimeter;
        total_price += price;
    }

    return total_price as isize;
}

fn get_perimeter(region: &Vec<Position>, farm: &Vec<String>) -> usize {
    let mut perimeter = 0;

    for &position in region {
        let mut local_perimeter = 4;
        for &direction in &DIRECTIONS {
            if let Some(neighbor) = move_from(position, direction, farm) {
                if region.contains(&neighbor) {
                    local_perimeter -= 1;
                }
            }
        }
        perimeter += local_perimeter;
    }

    perimeter
}

fn farm_as_map(farm: &Vec<String>) -> Vec<(String,Vec<Position>)> {
    let mut regions: Vec<(String,Vec<Position>)> = Vec::new();
    let mut visited_cells: HashSet<Position> = HashSet::new();

    for row in 0..farm.len() {
        for col in 0..farm[row].len() {
            let position: Position = Position(col, row );
            let char = get_char_at(Position(col,row), farm);
            if visited_cells.contains(&position) {                
                continue;
            }
            else {
                let region = flood_fill(position, farm, & mut visited_cells);
                regions.push((char,region));
            }            
        }
    }

    regions
}

fn flood_fill(start: Position, farm: &Vec<String>, visited_cells: &mut HashSet<Position>) -> Vec<Position> {
    let mut stack: Vec<Position> = vec![start];
    let mut region: Vec<Position> = Vec::new();

    let char = get_char_at(start, farm).chars().next().unwrap();

    while let Some(position) = stack.pop() {
        if !visited_cells.insert(position) {
            continue;
        }

        region.push(position);

        for &direction in &DIRECTIONS {
            if let Some(neighbor) = move_from(position, direction, farm) {
                if !visited_cells.contains(&neighbor) {
                    let neighbor_char = get_char_at(neighbor, farm).chars().next().unwrap();
                    if neighbor_char == char {
                        stack.push(neighbor); // Add valid neighbor to the stack
                    }
                }
            }
        }
    }

    region

}