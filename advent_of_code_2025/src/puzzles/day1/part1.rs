use advent_of_code_common::types::SimpleGrid;

pub fn result(map: &SimpleGrid) -> isize {
    
    let combinations = parse_combinations(map);
    let mut position = 50;
    let mut password = 0;
    for combination in combinations {
        apply_combination(&mut position, combination);
        if position == 0 {
            password += 1;
        }
    }
    
    return password as isize;
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
        temp_pos -= count
    } else if direction == "R" {
        temp_pos +=count
    } else {
        // no idea
    }
    while temp_pos < 0 {
        temp_pos += 100
    }
    while temp_pos > 99 {
        temp_pos -= 100
    }
    *position = temp_pos;
}
// chatgpt would have suggested someting called .wrapping_sub an .wrapping_add on the position.