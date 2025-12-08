pub fn result(battery_banks: &Vec<String>) -> isize {
    let mut sum_jolatges = 0;
    
    for battery_bank in battery_banks {
        let mut vec_joltages: Vec<(usize,u16)> = Vec::new();
        sum_jolatges += get_largest_joltage(battery_bank, &mut vec_joltages, 12);
    }
    
    sum_jolatges as isize
}

fn get_largest_joltage(battery_bank: &str, max_joltage: &mut Vec<(usize,u16)>, size: usize) -> u128 {
    while max_joltage.len() < size {
        build_largest_joltage(battery_bank, max_joltage, size);
    }
    calculate_joltage(max_joltage)
}

fn build_largest_joltage(battery_bank: &str, max_joltage: &mut Vec<(usize,u16)>, size: usize) {
    let last_index = match max_joltage.len() {
        0 => 0usize,
        _ => { 
            let (index,_) = max_joltage.get(max_joltage.len()-1).unwrap();
            *index
         }
    };
    if max_joltage.len() == 0 { // first time
        for i in (1..=9).rev() {
            let indices: Vec<usize> = battery_bank
            .match_indices(&i.to_string())
            .map(|(index,_)| index)
            .filter(|&index| index <= battery_bank.len()-size)
            .collect();
            if !indices.is_empty() {
                max_joltage.push((indices[0],i));
                break
            }
        }
    } else {
        for i in (1..=9).rev() {
            let indices: Vec<usize> = battery_bank
                .match_indices(&i.to_string())
                .map(|(index,_)| index)
                .filter(|&index| last_index < index && index <= battery_bank.len()-size+max_joltage.len())
                .collect();
            if !indices.is_empty() {
                max_joltage.push((indices[0],i));
                break
            }
        }
    }
}

fn calculate_joltage(max_joltage: &mut Vec<(usize,u16)>) -> u128 {
    let mut joltage_value = 0;
    for (_,joltage) in max_joltage {
        joltage_value = joltage_concat(joltage_value, *joltage as u128);
    }
    joltage_value
}

fn joltage_concat(a: u128, b: u128) -> u128 {
    let digits = 10u128.pow(b.ilog10()+1);
    a * digits + b
}