use advent_of_code_common::types::{Direction, SimpleGrid};

pub fn result(map: &SimpleGrid) -> isize {
    
    let combinations = parse_combinations(map);
    let mut position = 50;
    let clicks = count_total_clicks(&mut position, combinations);
    
    return clicks as isize;
}

fn parse_combinations(map: &SimpleGrid) -> Vec<(String,i16)> {
    let mut combinations: Vec<(String, i16)> = Vec::new();
    for item in map {
        if let Some((direction, count_chars)) = item.chars().collect::<Vec<char>>().split_first() {
            let count_string:String = count_chars.iter().collect();
            let count = count_string.parse::<i16>().unwrap();
            combinations.push((direction.to_string(),count));
        }
    }
    combinations
}

fn apply_combination(position: &mut i16, combination: (String, i16)) {
    let mut temp_pos = *position;
    let (direction, count) = combination;
    if direction == "L" {
        temp_pos -= count;
    } else if direction == "R" {
        temp_pos +=count
    } else {
        // no idea
    }
    while temp_pos < 0 {
        temp_pos += 100;
    }
    while temp_pos > 99 {
        temp_pos -= 100;
    }
    *position = temp_pos;
}

fn count_total_clicks(position: &mut i16, combinations: Vec<(String, i16)>) -> i16 {
    let mut clicks = 0;
    for (direction, count) in combinations {
        // Let's redo this entire thing, I hate it
        let mut temp = match direction.as_str() {
            "L" => {-count},
            "R" => {count},
            _ => {0}
        };
        while temp.abs() > 100 {
            clicks += 1;
            if direction == "L" {
                temp += 100
            }
            else if direction == "R" {
                temp -= 100
            }
        }
        // Let's redo up until here
    }

    clicks
}