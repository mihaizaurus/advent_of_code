use crate::common::{types::{Position, Direction, Edge, SimpleGrid, DIRECTIONS}, utils::{get_char_at, move_from}};
use std::collections::{HashMap,HashSet};

pub fn result(farm: &SimpleGrid) -> isize {
    // get plots as contiguous range of same char value (plant type)
    // get perimeter and area of each

    let regions = farm_as_map(farm);
    let mut total_price = 0;
    
    for (key,region) in regions {
        let area = region.len();
        let edges = get_edges(&region, &farm);
        let sides = consolidate_sides(edges);
        let price = area * sides;
        println!("region: [{key}] has area {area}, {sides} sides and price {price}");
        total_price += price;
    }

    return total_price as isize;
}

fn get_edges(region: &Vec<Position>, farm: &Vec<String>) -> HashSet<Edge> {
    let mut edges: HashSet<Edge> = HashSet::new();

    for &position in region {
        for &direction in &DIRECTIONS {
            if let Some(neighbor) = move_from(position, direction, farm) {
                if !region.contains(&neighbor) { // Neighbor not in region
                    edges.insert((position,direction,Some(neighbor)));
                }
            }
            else if let None = move_from(position, direction, farm) {
                edges.insert((position,direction,None));
            }
        }
    }
    edges
}

fn consolidate_sides(edges: HashSet<Edge>) -> usize {
    let mut grouped_edges: HashMap<(Direction, usize), Vec<Position>> = HashMap::new();

    // Group edges by their direction and shared axis
    for edge in edges {
        let (position, direction, _) = edge;

        // Group edges by direction and shared coordinate axis
        if direction.0 == 0 {
            // Horizontal edge, group by Y-axis
            grouped_edges.entry((direction, position.1)).or_default().push(position);
        } else {
            // Vertical edge, group by X-axis
            grouped_edges.entry((direction, position.0)).or_default().push(position);
        }
    }

    let mut consolidated_count = 0;

    // Consolidate edges in each group
    for (_key, mut group) in grouped_edges {
        // Sort positions within the group
        group.sort();

        // Merge contiguous positions
        let mut previous = group[0];
        for &position in &group[1..] {
            if position.0 != previous.0 + 1 && position.1 != previous.1 + 1 {
                // Break in continuity: finalize the current segment
                consolidated_count += 1;
            }
            previous = position;
        }

        // Count the final segment
        consolidated_count += 1;
    }

    consolidated_count
}

fn farm_as_map(farm: &Vec<String>) -> Vec<(String,Vec<Position>)> {
    let mut regions: Vec<(String,Vec<Position>)> = Vec::new();
    let mut visited_cells: HashSet<Position> = HashSet::new();

    for row in 0..farm.len() {
        for col in 0..farm[row].len() {
            let position: Position = (col, row );
            let char = get_char_at((col,row), farm);
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