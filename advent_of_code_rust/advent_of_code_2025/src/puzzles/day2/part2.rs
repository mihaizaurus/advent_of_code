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
    let mut check = false;
    let id_string = id.to_string();
    for num_chunks in 2..=id_string.len() {
        if id_string.len() % num_chunks == 0 {
            let chunk_size = id_string.len() / num_chunks;
            let parts: Vec<&str> = id_string
                .as_bytes()
                .chunks_exact(chunk_size)
                .map(|chunk| std::str::from_utf8(chunk).unwrap())
                .collect();
            
            if parts.iter().all(|p| p == &parts[0]) {    
                return true
            };
        }
        else {
            check = false
        }
    }
    check
}