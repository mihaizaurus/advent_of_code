use std::collections::HashMap;

pub fn result(initial_stones: &Vec<u128>) -> usize {
    let mut stone_counts: HashMap<u128, usize> = HashMap::new();

    // Initialize counts from the initial stones
    for &stone in initial_stones {
        *stone_counts.entry(stone).or_insert(0) += 1;
    }

    for _ in 0..75 {
        stone_counts = blink(stone_counts);
    }

    // Calculate the total number of stones
    stone_counts.values().sum()
}

fn blink(stone_counts: HashMap<u128, usize>) -> HashMap<u128, usize> {
    let mut new_counts: HashMap<u128, usize> = HashMap::new();

    for (stone, count) in stone_counts {
        if stone == 0 {
            *new_counts.entry(1).or_insert(0) += count;
        } else if stone.to_string().len() % 2 == 0 {
            let split_stones = split_stone(stone);
            for split in split_stones {
                *new_counts.entry(split).or_insert(0) += count;
            }
        } else {
            let new_stone = stone.checked_mul(2024).expect("Overflow detected");
            *new_counts.entry(new_stone).or_insert(0) += count;
        }
    }

    new_counts
}

fn split_stone(stone: u128) -> Vec<u128> {
    let stone_str = stone.to_string();
    let (left, right) = stone_str.split_at(stone.to_string().len() / 2);
    vec![left.parse::<u128>().unwrap(), right.parse::<u128>().unwrap()]
}