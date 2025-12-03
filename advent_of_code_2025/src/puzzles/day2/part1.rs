pub fn result(id_ranges: &Vec<(u128,u128)>) -> isize {
    
    let mut sum_invalid = 0;
    
    for &range in id_ranges {
        let invalid_ids = list_invalid_ids(range);
        for invalid_id in invalid_ids {
            sum_invalid += invalid_id
        }
    }
    
    return sum_invalid as isize;
}

fn list_invalid_ids((start,end):(u128,u128)) -> Vec<u128> {
    let mut invalid_ids = Vec::new();

    for id in start..=end {
        if is_id_invalid(id) {invalid_ids.push(id);}
    }

    invalid_ids
}

fn is_id_invalid(id: u128) -> bool {
    let id_string = id.to_string();
    if id_string.len() % 2 == 0 {
        let mid = id_string.len() / 2 ;

        let (left,right) = id_string.split_at(mid);
        left == right
    }
    else {
        false
    }
}