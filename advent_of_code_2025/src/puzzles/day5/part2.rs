pub fn result(ranges: &Vec<(usize,usize)>) -> isize {
    let mut fresh_ids_ranges: Vec<(usize,usize)> = compress_ranges(ranges);

    count_ids(&fresh_ids_ranges) as isize
}

fn compress_ranges(ranges: &Vec<(usize,usize)>) -> Vec<(usize,usize)> {
    let mut fresh_ids_ranges: Vec<(usize,usize)> = Vec::new();

    for &(min, max) in ranges {
        if fresh_ids_ranges.is_empty() { // Initial Push
            fresh_ids_ranges.push((min, max));
        }
        else { // for each subsequent range
            // check whether min and / or max are within an existing range
            for &existing_range in &fresh_ids_ranges {
                let (existing_min, existing_max) = existing_range;
                if existing_min <= min && min <= existing_max { // min is in range
                    if existing_min <= max && max <= existing_max { 
                        break; // max is also in range, so range is fully contained
                    }
                }
            }
        }
    }

    fresh_ids_ranges
}

fn count_ids(ranges: &Vec<(usize,usize)>) -> usize {
    let mut count = 0;
    for &(min, max) in ranges {
        count += max - min;
    }
    count
}