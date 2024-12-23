use crate::common::types::{Position,SimpleGrid};
use crate::common::utils::{get_char_at, get_neighbors};
use std::collections::HashSet;

pub fn result(map: &SimpleGrid) -> isize {
    
    let trailheads = get_trailheads(map);
    let mut score = 0;
    for trailhead in trailheads {
        score += get_score(trailhead, &map);
    }
    
    
    
    return score as isize;
}

fn get_trailheads(map: &SimpleGrid) -> Vec<Position> {
    let mut trailheads: Vec<Position> = Vec::new();

    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if get_char_at((col, row), map) == "0" {
                trailheads.push((col as isize, row as isize));
            }
        }
    }

    return trailheads;
}

fn get_score(trailhead: Position, map: &SimpleGrid) -> usize {
    let mut trails: Vec<HashSet<Position>> = Vec::new();
    let mut start_hash = HashSet::new();
    start_hash.insert(trailhead);

    trails.push(start_hash);

    for i in 0..9 {
        if i >= trails.len() {

            break;
        }
        let positions = trails[i].clone();
        let mut next_positions = HashSet::new();

        for position in positions {
            let my_next_positions = get_neighbors(position, (i+1).to_string(), map);

            if my_next_positions.len() <= 0 {
                continue;
            }

            next_positions.extend(my_next_positions);
        }

        trails.push(next_positions);
    }
    
    if trails.len() == 10 {
        return trails[9].len();
    }
    else {
        return 0;
    }
    
}

