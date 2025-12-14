use advent_of_code_common::types::SimpleGrid;

pub fn result(map: &SimpleGrid) -> isize {
    
    let combinations = parse_combinations(map);
    let mut position = 50;
    let mut crossings = 0;
    count_total_clicks(&mut crossings, &mut position, combinations);

    return crossings as isize;
}

fn parse_combinations(map: &SimpleGrid) -> Vec<(char,u16)> {
    let mut combinations: Vec<(char, u16)> = Vec::new();
    for line in map {
        if line.is_empty() {continue;} // not really useful here but jic.
        let (dir, count_chars) = line.split_at(1);

        let direction = dir.chars().nth(0).unwrap();
        let count = count_chars.parse::<u16>().unwrap();
        
        combinations.push((direction,count));
        
    }
    combinations
}

fn apply_combination(position: &mut u16, combination: (char, u16)) {
    let (direction, count) = combination;
    // let steps = (count.rem_euclid(100)) as u16; // another way to write count % 100...
    let steps = count % 100;
    
    let temp = match direction {
        'L' => (*position + 100 - steps) % 100,
        'R' => (*position + steps) % 100,
        _ => *position
    };

    *position = temp
}

fn count_total_clicks(crossings: &mut u16, position: &mut u16, combinations: Vec<(char, u16)>) -> u16 {
    for combination in combinations {
        let (direction, count) = &combination;
        let steps = *count;
        // dbg!(&steps,&count,&direction,&position);
        if *position > 0 {
            match direction {
                'L' => {
                    if steps >= *position {
                        let remaining = (steps - *position) as u16;
                        *crossings += 1 + remaining / 100
                    }
                },
                'R' => {
                    let dist = 100 - *position;
                    if steps >= dist {
                        let remaining = (steps - dist) as u16;
                        *crossings += 1 + remaining / 100
                    }
                },
                _ => {}
            }
        } else {
            *crossings += steps as u16 / 100
        }

        apply_combination(position, combination);
    }

    *crossings
}