pub fn result(initial_stones: Vec<u128>) -> isize {
    let mut stones = initial_stones.clone();

    println!("stones: {:?}",stones);

    for _ in 0..25 {
        stones = blink(stones);
        println!("stones: {:?}",stones);
    }

    // return 55312
    return stones.len() as isize;
}

fn blink(initial_stones: Vec<u128>) -> Vec<u128> {
    let mut stones = initial_stones.clone();
    let mut i = 0;

    while i < stones.len() {
        if stones[i] == 0 {
            stones[i] = 1;
        } else if stones[i].to_string().len() % 2 == 0 {
            let split_stones = split_stone(stones[i]);
            stones.splice(i..=i, split_stones);         
            i += 1;
        } else {
            stones[i] *= 2024;
        }
        i += 1;
    }

    return stones
}

fn split_stone(stone: u128) -> Vec<u128> {
    let stone_str = stone.to_string();
    let (left, right) = stone_str.split_at(stone.to_string().len() / 2);
    vec![left.parse::<u128>().unwrap(), right.parse::<u128>().unwrap()]
}