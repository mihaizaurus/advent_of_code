pub fn result(battery_banks: &Vec<String>) -> isize {
    let mut sum_jolatges = 0;
    
    for battery_bank in battery_banks {
        sum_jolatges += get_largest_joltage(battery_bank);
    }

    sum_jolatges as isize
}

fn get_largest_joltage(battery_bank: &str) -> u16 {
    let mut largest_joltage = 0;
    // check numbers decreasing
    for i in (1..=9).rev() {
        // get all number indexes which can have neighbors to the right (smaller than len)
        let indices: Vec<usize> = battery_bank.match_indices(i.to_string().as_str()).map(|(index, _)| {return index}).filter(|index| *index != battery_bank.len()-1).collect();
        for index_a in indices {
            // get the next highest number to the right of this one
            for j in (1..=9).rev() {
                let sub_indices: Vec<usize> = battery_bank.match_indices(j.to_string().as_str()).map(|(index, _)| {return index}).filter(|index| *index != battery_bank.len() && *index > index_a).collect();
                for _ in sub_indices {
                    let joltage = joltage_concat(i, j);
                    largest_joltage = largest_joltage.max(joltage);
                }
            }
        }
    }
    largest_joltage
}

fn joltage_concat(a: u16, b: u16) -> u16 {
    let digits = 10u16.pow(b.ilog10()+1);
    a * digits + b
}